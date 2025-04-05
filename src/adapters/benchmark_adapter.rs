//! Benchmark Adapter Module
//!
//! This module provides an adapter for running benchmarks. It implements the BenchmarkPort trait,
//! allowing for the execution of benchmarking commands and returning their output.

use crate::ports::benchmark_port::BenchmarkPort;
use crate::ports::log_port::LoggerPort;
use anyhow::Result;
use std::process::Command;
use std::sync::Arc;

/// default FIO benchmark parameters
const FIO_DEAULT_ARGS: [&str; 5] = [
    "--name=test",
    "--ioengine=libaio",
    "--rw=randread",
    "--bs=4k",
    "--size=1G",
];

/// Adapter for executing system bechmarks and performance tests
///
/// # Type Parameters
///
/// * `command` - The benchmark command to execute
/// * `args` - Arguments to pass to the benchmark commands
/// * `logger` - Thread-safe reference to the logging implementation
#[derive(Debug)]
pub struct BenchmarkAdapter {
    command: String,
    args: Vec<String>,
    logger: Arc<dyn LoggerPort>,
}

impl BenchmarkAdapter {
    /// Creates a new BenchmarkAdapter instance
    ///
    /// # Arguments
    ///
    /// * `command` - The benchmark command to execute
    /// * `args` - Vector of command arguments
    /// * `logger` - Thread-safe reference to logger implementation
    ///
    /// # Example
    ///
    /// ```
    /// let logger = Arc::new(MyLogger::new());
    /// let adapter = BenchmarkAdapter::new(
    ///     String::from("fio"),
    ///     vec![String::from("--version")],
    ///     logger
    /// );
    /// ```
    pub fn new(command: String, args: Vec<String>, logger: Arc<dyn LoggerPort>) -> Self {
        logger.log_debug(&format!(
            "Creating new BenchmarkAdapter with command: {}, args: {:?}",
            command, args
        ));

        Self {
            command,
            args,
            logger,
        }
    }

    /// Creates a new BenchmarkAdapter configured for FIO benchmarks
    ///
    /// Initializes with default FIO parameters for basic disk I/O testing
    ///
    /// # Arguments
    ///
    /// * `logger` - Thread-safe reference to logger implementation
    ///
    /// # Example
    ///
    /// ```
    /// let logger = Arc::new(MyLogger::new());
    /// let fio_adapter = BenchmarkAdapter::new_fio(logger);
    /// ```
    pub fn new_fio(logger: Arc<dyn LoggerPort>) -> Self {
        logger.log_debug("Creating new FIO BenchmarkAdapter with default parameters");

        Self {
            command: String::from("fio"),
            args: FIO_DEAULT_ARGS.iter().map(|&s| s.to_string()).collect(),
            logger,
        }
    }
    fn format_output(&self, output: &[u8], is_error: bool) -> String {
        let output_str = String::from_utf8_lossy(output);
        if is_error {
            format!("Error output: {}", output_str)
        } else {
            format!("Command output: {}", output_str)
        }
    }
}

impl BenchmarkPort for BenchmarkAdapter {
    /// Executes the configured benchmark command
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or error with details
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// * Command execution fails
    /// * Command returns non-zero exit status
    fn run(&self) -> Result<()> {
        // Log benchmark execution start
        self.logger.log_debug(&format!(
            "Starting benchmark run with command: {}",
            self.command
        ));
        self.logger
            .log_debug(&format!("Arguments: {:?}", self.args));
        self.logger.log_info(&format!(
            "Executing: {} {}",
            self.command,
            self.args.join(" ")
        ));

        // Execute command and handle result
        let output = match Command::new(&self.command).args(&self.args).output() {
            Ok(output) => output,
            Err(e) => {
                let error_msg = format!("Failed to execute command: {}", e);
                self.logger.log_error(&error_msg);
                return Err(anyhow::anyhow!(error_msg));
            }
        };

        // Process command output
        if output.status.success() {
            self.logger.log_info("\nBenchmark completed successfully");
            self.logger
                .log_debug(&self.format_output(&output.stdout, false));
            Ok(())
        } else {
            let error_msg = self.format_output(&output.stderr, true);
            self.logger.log_error(&error_msg);
            Err(anyhow::anyhow!(error_msg))
        }
    }

    fn validate(&self) -> Result<()> {
        self.logger.log_debug("Validating benchmark directory");

        // Check if benchmark directory exists
        if !std::path::Path::new("/mnt/benchmark").exists() {
            let error_msg = "Benchmark directory does not exist";
            self.logger.log_error(error_msg);
            Err(anyhow::anyhow!(error_msg))
        } else {
            self.logger
                .log_info("Benchmark directory validation successful");
            Ok(())
        }
    }

    fn run_command(&self, command: &str, args: &str) -> Result<String> {
        self.logger
            .log_debug(&format!("Running command: {} with args: {}", command, args));
        match Command::new(command).arg(args).output() {
            Ok(output) => {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                    self.logger
                        .log_info(&format!("Command output: {}", output_str));
                    Ok(output_str)
                } else {
                    let error_msg = self.format_output(&output.stderr, true);
                    self.logger.log_error(&error_msg);
                    Err(anyhow::anyhow!(error_msg))
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to execute command: {}", e);
                self.logger.log_error(&error_msg);
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }

    /// Executes FIO benchmark with version check
    ///
    /// # Returns
    ///
    /// * `Result<String>` - FIO version string or error
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// * FIO command is not available
    /// * Version check fails
    fn run_fio(&self) -> Result<String> {
        self.logger.log_debug("Running FIO version check");

        // Execute FIO version check
        let output = match Command::new("fio").arg("--version").output() {
            Ok(output) => output,
            Err(e) => {
                let error_msg = format!("Failed to execute FIO command: {}", e);
                self.logger.log_error(&error_msg);
                return Err(anyhow::anyhow!(error_msg));
            }
        };

        // Process version check output
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).to_string();
            self.logger
                .log_info(&format!("FIO version check successful: {}", version));
            Ok(version)
        } else {
            let error_msg = self.format_output(&output.stderr, true);
            self.logger.log_error(&error_msg);
            Err(anyhow::anyhow!("Failed to run fio command: {}", error_msg))
        }
    }
}
