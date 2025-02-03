# News Sentiment Agent

## About
This AI agent framework trades crypto tokens by analyzing news sentiment from a variety of news sources. The base agent currently trades BTC using Jupiterswap, and analyzes news from Bloomberg.

The framework is designed to be modular, and can be extended to trade any token, or use any news source.

## Initializing the terminal
The agent will gather articles from the news sources and create a sentiment for BTC. Fill out the `example.env` file with the necessary information, and change the file name to `.env`.

# Technical Details

## Agent

## AI
The base agent uses OpenAI and DeepSeek to analyze the sentiment of the news articles. The sentiment is then used to trade the token. Each article is analyzed and given a sentiment and confidence score. Using a total current score gathered from multiple news sources, the agent will determine if the sentiment is bullish or bearish.

## Database
The database is a postgres database, designed to cache the latest data from the news sources. There are two tables defined in the database: 

- `articles` - Contains the latest articles from the news sources.
- `agent` - Stores the current state of the agent, including the current sentiment of the market.

## Feeds
Implement the news feed trait to add a new news source, then add it as a feed to the agent.

## Trading
The agent trades using [Jupiterswap](https://jup.ag), and uses the [Jupiter SDK](https://github.com/jupiterswap/jupiter-sdk) to interact with the JupiterSwap API.