use chrono::{DateTime, Local};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    text::{Line, Span},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const MAX_STORIES: usize = 6;
const MAX_TRADES: usize = 5;

#[derive(Clone)]
pub struct Story {
    pub content: String,
    pub sentiment: f64,
    pub score: f64,
    pub timestamp: DateTime<Local>,
}

#[derive(Clone)]
pub struct Trade {
    pub amount: f64,
    pub direction: TradeDirection,
    pub timestamp: DateTime<Local>,
}

#[derive(Clone)]
pub enum TradeDirection {
    Buy,
    Sell,
}

pub struct AppState {
    stories: Vec<Story>,
    trades: Vec<Trade>,
    total_balance: f64,
    total_sentiment: f64,
    total_score: f64,
    // Track if sentiment/score should be calculated from stories or use manual values
    manual_sentiment: bool,
    manual_score: bool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            stories: Vec::new(),
            trades: Vec::new(),
            total_balance: 0.0,
            total_sentiment: 0.0,
            total_score: 0.0,
            manual_sentiment: false,
            manual_score: false,
        }
    }

    pub fn add_story(&mut self, story: Story) {
        self.stories.push(story);
        if self.stories.len() > MAX_STORIES {
            self.stories.remove(0);
        }
        self.update_totals();
    }

    pub fn add_trade(&mut self, trade: Trade) {
        match trade.direction {
            TradeDirection::Buy => self.total_balance += trade.amount,
            TradeDirection::Sell => self.total_balance -= trade.amount,
        }
        self.trades.push(trade);
        if self.trades.len() > MAX_TRADES {
            self.trades.remove(0);
        }
    }

    pub fn update_totals(&mut self) {
        if !self.stories.is_empty() {
            if !self.manual_sentiment {
                self.total_sentiment = self.stories.iter().map(|s| s.sentiment).sum::<f64>() / self.stories.len() as f64;
            }
            if !self.manual_score {
                self.total_score = self.stories.iter().map(|s| s.score).sum::<f64>() / self.stories.len() as f64;
            }
        }
    }

    pub fn set_sentiment(&mut self, sentiment: f64) {
        self.manual_sentiment = true;
        self.total_sentiment = sentiment;
    }

    pub fn set_score(&mut self, score: f64) {
        self.manual_score = true;
        self.total_score = score;
    }

    pub fn reset_to_story_calculations(&mut self) {
        self.manual_sentiment = false;
        self.manual_score = false;
        self.update_totals();
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.total_balance = balance;
    }
}

// This is the struct that external code will interact with
pub struct Dashboard {
    state: Arc<Mutex<AppState>>,
    running: Arc<Mutex<bool>>,
}

impl Dashboard {
    pub fn update_sentiment(&self, sentiment: f64) {
        if let Ok(mut state) = self.state.lock() {
            state.set_sentiment(sentiment);
        }
    }

    pub fn update_score(&self, score: f64) {
        if let Ok(mut state) = self.state.lock() {
            state.set_score(score);
        }
    }

    pub fn reset_to_story_calculations(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.reset_to_story_calculations();
        }
    }

    pub fn get_state(&self) -> Option<(f64, f64, f64)> {
        if let Ok(state) = self.state.lock() {
            Some((state.total_balance, state.total_sentiment, state.total_score))
        } else {
            None
        }
    }

    pub fn new() -> Dashboard {
        Dashboard {
            state: Arc::new(Mutex::new(AppState::new())),
            running: Arc::new(Mutex::new(true)),
        }
    }

    // Method to add a new story
    pub fn add_story(&self, content: String, sentiment: f64, score: f64) {
        if let Ok(mut state) = self.state.lock() {
            state.add_story(Story {
                content,
                sentiment,
                score,
                timestamp: Local::now(),
            });
        }
    }

    // Method to add a new trade
    pub fn add_trade(&self, amount: f64, direction: TradeDirection) {
        if let Ok(mut state) = self.state.lock() {
            state.add_trade(Trade {
                amount,
                direction,
                timestamp: Local::now(),
            });
        }
    }

    // Method to update the total balance
    pub fn update_balance(&self, balance: f64) {
        if let Ok(mut state) = self.state.lock() {
            state.set_balance(balance);
        }
    }

    // Method to stop the dashboard
    pub fn stop(&self) {
        if let Ok(mut running) = self.running.lock() {
            *running = false;
        }
    }

    // Method to start the dashboard UI
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let state_clone = Arc::clone(&self.state);
        let running_clone = Arc::clone(&self.running);

        // Run the UI loop
        let res = run_app(&mut terminal, state_clone, running_clone);

        // Cleanup
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }

    pub fn clear_data(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.stories.clear();
            state.trades.clear();
            state.total_balance = 0.0;
            state.total_sentiment = 0.0;
            state.total_score = 0.0;
            state.manual_sentiment = false;
            state.manual_score = false;
        }
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: Arc<Mutex<AppState>>,
    running: Arc<Mutex<bool>>,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        // Check if we should continue running
        if let Ok(is_running) = running.lock() {
            if !*is_running {
                break;
            }
        }

        // Get current state for rendering
        if let Ok(state) = state.lock() {
            terminal.draw(|f| ui(f, &state))?;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to acquire state lock"));
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    if let Ok(mut running) = running.lock() {
                        *running = false;
                    }
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(chunks[0]);

    // News section
    let news_block = Block::default()
        .title("News")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    
    let stories: Vec<ListItem> = state
        .stories
        .iter()
        .map(|story| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("[{}] ", story.timestamp.format("%H:%M:%S")),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("{:<40}", story.content),
                    Style::default().fg(Color::Green),
                ),
                Span::styled(
                    format!("{:>8.2}", story.sentiment),
                    Style::default().fg(if story.sentiment >= 0.0 {
                        Color::LightGreen
                    } else {
                        Color::LightRed
                    }),
                ),
                Span::styled(
                    format!("{:>8.2}", story.score),
                    Style::default().fg(Color::Yellow),
                ),
            ]))
        })
        .collect();

    let stories_list = List::new(stories)
        .block(news_block)
        .style(Style::default().fg(Color::White));

    f.render_widget(stories_list, left_chunks[0]);

    // Bottom text
    let bottom_text = Paragraph::new("Press q to quit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(bottom_text, left_chunks[1]);

    // Right side
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Trade History title
    let trade_title = Paragraph::new("Trade History")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(trade_title, right_chunks[0]);

    // Total Balance
    let balance_text = Paragraph::new(format!("Total Balance: ${:.2}", state.total_balance))
        .style(Style::default().fg(if state.total_balance >= 0.0 {
            Color::LightGreen
        } else {
            Color::LightRed
        }))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(balance_text, right_chunks[1]);

    // Trade transactions
    let trades: Vec<ListItem> = state
        .trades
        .iter()
        .map(|trade| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("[{}] ", trade.timestamp.format("%H:%M:%S")),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    match trade.direction {
                        TradeDirection::Buy => "BUY  ",
                        TradeDirection::Sell => "SELL ",
                    },
                    Style::default().fg(match trade.direction {
                        TradeDirection::Buy => Color::LightGreen,
                        TradeDirection::Sell => Color::LightRed,
                    }),
                ),
                Span::styled(
                    format!("${:.2}", trade.amount),
                    Style::default().fg(Color::White),
                ),
            ]))
        })
        .collect();

    let trades_list = List::new(trades)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(trades_list, right_chunks[2]);

    // BTC Logo and Sentiment
    let logo_block = Block::default()
        .title("BTC")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let sentiment_text = vec![
        Line::from(vec![
            Span::raw("SENTIMENT: "),
            Span::styled(
                format!("{:.2}", state.total_sentiment),
                Style::default().fg(if state.total_sentiment >= 0.0 {
                    Color::LightGreen
                } else {
                    Color::LightRed
                }),
            ),
        ]),
        Line::from(vec![
            Span::raw("SCORE: "),
            Span::styled(
                format!("{:.2}", state.total_score),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let sentiment_paragraph = Paragraph::new(sentiment_text)
        .block(logo_block)
        .style(Style::default().fg(Color::White));

    f.render_widget(sentiment_paragraph, right_chunks[3]);
}