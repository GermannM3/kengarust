use social_forecaster::{predict, AnalysisContext};

pub async fn generate_forecast(cortex: &Cortex) -> String {
    let context = AnalysisContext {
        knowledge_graph: cortex.memory.export_knowledge(),
        temporal_data: cortex.get_temporal_patterns(),
    };
    
    predict(context).await.unwrap_or_else(|_| "Недостаточно данных".into())
}

use crate::neural::Cortex;

pub async fn predict_trends(cortex: &Cortex) -> String {
    // Заглушка для интеграции
    "Прогноз: Рост интереса к AI".into()
}
