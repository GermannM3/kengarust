use async_trait::async_trait;

pub struct NLPProcessor;

impl NLPProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn parse(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect()
    }
}
