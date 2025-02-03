use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub author: String,
    pub body: String,
    pub url: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
}

#[async_trait]
pub trait Feed {
    async fn get_new_articles(&self) -> Result<Vec<Article>, Box<dyn std::error::Error>>;
}
