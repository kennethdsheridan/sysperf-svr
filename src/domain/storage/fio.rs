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

#[derive(Debug, Serialize, Deserialize)]
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
