use tracing::{info, error};

pub fn init_logging() {
    tracing_subscriber::fmt().init();
}

pub fn log_event(event: &str) {
    info!("Event: {}", event);
}
