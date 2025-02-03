use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use crate::feeds::base::Article;

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
    fn get_system_prompt() -> String {
        "
            You are to ONLY respond in XML format. You will not respond any other way. You will close any XML tags created with the closing XML tag. You will not write anything without an XML tag.
            You are an expert news trader at a prestigious hedge fund. I will give you the text of a news story. You will respond with the sentiment (POSITIVE, NEGATIVE, NEUTRAL) and a confidence score between 0 and 1. Here is an example of a response:
            ```
            <Sentiment>
                POSITIVE
            </Sentiment>
            <Confidence>
                0.23
            </Confidence>
            ```

            The response does not include the backticks.
        ".to_string()
    }
    fn get_prompt_for_article(article: &Article) -> String {
        format!("
        <title>{}</title>
        <author>{}</author>
        <published_at>{}</published_at>
        <source>{}</source>
        <content>{}</content>
        ", article.title, article.author, article.published_at, article.source, article.body)
    }
    async fn analyze_sentiment(&self, article: Article) -> Result<SentimentAnalysisResult, Box<dyn std::error::Error>>;
}