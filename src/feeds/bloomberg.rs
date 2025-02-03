use reqwest;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use super::base::Article;
use super::base::Feed;
use scraper::Html;
use async_trait;

#[derive(Debug, Deserialize)]
struct BloombergArticle {
    pub headline: String,
    pub byline: String,
    pub url: String,
    pub published_at: String,
}

pub struct Bloomberg {
    client: reqwest::Client,
}

impl Bloomberg {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
        }
    }

    async fn get_story(&self, url: String) -> Result<Article, Box<dyn std::error::Error>> {
        let response = self.client.get(&url)
            .header("Host", "www.bloomberg.com")
            .header("Cookie", "exp_pref=AMER; country_code=US")
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", "\"macOS\"")
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .header("sec-ch-ua", "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"")
            .header("accept", "*/*")
            .header("sec-fetch-site", "same-origin")
            .header("sec-fetch-mode", "cors") 
            .header("sec-fetch-dest", "empty")
            .header("referer", "https://www.bloomberg.com/latest")
            .header("accept-language", "en-US,en;q=0.9")
            .header("priority", "u=1, i")
            .send()
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&response);

        let title = self.select_meta_content(&document, "og:title")?;
        let author = self.select_meta_content(&document, "parsely-author")?;
        let published_at = self.select_meta_content(&document, "parsely-pub-date")?;
        let published_at = DateTime::parse_from_rfc3339(&published_at)?.with_timezone(&Utc);
        let description = self.select_meta_content(&document, "og:description")?;

        Ok(Article {
            title,
            author,
            body: description,
            url: url.to_string(),
            source: "Bloomberg".to_string(),
            published_at,
        })
    }

    fn select_meta_content(&self, document: &Html, property: &str) -> Result<String, Box<dyn std::error::Error>> {
        let selector = scraper::Selector::parse(&format!("meta[property='{}'], meta[name='{}']", property, property))
            .map_err(|e| format!("Failed to parse selector: {}", e))?;
        
        let content = document
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .ok_or_else(|| format!("Meta tag with property '{}' not found", property))?
            .to_string();

        Ok(content)
    }
    
    async fn get_stories(&self, page: &str) -> Result<Vec<BloombergArticle>, Box<dyn std::error::Error>> {
        let response = self.client
            .get("https://www.bloomberg.com/lineup-next/api/stories")
            .query(&[
                ("limit", "25"),
                ("brand", "MARKETS"), 
                ("pageNumber", &page),
                ("types", "ARTICLE")
            ])
            .header("Host", "www.bloomberg.com")
            .header("Cookie", "exp_pref=AMER; country_code=US")
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", "\"macOS\"")
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .header("sec-ch-ua", "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"")
            .header("accept", "*/*")
            .header("sec-fetch-site", "same-origin")
            .header("sec-fetch-mode", "cors") 
            .header("sec-fetch-dest", "empty")
            .header("referer", "https://www.bloomberg.com/latest")
            .header("accept-language", "en-US,en;q=0.9")
            .header("priority", "u=1, i")
            .send()
            .await?;

        let bloomberg_articles: Vec<BloombergArticle> = response.json().await?;
        Ok(bloomberg_articles)
    }
}

#[async_trait::async_trait]
impl Feed for Bloomberg {
    async fn get_new_articles(&self) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
        let stories = match self.get_stories("1").await {
            Ok(stories) => stories,
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get stories")))
        };

        let mut articles = Vec::new();
        for story in stories {
            if let Ok(article) = self.get_story(story.url).await {
                articles.push(article);
            }
        }
        
        Ok(articles)
    }
}