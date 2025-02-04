# 🌸 Bloomy OS

<div align="center">

![picture of bloomy](./assets/bloomy.jpg)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-black?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)

*An AI-powered crypto trading framework that makes decisions based on real-time news sentiment analysis* 🤖 📈

[Getting Started](#getting-started) •
[Features](#features) •
[Architecture](#architecture) •
[Examples](#examples) •
[Contributing](#contributing)

</div>

# 📚 Table of Contents

- [About](#about)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Examples](#examples)
  - [Pure Crypto News](#pure-crypto-news)
  - [Pure Major News](#pure-major-news)
- [Architecture](#architecture)
  - [Agent Framework](#agent-framework)
  - [AI Integration](#ai-integration)
  - [Database](#database)
  - [News Feeds](#news-feeds)
  - [Trading](#trading)
- [Contributing](#contributing)
- [License](#license)

# 🎯 About

Bloomy OS is a sophisticated AI agent framework that trades cryptocurrencies by analyzing news sentiment from various sources. The base agent trades BTC using Jupiterswap and analyzes news from Bloomberg.

The framework is designed to be highly modular and can be extended to:
- Trade any token 🪙
- Integrate any news source 📰
- Use any AI model for analysis 🧠

# ✨ Features

- 🔌 Highly modular framework
- 🦀 Efficient agent code written in Rust
- 📊 Multiple news sources with configurable feeds
- 🤖 AI-powered sentiment analysis
- 💱 Automated trading on Jupiterswap
- 📈 Real-time market sentiment tracking
- 🗄️ PostgreSQL database for caching and state management

# 🚀 Getting Started

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Solana CLI
- API keys for:
  - OpenAI or DeepSeek
  - Bloomberg (for news feed)
  - Jupiterswap

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/bloomy-os.git
cd bloomy-os
```

2. Copy the environment file and fill in your credentials:
```bash
cp example.env .env
```

3. Build the project:
```bash
cargo build --release
```

# 💡 Examples

## Pure Crypto News 
🔍 **Feeds**: CoinMarketCap, Coingecko, CoinDesk  
🤖 **AI**: OpenAI GPT-4  

**Pros**:
- ✅ Better aligned with crypto news
- ✅ More accurate market predictions
- ✅ Direct correlation with crypto markets

**Cons**:
- ❌ Lags behind traditional news sources

## Pure Major News
🔍 **Feeds**: Bloomberg, Financial Times  

**Pros**:
- ✅ Faster news delivery
- ✅ Broader market context
- ✅ Early indicator of market-moving events

**Cons**:
- ❌ Requires more complex AI interpretation
- ❌ Higher chance of false signals

# 🏗️ Architecture

## Agent Framework
The agent operates through a sophisticated pipeline:

1. 📡 Creates channels for each news source
2. 📊 Evaluates incoming news using AI
3. 🧮 Aggregates sentiment scores
4. 📈 Executes trades based on global evaluation

## AI Integration
The base agent leverages:
- 🤖 OpenAI GPT-4
- 🧠 DeepSeek

Each article receives:
- Sentiment score (-1 to 1)
- Confidence rating (0 to 1)

## Database
PostgreSQL database with two main tables:

- 📰 `articles`: Caches news articles
- 🤖 `agent`: Stores agent state and market sentiment

## News Feeds
Modular feed system:
- 🔌 Implement the news feed trait
- 📰 Bloomberg integration included
- 🔄 Easy to add new sources

## Trading
🔄 Trading execution via [Jupiterswap](https://jup.ag):
- Uses [Jupiter SDK](https://github.com/jupiterswap/jupiter-sdk)
- Optimized for Solana blockchain
- Automated trade execution

# 👥 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

# 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">
Made with ❤️ by the Bloomy OS team
</div>