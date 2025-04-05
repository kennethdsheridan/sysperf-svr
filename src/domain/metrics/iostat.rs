//! I/O statistics collection using iostat
//!
//! This module provides functionality to collect and parse I/O statistics
//! using the iostat command-line tool. It supports both system-wide and per-device
//! statistics collection with configurable sampling intervals.
//!
//! # Example
//!
//! ```rust
//! use sysperf_svr::domain::metrics::iostat::{IostatCollector, IostatConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = IostatConfig {
//!         interval: Duration::from_secs(1),
//!         count: Some(10),
//!         per_device: true,
//!         include_extended: true,
//!     };
//!
//!     let collector = IostatCollector::new(config);
//!     let stats = collector.collect().await?;
//!
//!     for stat in stats {
//!         println!("Device {}: tps {}, read KB/s {}, write KB/s {}",
//!             stat.device, stat.tps, stat.kb_read_per_sec, stat.kb_wrtn_per_sec);
//!     }
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Errors that can occur during iostat operations
#[derive(Debug, Error)]
pub enum IostatError {
    /// Error executing the iostat command
    #[error("Failed to execute iostat command: {0}")]
    ExecutionError(String),

    /// Error parsing iostat command output
    #[error("Failed to parse iostat output: {0}")]
    ParseError(String),

    /// Invalid interval specification
    #[error("Invalid interval specified: {0}")]
    InvalidInterval(String),
}

/// Statistics for a single device or all devices combined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStats {
    /// Device name ("all" for system-wide or device name)
    pub device: String,

    /// Transfers per second
    pub tps: f64,

    /// Kilobytes read per second
    pub kb_read_per_sec: f64,

    /// Kilobytes written per second
    pub kb_wrtn_per_sec: f64,

    /// Average read request size in sectors
    pub rareq_sz: f64,

    /// Average write request size in sectors
    pub wareq_sz: f64,

    /// Average queue length
    pub aqu_sz: f64,

    /// Average service time in milliseconds
    pub await_ms: f64,

    /// Percentage of CPU time during which I/O requests were issued
    pub util: f64,
}

/// Configuration options for iostat collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IostatConfig {
    /// Time interval between samples
    pub interval: Duration,

    /// Number of samples to collect (None for continuous collection)
    pub count: Option<u32>,

    /// Whether to collect statistics for each device separately
    pub per_device: bool,

    /// Whether to include extended statistics
    pub include_extended: bool,
}

impl Default for IostatConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            count: Some(1),
            per_device: true,
            include_extended: true,
        }
    }
}

/// Collector for I/O statistics using iostat
#[derive(Debug)]
pub struct IostatCollector {
    config: IostatConfig,
}

impl IostatCollector {
    /// Creates a new IostatCollector with the specified configuration
    pub fn new(config: IostatConfig) -> Self {
        Self { config }
    }

    /// Collects I/O statistics according to the configured options
    ///
    /// # Returns
    ///
    /// Returns a Result containing a vector of DeviceStats on success, or
    /// an IostatError on failure
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - The iostat command fails to execute
    /// - The command output cannot be parsed
    /// - No valid statistics are found in the output
    pub async fn collect(&self) -> Result<Vec<DeviceStats>, IostatError> {
        let output = self.execute_iostat().await?;
        self.parse_iostat_output(&output)
    }

    /// Executes the iostat command with the configured options
    async fn execute_iostat(&self) -> Result<String, IostatError> {
        let mut cmd = tokio::process::Command::new("iostat");

        // Add -x flag for extended statistics if configured
        if self.config.include_extended {
            cmd.arg("-x");
        }

        cmd.arg(self.config.interval.as_secs().to_string());
        if let Some(count) = self.config.count {
            cmd.arg(count.to_string());
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| IostatError::ExecutionError(e.to_string()))?;

        String::from_utf8(output.stdout).map_err(|e| IostatError::ExecutionError(e.to_string()))
    }

    /// Parses the raw iostat output into DeviceStats structs
    fn parse_iostat_output(&self, output: &str) -> Result<Vec<DeviceStats>, IostatError> {
        // Implementation would parse the iostat output format
        // This is a placeholder that should be implemented based on the
        // actual iostat output format
        todo!("Implement iostat output parsing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_iostat_collector() {
        let config = IostatConfig {
            interval: Duration::from_secs(1),
            count: Some(1),
            per_device: true,
            include_extended: true,
        };

        let collector = IostatCollector::new(config);
        let result = collector.collect().await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(!stats.is_empty());
    }
}
