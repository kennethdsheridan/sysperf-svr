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
