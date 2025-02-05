use std::collections::VecDeque;

pub struct EpisodicMemory {
    events: VecDeque<String>,
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(1000),
        }
    }

    pub fn store(&mut self, event: String) {
        self.events.push_back(event);
    }

    pub fn recall(&self, pattern: &str) -> Vec<String> {
        self.events.iter()
            .filter(|e| e.contains(pattern))
            .cloned()
            .collect()
    }
}
