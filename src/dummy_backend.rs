use burn::tensor::backend::Backend;

// Заглушка для Backend
pub struct DummyBackend;

impl Backend for DummyBackend {
    // Реализуйте методы интерфейса Backend по необходимости
} 