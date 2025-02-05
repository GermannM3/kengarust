use crate::neural::memory::{Concept, KnowledgeGraph};

pub struct HypothesisEngine {
    knowledge_graph: Arc<RwLock<KnowledgeGraph>>,
}

impl HypothesisEngine {
    pub async fn generate(&self) -> Vec<Hypothesis> {
        let graph = self.knowledge_graph.read().await;
        let concepts = graph.get_related_concepts(5);
        
        concepts.windows(2)
            .filter_map(|pair| self.form_hypothesis(&pair[0], &pair[1]))
            .collect()
    }

    fn form_hypothesis(&self, a: &Concept, b: &Concept) -> Option<Hypothesis> {
        if a.relation_distance(b) < 0.3 {
            Some(Hypothesis {
                premise: format!("Если {} то {}", a, b),
                confidence: 0.0,
                test_cases: Vec::new(),
            })
        } else {
            None
        }
    }
}
