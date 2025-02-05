use crate::neural::Cortex;
use std::sync::Arc;

pub struct SelfTester {
    cortex: Arc<Cortex>,
}

#[derive(Debug)]
pub struct HealthReport {
    memory_usage: f32,
    learning_efficiency: f32,
    data_throughput: f32,
}

impl SelfTester {
    pub fn new(cortex: Arc<Cortex>) -> Self {
        Self { cortex }
    }

    pub async fn run_diagnostics(&self) -> HealthReport {
        HealthReport {
            memory_usage: self.cortex.memory_usage().await,
            learning_efficiency: self.cortex.learning_metrics().await,
            data_throughput: self.cortex.data_throughput().await,
        }
    }
}

