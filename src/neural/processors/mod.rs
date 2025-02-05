pub mod nlp;
pub mod vision;
pub mod hypothesis;

pub struct Pipeline<B: Backend> {
    pub nlp: nlp::NLPProcessor,
    pub vision: vision::VisionProcessor,
    pub hypothesis: hypothesis::HypothesisGenerator<B>,
}

impl<B: Backend> Default for Pipeline<B> {
    fn default() -> Self {
        Self {
            nlp: nlp::NLPProcessor::new(),
            vision: vision::VisionProcessor::new(),
            hypothesis: hypothesis::HypothesisGenerator::default(),
        }
    }
}
