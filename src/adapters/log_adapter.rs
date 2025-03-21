use chrono::Local;
use log::{Level, LevelFilter};

use std::fmt;
use std::fs;
use std::fs::File;

use colored::*;
use fern::{log_file, Dispatch};

use crate::ports::log_port::LoggerPort;

/// `FernLogger` is a struct that implements the `LoggerPort` trait.
/// It is thread-safe due to the implementation of `Sync` and `Send` traits.
pub struct FernLogger;

// Implement the Sync trait for the FernLogger struct. This allows the struct to be sent between threads safely.
unsafe impl Sync for FernLogger {}

// Implement the Send trait for the FernLogger struct.
impl FernLogger {
    /// Creates a new instance of `FernLogger`.
    ///
    /// # Returns
    ///
    /// * `FernLogger` - The new `FernLogger` instance.
    pub fn new() -> Self {
        FernLogger
    }
}

// Implement the Debug trait for FernLogger.
impl fmt::Debug for FernLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FernLogger")
    }
}

// Implement the LoggerPort trait for the FernLogger struct.
impl LoggerPort for FernLogger {
    /// Logs an informational message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_info(&self, message: &str) {
        log::info!("{}", message);
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_warn(&self, message: &str) {
        log::warn!("{}", message);
    }

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_error(&self, message: &str) {
        log::error!("{}", message);
    }

    /// Logs a debug message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_debug(&self, message: &str) {
        log::debug!("{}", message);
    }

    /// Logs a trace message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_trace(&self, message: &str) {
        log::trace!("{}", message);
    }
}

/// Initializes the logging system.
///
/// # Arguments
///
/// * `log_dir_path` - The path to the directory where the log files will be stored.
/// * `level_filter` - The minimum severity level of log messages that should be logged.
///
/// # Returns
///
/// * `FernLogger` - The initialized `FernLogger` instance.
pub fn init(
    log_dir_path: &str,
    level_filter: LevelFilter,
) -> Result<FernLogger, Box<dyn std::error::Error>> {
    // Ensure the log directory exists, creating it if necessary.
    fs::create_dir_all(log_dir_path)
        .map_err(|e| format!("Failed to create log directory '{}': {}", log_dir_path, e))?;

    // Create or truncate log files for each log level.
    let log_levels = ["error", "warn", "info", "debug", "trace"];
    for level in &log_levels {
        let file_path = format!("{}/one_4_all_{}.log", log_dir_path, level);
        File::create(&file_path).map_err(|e| {
            format!(
                "Failed to create or truncate log file '{}': {}",
                file_path, e
            )
        })?;
    }

    // Set up the base configuration for the logger, including message formatting.
    let base_config = Dispatch::new()
        .format(move |out, message, record| {
            // Colorize messages based on their log level.
            let color_message = match record.level() {
                Level::Error => message.to_string().red(),
                Level::Warn => message.to_string().yellow(),
                Level::Info => message.to_string().green(),
                Level::Debug => message.to_string().blue(),
                Level::Trace => message.to_string().cyan(),
            };
            // Format the log message with a timestamp, level, and the colorized message.
            out.finish(format_args!(
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                color_message
            ))
        })
        .level(level_filter);

    // Set up individual log files for each log level.
    let error_log = log_file(&format!("{}/error.log", log_dir_path))
        .map_err(|e| format!("Failed to open error log file: {}", e))?;
    let warn_log = log_file(&format!("{}/warn.log", log_dir_path))
        .map_err(|e| format!("Failed to open warn log file: {}", e))?;
    let info_log = log_file(&format!("{}/info.log", log_dir_path))
        .map_err(|e| format!("Failed to open info log file: {}", e))?;
    let debug_log = log_file(&format!("{}/debug.log", log_dir_path))
        .map_err(|e| format!("Failed to open debug log file: {}", e))?;
    let trace_log = log_file(&format!("{}/trace.log", log_dir_path))
        .map_err(|e| format!("Failed to open trace log file: {}", e))?;

    // Create dispatch configurations for each log level, filtering and chaining to the respective log file.
    let error_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == Level::Error)
        .chain(error_log);

    let warn_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == Level::Warn)
        .chain(warn_log);

    let info_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == Level::Info)
        .chain(info_log);

    let debug_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == Level::Debug)
        .chain(debug_log);

    let trace_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == Level::Trace)
        .chain(trace_log);

    // Combine all dispatch configurations into one.
    let combined_config = base_config
        .chain(error_dispatch)
        .chain(warn_dispatch)
        .chain(info_dispatch)
        .chain(debug_dispatch)
        .chain(trace_dispatch)
        .chain(std::io::stdout()); // Also log to standard output.

    // Apply the combined logger configuration.
    combined_config
        .apply()
        .map_err(|e| format!("Failed to initialize logger: {}", e))?;

    // Return the logger instance for use by other modules in the application.
    Ok(FernLogger)
}
