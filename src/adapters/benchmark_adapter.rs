//! Benchmark Adapter Module
//!
//! This module provides an adapter for running benchmarks. It implements the BenchmarkPort trait,
//! allowing for the execution of benchmarking commands and returning their output.

use crate::ports::benchmark_port::BenchmarkPort;
use crate::ports::log_port::LoggerPort;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

/// default FIO benchmark parameters
const FIO_DEAULT_ARGS: [&str; 8] = [
    "--name=test",
    "--ioengine=libaio",
    "--rw=randread",
    "--bs=4k",
    "--size=1G",
    "--numjobs=1",
    "--group_reporting",
    "--output-format=json",
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
    benchmark_dir: PathBuf,
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

        let benchmark_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("benchmark");

        Self {
            command,
            args,
            logger,
            benchmark_dir,
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

        let benchmark_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("benchmark");

        // Create base args from the defaults
        let mut args: Vec<String> = FIO_DEAULT_ARGS.iter().map(|&s| s.to_string()).collect();

        // then add filename argument using benchmark directory
        args.push(format!("--filename={}/testfile", benchmark_dir.display()));

        Self {
            command: String::from("fio"),
            args,
            logger,
            benchmark_dir,
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
        // First validate the environment
        self.validate()?;
        
        // Check if FIO is installed
        self.check_fio_installation()?;

        // Create a test file of sufficient size
        let test_file = self.benchmark_dir.join("test.fio");
        self.logger.log_info(&format!("Creating test file at: {}", test_file.display()));

        // Define comprehensive FIO test parameters
        let test_args = vec![
            format!("--filename={}", test_file.display()),
            "--direct=1".to_string(),        // Use direct I/O
            "--rw=randrw".to_string(),       // Mixed random read/write
            "--bs=4k".to_string(),           // 4KB block size
            "--size=1G".to_string(),         // 1GB file size
            "--numjobs=4".to_string(),       // Use 4 parallel jobs
            "--runtime=30".to_string(),      // Run for 30 seconds
            "--group_reporting".to_string(), 
            "--name=fio_test".to_string(),
            "--output-format=json".to_string(),
        ];

        // Log the actual command being executed
        self.logger.log_info(&format!(
            "Executing FIO benchmark: fio {}",
            test_args.join(" ")
        ));

        // Execute FIO command
        let output = Command::new("fio")
            .args(&test_args)
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to execute FIO: {}", e))?;

        // Process command output
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            self.logger.log_info("\nBenchmark completed successfully");
            self.logger.log_info(&format!("\nResults:\n{}", output_str));

            self.logger.log_info(&format!("Test file retained at: {}", test_file.display()));
            
            
            Ok(())
        } else {
            let error_msg = self.format_output(&output.stderr, true);
            self.logger.log_error(&error_msg);
            Err(anyhow::anyhow!(error_msg))
        }
    }

    fn check_fio_installation(&self) -> Result<()> {
        self.logger.log_debug("Checking FIO installation");

        match Command::new("fio").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    self.logger.log_info(&format!("FIO version: {}", version));
                    Ok(())
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    Err(anyhow::anyhow!("FIO check failed: {}", error))
                }
            }
            Err(e) => {
                let error_msg = format!("FIO not found. Please install FIO: {}", e);
                self.logger.log_error(&error_msg);
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }


    fn validate(&self) -> Result<()> {
        self.logger.log_debug("Validating benchmark directory");

        // Get project root directory (where Cargo.toml is located)
        let project_root = std::env::current_dir().map_err(|e| {
            let error_msg = format!("Could not determine current directory: {}", e);
            self.logger.log_error(&error_msg);
            anyhow::anyhow!(error_msg)
        })?;

        // Create benchmark directory path within project root
        let benchmark_dir = project_root.join("benchmark");

        // Create directory if it doesn't exist
        if !benchmark_dir.exists() {
            self.logger
                .log_info("Benchmark directory does not exist, creating it");
            std::fs::create_dir_all(&benchmark_dir).map_err(|e| {
                let error_msg = format!(
                    "Failed to create benchmark directory {}: {}",
                    benchmark_dir.display(),
                    e
                );
                self.logger.log_error(&error_msg);
                anyhow::anyhow!(error_msg)
            })?;

            self.logger.log_info(&format!(
                "Successfully created benchmark directory: {}",
                benchmark_dir.display()
            ));
        }

        self.logger
            .log_info("Benchmark directory validation successful");
        Ok(())
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
