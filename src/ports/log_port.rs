use std::fmt::Debug;
use std::sync::Arc;

/// The `LoggerPort` trait acts as a port in the hexagonal architecture.
pub trait LoggerPort: Sync + Send + Debug {
    fn log_info(&self, message: &str);
    fn log_warn(&self, message: &str);
    fn log_error(&self, message: &str);
    fn log_debug(&self, message: &str);
    fn log_trace(&self, message: &str);
}

// Implement LoggerPort for Arc<dyn LoggerPort>
impl LoggerPort for Arc<dyn LoggerPort> {
    fn log_info(&self, message: &str) {
        self.as_ref().log_info(message)
    }

    fn log_warn(&self, message: &str) {
        self.as_ref().log_warn(message)
    }

    fn log_error(&self, message: &str) {
        self.as_ref().log_error(message)
    }

    fn log_debug(&self, message: &str) {
        self.as_ref().log_debug(message)
    }

    fn log_trace(&self, message: &str) {
        self.as_ref().log_trace(message)
    }
}

