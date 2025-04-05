use std::fmt::Debug;
use std::sync::Arc;

/// Port interface for logging operations in the hexagonal architecture.
///
/// This trait defines a standardized logging interface that decouples the core business
/// logic from specific logging implementations. It supports common log levels and
/// thread-safe logging operations through the `Sync + Send` traits.
///
/// # Design Philosophy
/// - Follows the ports and adapters pattern to maintain separation of concerns
/// - Enables easy swapping of logging implementations without affecting business logic
/// - Ensures thread safety for concurrent logging operations
/// - Supports standard log levels (info, warn, error, debug, trace)
///
/// # Implementation Requirements
/// Implementors must:
/// - Be thread-safe (`Sync + Send`)
/// - Support Debug formatting
/// - Handle concurrent logging operations safely
/// - Implement all log level methods
///
/// # Example Implementation
/// ```rust
/// #[derive(Debug)]
/// struct ConsoleLogger;
///
/// impl LoggerPort for ConsoleLogger {
///     fn log_info(&self, message: &str) {
///         println!("[INFO] {}", message);
///     }
///     // ... implement other methods
/// }
/// ```
pub trait LoggerPort: Sync + Send + Debug {
    /// Logs a message at INFO level.
    ///
    /// Used for general information about program execution that would be useful in
    /// understanding the application's normal operation.
    ///
    /// # Arguments
    /// * `message` - The message to log
    fn log_info(&self, message: &str);

    /// Logs a message at WARN level.
    ///
    /// Used for potentially harmful situations or unexpected states that the application
    /// can recover from.
    ///
    /// # Arguments
    /// * `message` - The warning message to log
    fn log_warn(&self, message: &str);

    /// Logs a message at ERROR level.
    ///
    /// Used for error conditions that might still allow the application to continue running,
    /// but require attention.
    ///
    /// # Arguments
    /// * `message` - The error message to log
    fn log_error(&self, message: &str);

    /// Logs a message at DEBUG level.
    ///
    /// Used for detailed information about program execution, typically only enabled
    /// during debugging.
    ///
    /// # Arguments
    /// * `message` - The debug message to log
    fn log_debug(&self, message: &str);

    /// Logs a message at TRACE level.
    ///
    /// Used for very detailed debugging information, typically including program flow
    /// and data values.
    ///
    /// # Arguments
    /// * `message` - The trace message to log
    fn log_trace(&self, message: &str);
}

/// Implementation of LoggerPort for Arc-wrapped LoggerPort implementations.
///
/// This implementation allows for shared ownership of a logger across multiple
/// threads while maintaining thread safety. It delegates all logging operations
/// to the underlying logger implementation.
///
/// # Thread Safety
/// The Arc wrapper ensures that the logger can be safely shared between threads
/// while maintaining proper reference counting.
///
/// # Example Usage
/// ```rust
/// let logger = Arc::new(ConsoleLogger::new());
/// logger.log_info("Application started");
/// ```
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

