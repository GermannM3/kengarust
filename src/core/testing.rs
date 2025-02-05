use crate::neural::cortex::NeuralCortex;

pub struct SelfValidator {
    cortex: Arc<RwLock<NeuralCortex>>,
}

impl SelfValidator {
    pub async fn run_diagnostics(&self) -> ValidationReport {
        let test_cases = self.generate_test_cases().await;
        let mut report = ValidationReport::new();
        
        for case in test_cases {
            let result = self.cortex.write().await.process(&case.input).await;
            report.add_case(case, result);
        }
        
        self.adjust_parameters(report.clone()).await;
        report
    }
}
