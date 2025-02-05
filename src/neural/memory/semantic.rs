use std::collections::HashMap;

pub struct SemanticMemory {
    concepts: HashMap<String, f32>,
}

impl SemanticMemory {
    pub fn new() -> Self {
        Self {
            concepts: HashMap::new(),
        }
    }

    pub fn update(&mut self, concept: String, weight: f32) {
        self.concepts.insert(concept, weight);
    }

    pub fn get_weight(&self, concept: &str) -> f32 {
        *self.concepts.get(concept).unwrap_or(&0.0)
    }
}
