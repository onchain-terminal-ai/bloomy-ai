use openai::{
    chat,
    Credentials
};

struct OpenAI {
    credentails: Credentials,
}

impl OpenAI {
    pub fn new(api_key: String) -> Self {
        let credentails = Credentials::new(api_key, "");
        Self {
            credentails
        }
    }
}