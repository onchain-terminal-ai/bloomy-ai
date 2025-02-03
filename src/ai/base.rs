use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sentiment {
    Positive,
    Negative,
    Neutral
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysisResult {
    pub sentiment: Sentiment,
    pub confidence: f32
}

#[async_trait]
pub trait AI {
    async fn analyze_sentiment(&self, text: &str) -> Result<SentimentAnalysisResult, Box<dyn std::error::Error>>;
}