use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Configuration for running system benchmarks.
/// Provides a flexible structure for defining different types of performance tests
/// across various benchmarking tools.
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Unique identifier for the benchmark run
    pub name: String,
    /// The benchmarking tool to use (e.g., FIO, StressNg)
    pub tool: BenchmarkTool,
    /// Tool-specific parameters for the benchmark
    pub params: BenchmarkParams,
}

/// Supported benchmarking tools in the system.
/// Each variant represents a different specialized testing tool
/// that can be used for specific types of performance analysis.
#[derive(Debug, Serialize, Deserialize)]
pub enum BenchmarkTool {
    /// Flexible I/O Tester - for storage subsystem benchmarking
    FIO,
    /// Stress-Next Generation - for system stress testing and CPU benchmarking
    StressNg,
    // Easy to add more benchmark tools
}

/// Container for tool-specific benchmark parameters.
/// Allows for type-safe parameter passing to different benchmark tools
/// while maintaining a common interface.
#[derive(Debug, Serialize, Deserialize)]
pub enum BenchmarkParams {
    /// Parameters specific to FIO benchmarks
    FIO(FIOParams),
    /// Parameters specific to stress-ng benchmarks
    StressNg(StressNgParams),
}

/// Configuration parameters for FIO benchmarks.
/// Defines the complete set of parameters needed to run
/// storage performance tests using FIO.
#[derive(Debug, Serialize, Deserialize)]
pub struct FIOParams {
    /// Target directory for I/O operations
    pub directory: String,
    /// I/O block size (e.g., "4k", "1m")
    pub block_size: String,
    /// Type of I/O operation to perform
    pub io_type: IOType,
    /// Total size of the test file
    pub size: String,
    /// Duration of the test in seconds
    pub runtime: u32,
    /// Number of parallel jobs to run
    pub num_jobs: u32,
    /// Queue depth for I/O operations
    pub io_depth: u32,
}

/// Configuration parameters for stress-ng benchmarks.
/// Defines parameters for system stress testing and
/// CPU performance evaluation.
#[derive(Debug, Serialize, Deserialize)]
pub struct StressNgParams {
    /// Target CPU load percentage
    pub cpu_load: u32,
    /// Test duration in seconds
    pub duration: u32,
    /// Number of worker threads
    pub workers: u32,
}

/// Types of I/O operations supported in storage benchmarks.
/// Covers the fundamental patterns of storage access that
/// need to be tested in a storage subsystem.
#[derive(Debug, Serialize, Deserialize)]
pub enum IOType {
    /// Sequential read operations
    SequentialRead,
    /// Random read operations
    RandomRead,
    /// Sequential write operations
    SequentialWrite,
    /// Random write operations
    RandomWrite,
}

/// Results from a benchmark run.
/// Provides both processed metrics and raw output for
/// detailed analysis and verification.
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// The tool used for this benchmark
    pub tool: BenchmarkTool,
    /// Processed benchmark metrics
    pub metrics: BenchmarkMetrics,
    /// Raw output from the benchmark tool
    pub raw_output: String,
}

/// Container for tool-specific benchmark metrics.
/// Allows for structured storage of different types of
/// performance metrics based on the benchmark tool used.
#[derive(Debug, Serialize, Deserialize)]
pub enum BenchmarkMetrics {
    /// Metrics specific to FIO benchmarks
    FIO(FIOMetrics),
    /// Metrics specific to stress-ng benchmarks
    StressNg(StressNgMetrics),
}

/// Performance metrics collected from FIO benchmarks.
/// Captures key storage performance indicators that are
/// critical for storage subsystem evaluation.
#[derive(Debug, Serialize, Deserialize)]
pub struct FIOMetrics {
    /// I/O operations per second
    pub iops: f64,
    /// Throughput in bytes per second
    pub bandwidth: f64,
    /// Average I/O latency in microseconds
    pub latency: f64,
}

/// Performance metrics collected from stress-ng benchmarks.
/// Captures system stress test results and CPU performance metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct StressNgMetrics {
    /// Bogus operations per second (stress-ng's performance metric)
    pub bogo_ops: f64,
    /// CPU utilization percentage during the test
    pub cpu_usage: f64,
}

/// Port interface for benchmark operations.
/// Defines the contract that benchmark adapters must implement
/// to provide consistent benchmarking capabilities across different tools.
#[async_trait]
pub trait BenchmarkPort {
    /// Executes the configured benchmark
    fn run(&self) -> Result<()>;
    /// Executes an FIO benchmark with current configuration
    fn run_fio(&self) -> Result<String>;
    /// Validates the benchmark configuration
    fn validate(&self) -> Result<()>;
    /// Executes a system command with given arguments
    fn run_command(&self, command: &str, args: &str) -> Result<String>;
}

