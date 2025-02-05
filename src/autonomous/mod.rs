pub mod learning;
pub mod crawler;
pub mod self_test;

pub use learning::AutonomousLearner;
pub use crawler::WebCrawler;
pub use self_test::SelfTester;

pub async fn start_cycle() -> Result<(), Box<dyn std::error::Error>> {
    // Пример: вызов функции переобучения или тестов
    crate::autonomous::learning::perform_learning();
    Ok(())
}

