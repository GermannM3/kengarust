use reqwest::Client;
use select::document::Document;
use select::predicate::Name;

pub struct WebCrawler {
    client: Client,
    sources: Vec<String>,
}

impl WebCrawler {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            sources: vec![
                "https://en.wikipedia.org/wiki/Special:Random".into(),
                "https://arxiv.org/list/cs.AI/recent".into(),
            ],
        }
    }

    pub async fn fetch_knowledge(&self) -> Vec<String> {
        let mut knowledge = Vec::new();
        
        for url in &self.sources {
            if let Ok(content) = self.client.get(url).send().await {
                if let Ok(text) = content.text().await {
                    knowledge.extend(self.extract_knowledge(&text));
                }
            }
        }
        
        knowledge
    }

    fn extract_knowledge(&self, html: &str) -> Vec<String> {
        Document::from(html)
            .find(Name("p"))
            .filter_map(|n| Some(n.text().trim().to_string()))
            .collect()
    }
}
