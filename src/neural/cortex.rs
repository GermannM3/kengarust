use burn::module::Module;
use burn::tensor::backend::Backend;
use std::sync::Arc;

pub struct Cortex<B: Backend> {
    memory: Arc<memory::HierarchicalMemory>,
    processors: processors::Pipeline<B>,
}

impl<B: Backend> Cortex<B> {
    pub async fn load(path: &str) -> Result<Self, CortexError> {
        let memory = Arc::new(memory::HierarchicalMemory::new());
        let processors = processors::Pipeline::default();
        Ok(Self { memory, processors })
    }

    pub async fn process(&self, input: &str) -> String {
        let parsed = self.processors.nlp.parse(input).await;
        let context = self.memory.recall(&parsed).await;
        self.processors.generate_response(parsed, context).await
    }
}

#[derive(Debug)]
pub enum CortexError {
    LoadFailed,
    ProcessingError,
}
