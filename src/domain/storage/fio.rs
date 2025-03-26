//! FIO (Flexible I/O Tester) configuration and result handling.
//!
//! This module provides a type-safe interface for configuring and running FIO benchmarks
//! on any storage target. It supports all major FIO features including:
//!
//! - Multiple I/O engines (sync, psync, libaio, io_uring, etc.)
//! - Various I/O patterns (sequential, random, mixed)
//! - Both file and device targets
//! - Direct and buffered I/O
//! - Comprehensive performance metrics
//!
//! # Example
//! ```no_run
//! use sysperf_svr::domain::storage::fio::{FioJobConfig, StorageTarget, IoEngine, IoPattern};
//!
//! let target = StorageTarget {
//!     path: "/dev/nvme0n1".into(),
//!     target_type: "device".into(),
//!     options: Default::default(),
//! };
//!
//! let job = FioJobConfig {
//!     ioengine: IoEngine::Libaio,
//!     rw: IoPattern::RandRead,
//!     bs: "4k".into(),
//!     size: "1G".into(),
//!     numjobs: 4,
//!     iodepth: 32,
//!     direct: true,
//!     ..Default::default()
//! };
//! ```
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IoEngine {
    Sync,
    Psync,
    Libaio,
    IoUring,
    External(String),
    #[serde(other)]
    Other,
}

/// I/O patterns supported by FIO.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IoPattern {
    Read,
    Write,
    RandRead,
    RandWrite,
    RandRW,
    Trim,
}

/// Configuration for a single FIO job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FioJobConfig {
    /// I/O engine to use for the Tester
    pub ioengine: IoEngine,
    /// I/O pattern to Tester
    pub rw: IoPattern,
    /// Block size for I/O operations (e.g. "4k", "1M")
    pub bs: String,
    /// Total size per file/device to Tester
    pub size: String,
    /// Number of concurrent jobs to running
    pub numjobs: u32,
    /// I/O queue depth
    pub iodepth: u32,
    /// Use O_DIRECT flag for non-buffered I/O
    #[serde(default)]
    pub direct: bool,
    /// Use buffered I/O
    #[serde(default)]
    pub buffered: bool,
    /// Percentage of reads for mixed workloads
    /// Only applicable when rw = RandRW
    #[serde(default)]
    pub rwmixread: Option<u32>,
    /// Additional job-specific options
    #[serde(flatten)]
    pub extra_options: HashMap<String, String>,
}

/// Storage target configuration for FIO testing.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageTarget {
    /// Path to the target (file, device, or directory)
    pub path: PathBuf,

    /// Type of the target ("file", "device", "directory")
    pub target_type: String,

    /// Target-specific options
    #[serde(default)]
    pub options: HashMap<String, String>,
}

/// Statistics for a specific type of I/O operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct IoStats {
    /// Operations per second
    pub iops: f64,
    /// Bandwidth in megabytes per second
    pub bandwidth_mb: f64,
    /// Average latency in microseconds
    pub lat_usec: f64,
    /// 99th percentile latency in microseconds
    pub lat_usec_p99: f64,
    /// Maximum observed latency in microseconds
    pub lat_usec_max: f64,
}

/// Results from a FIO benchmark run.
#[derive(Debug, Serialize, Deserialize)]
pub struct FioResult {
    /// Read operation statistics
    pub read: IoStats,
    /// Write operation statistics
    pub write: IoStats,
    /// Error information if the benchmark failed
    pub error: Option<String>,
}

impl Default for FioJobConfig {
    fn default() -> Self {
        Self {
            ioengine: IoEngine::Sync,
            rw: IoPattern::Read,
            bs: String::from("4k"),
            size: String::from("1G"),
            numjobs: 1,
            iodepth: 1,
            direct: false,
            buffered: true,
            rwmixread: None,
            extra_options: HashMap::new(),
        }
    }
}

impl Default for IoStats {
    fn default() -> Self {
        Self {
            iops: 0.0,
            bandwidth_mb: 0.0,
            lat_usec: 0.0,
            lat_usec_p99: 0.0,
            lat_usec_max: 0.0,
        }
    }
}

impl Default for FioResult {
    fn default() -> Self {
        Self {
            read: IoStats::default(),
            write: IoStats::default(),
            error: None,
        }
    }
}

impl FioJobConfig {
    /// Creates a new FIO job configuration for sequential read testing.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysperf_svr::domain::storage::fio::FioJobConfig;
    ///
    /// let config = FioJobConfig::new_sequential_read("4k", "1G", 4, 32);
    /// assert_eq!(config.bs, "4k");
    /// assert_eq!(config.numjobs, 4);
    /// ```
    pub fn new_sequential_read(bs: &str, size: &str, numjobs: u32, iodepth: u32) -> Self {
        Self {
            ioengine: IoEngine::Libaio,
            rw: IoPattern::Read,
            bs: bs.to_string(),
            size: size.to_string(),
            numjobs,
            iodepth,
            direct: true,
            ..Default::default()
        }
    }

    /// Creates a new FIO job configuration for random write testing.
    pub fn new_random_write(bs: &str, size: &str, numjobs: u32, iodepth: u32) -> Self {
        Self {
            ioengine: IoEngine::Libaio,
            rw: IoPattern::RandWrite,
            bs: bs.to_string(),
            size: size.to_string(),
            numjobs,
            iodepth,
            direct: true,
            ..Default::default()
        }
    }
}

impl StorageTarget {
    /// Creates a new file-based storage target.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file or directory
    /// * `options` - Optional target-specific configuration
    pub fn new_file<P: Into<PathBuf>>(path: P, options: Option<HashMap<String, String>>) -> Self {
        Self {
            path: path.into(),
            target_type: "file".to_string(),
            options: options.unwrap_or_default(),
        }
    }

    /// Creates a new device-based storage target.
    pub fn new_device<P: Into<PathBuf>>(path: P, options: Option<HashMap<String, String>>) -> Self {
        Self {
            path: path.into(),
            target_type: "device".to_string(),
            options: options.unwrap_or_default(),
        }
    }
}
