//! Benchmark Adapter Module
//!
//! This module provides an adapter for running FIO‑based storage benchmarks and wraps all test
//! orchestration behind the `BenchmarkPort` trait.  In addition to the classic OLTP‑style mixes
//! (75 / 25, 50 / 50, etc.) it now documents—**and automatically exercises**—the I/O patterns that
//! dominate modern AI/ML pipelines.
//!
//! # Reference read / write mixes
//!
//! | Workload class & typical block‑size | Canonical R/W mix | FIO settings (`--rw`, `--rwmixread`) |
//! |------------------------------------|-------------------|----------------------------------------|
//! | OLTP / relational DB (8 KiB random) | **75 % R / 25 % W** | `randrw`, `75` |
//! | General virtualised servers (8 KiB random) | **70 / 30** | `randrw`, `70` |
//! | VDI / e‑mail boot & login storm (4 KiB random) | **65 / 35** | `randrw`, `65` |
//! | Mixed enterprise apps “worst case” | **50 / 50** | `randrw`, `50` |
//! | Data‑warehouse scans (64 KiB seq) | **95 / 5** | `rw`, `95` |
//! | Backup / log capture (256 KiB seq) | **0–5 / 95–100** | `rw`, `5` |
//! | **AI – deep‑learning training** (128 KiB–1 MiB seq) | **95 / 5** | `randrw`, `95` |
//! | **AI – checkpoint burst** (1–4 MiB seq) | **10 / 90** | `randrw`, `10` |
//! | **AI – pipeline aggregate** (prep + training) | **48 / 52** | `randrw`, `48` |
//! | **AI – feature‑store ingest** (64 KiB seq) | **20 / 80** | `randrw`, `20` |
//! | **AI – real‑time inference** (4–64 KiB random) | **99 / 1** | `randrw`, `99` |
//!
//! The helper `get_test_configs()` returns a ready‑to‑run vector of these mixes so that every
//! invocation of [`BenchmarkAdapter::run`] iterates through **all** patterns in a single pass.  The
//! generated result files are timestamped and self‑describing (`results_ai_train_95r_5w_20250425…`),
//! making post‑processing trivial.
//!
//! ---
//!
//! ## Adding IO new mixes
//! 1. Append a new row to the table above (keep it alphabetically grouped).
//! 2. Insert a new `TestConfig` entry in `get_test_configs()` with matching `name`, `rw_type`, and
//!    `rwmixread`.
//! 3. That’s it—`BenchmarkAdapter::run` will automatically pick it up.

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

/// Configuration for a single FIO test variant
#[derive(Debug, Clone)]
struct TestConfig {
    rw_type: String,
    rwmixread: Option<u8>,
    name: String,
}

impl BenchmarkAdapter {
    /// Return a vector of canonical I/O patterns to benchmark.
    ///
    /// Each `TestConfig` corresponds to one row of the table in the module‑level docs above.
    fn get_test_configs() -> Vec<TestConfig> {
        vec![
            // ‑‑‑ Traditional mixes ‑‑‑
            //   TestConfig {
            //       rw_type: "randread".into(),
            //       rwmixread: None,
            //       name: "pure_read".into(),
            //   },
            //   TestConfig {
            //       rw_type: "randwrite".into(),
            //       rwmixread: None,
            //       name: "pure_write".into(),
            //   },
            //   TestConfig {
            //       rw_type: "randrw".into(),
            //       rwmixread: Some(75),
            //       name: "mixed_75r_25w".into(),
            //   },
            //   TestConfig {
            //       rw_type: "randrw".into(),
            //       rwmixread: Some(70),
            //       name: "mixed_70r_30w".into(),
            //   },
            //   TestConfig {
            //       rw_type: "randrw".into(),
            //       rwmixread: Some(65),
            //       name: "mixed_65r_35w".into(),
            //   },
            //   TestConfig {
            //       rw_type: "randrw".into(),
            //       rwmixread: Some(50),
            //       name: "mixed_50r_50w".into(),
            //   },
            //   TestConfig {
            //       rw_type: "randrw".into(),
            //       rwmixread: Some(25),
            //       name: "mixed_25r_75w".into(),
            //   },
            // ‑‑‑ AI / ML patterns ‑‑‑
            TestConfig {
                rw_type: "randrw".into(),
                rwmixread: Some(95),
                name: "ai_train_95r_5w".into(),
            },
            TestConfig {
                rw_type: "randrw".into(),
                rwmixread: Some(10),
                name: "ai_checkpoint_10r_90w".into(),
            },
            TestConfig {
                rw_type: "randrw".into(),
                rwmixread: Some(48),
                name: "ai_pipeline_48r_52w".into(),
            },
            TestConfig {
                rw_type: "randrw".into(),
                rwmixread: Some(20),
                name: "ai_feature_ingest_20r_80w".into(),
            },
            TestConfig {
                rw_type: "randrw".into(),
                rwmixread: Some(99),
                name: "ai_inference_99r_1w".into(),
            },
        ]
    }

    /// Run a **single** workload variant and persist its results.
    ///
    /// * `config` – The [`TestConfig`] describing which `--rw` and `--rwmixread` to apply.
    ///
    /// The method builds a dedicated data‑file and JSON result name that embeds both the config
    /// name and a timestamp.  That keeps parallel test runs from stepping on each other and makes
    /// it trivial to correlate `.dat` scratch files with their matching `.json` metrics later on.
    fn run_benchmark_type(&self, config: &TestConfig) -> Result<()> {
        // file names (unique per run)
        let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let test_file = self
            .benchmark_dir
            .join(format!("fio_{}_{}.dat", config.name, ts));
        let results_file = self
            .benchmark_dir
            .join(format!("results_{}_{}.json", config.name, ts));

        // tuned to saturate NVMe
        let mut args = vec![
            format!("--filename={}", test_file.display()), // raw block dev or sparse file
            "--ioengine=uring".into(),                     // io_uring if available, else libaio
            "--direct=1".into(), // bypass page‑cache (safe even with 1 TB RAM)
            format!("--rw={}", config.rw_type), // workload pattern
            // Use a **hybrid block‑size strategy**: 4 KiB for random (IOPS) workloads, 1 MiB for
            // sequential throughput.  FIO lets us override per‑job if needed, but as a rule of
            // thumb large sequential reads/writes hit peak GB/s with ≥ 1 MiB.
            if config.rw_type == "randrw" {
                "--bs=4k".into()
            } else {
                "--bs=1M".into()
            },
            // Push well beyond page‑cache yet stay inside most NVMe capacities.
            "--size=256G".into(), // ~¼ TiB – enough to observe steady‑state behaviour
            // Fan out across sockets/cores: 16 jobs × 128‑deep iodepth ≈ 2 K outstanding I/Os.
            "--numjobs=16".into(),  // parallel threads per device
            "--iodepth=128".into(), // deeper queue for PCIe Gen4/5 SSDs
            "--runtime=600".into(), // three‑minute window for convergence
            "--time_based".into(),  // use runtime, ignore size limit once sustained
            "--group_reporting".into(),
            format!("--name=fio_{}_nvme", config.name),
            "--output-format=json".into(),
            format!("--output={}", results_file.display()),
        ];

        if let Some(mix) = config.rwmixread {
            args.push(format!("--rwmixread={}", mix));
        }

        // 3. Execute
        self.logger.log_info(&format!(
            "▶︎ {}{} → {}",
            config.name,
            config
                .rwmixread
                .map_or(String::new(), |m| format!(" ({}% R)", m)),
            results_file.display()
        ));

        let output = Command::new("fio")
            .args(&args)
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to spawn FIO: {}", e))?;

        if output.status.success() {
            self.logger
                .log_info(&format!("✔ {} completed", config.name));
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "{} failed: {}",
                config.name,
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

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

    /// Formats command output for logging
    ///
    /// # Arguments
    ///
    /// * `output` - Command output as a byte array
    /// * `is_error` - Flag indicating if the output is an error
    ///
    /// # Returns
    ///
    /// * `String` - Formatted output String        
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
        self.validate()?;
        self.check_fio_installation()?;

        for config in Self::get_test_configs() {
            self.run_benchmark_type(&config)?;
        }

        self.logger.log_info("All benchmarks completed");
        Ok(())
    }

    /// Checks if FIO is installed
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or error with details
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// * FIO is not installed
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

    /// Validates the benchmark directory
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or error with details
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// * Directory creation fails
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

    /// Executes a command with given arguments
    ///
    /// # Arguments
    ///
    /// * `command` - The command to Execute
    /// * `args` - The arguments for the command
    ///
    /// # Returns
    ///
    /// * `Result<String>` - Command output or error_msg    
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

