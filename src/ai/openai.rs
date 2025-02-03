use openai::{
    chat:: {
        ChatCompletion,
        ChatCompletionMessage,
        ChatCompletionMessageRole
    },
    Credentials,
};
use super::base::{
    AI,
    SentimentAnalysisResult,
    Sentiment
};
use crate::feeds::base::Article;
use async_trait::async_trait;
use roxmltree;

struct OpenAI {
    credentails: Credentials,
}

impl OpenAI {
    pub fn new(api_key: String) -> Self {
        let credentails = Credentials::new(api_key, "https://api.openai.com/");
        Self {
            credentails
        }
    }
}

#[async_trait]
impl AI for OpenAI {
    async fn analyze_sentiment(&self, article: Article) -> Result<SentimentAnalysisResult, Box<dyn std::error::Error>> {
        let messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(Self::get_system_prompt()),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: vec![],
            },
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(Self::get_prompt_for_article(&article)),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: vec![],
            },
        ];
        let chat_completion = ChatCompletion::builder("gpt-4o", messages.clone())
            .credentials(self.credentails.clone())
            .create()
            .await?;
        let returned_message = chat_completion.choices.first()
            .ok_or_else(|| Box::<dyn std::error::Error>::from("No completion choices returned"))?
            .message.content.clone()
            .ok_or_else(|| Box::<dyn std::error::Error>::from("No message content returned"))?;
        
        let doc = roxmltree::Document::parse(&returned_message)?;
        let sentiment = match doc.descendants().find(|n| n.tag_name().name() == "Sentiment".to_string()).unwrap().text().unwrap().trim() {
            "POSITIVE" => Sentiment::Positive,
            "NEGATIVE" => Sentiment::Negative,
            "NEUTRAL" => Sentiment::Neutral,
            _ => return Err("Invalid sentiment value".into()),
        };
        let confidence: f32 = match doc.descendants().find(|n| n.tag_name().name() == "Confidence".to_string()).unwrap().text().unwrap().trim().parse::<f32>() {
            Ok(number) => number,
            Err(e) => return Err(format!("Invalid value for confidence score: {e}").into())
        };

        Ok(SentimentAnalysisResult {
            sentiment,
            confidence
        })
    }
}