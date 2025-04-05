//! Virtual Memory Statistics collection using vmstat
//!
//! This module provides functionality to collect and parse system statistics
//! using the vmstat command-line tool. It provides information about system memory,
//! processes, paging, block I/O, and CPU activity.
//!
//! # Example
//!
//! ```rust
//! use sysperf_svr::domain::metrics::vmstat::{VmstatCollector, VmstatConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = VmstatConfig {
//!         interval: Duration::from_secs(1),
//!         count: Some(10),
//!     };
//!
//!     let collector = VmstatCollector::new(config);
//!     let stats = collector.collect().await?;
//!
//!     for stat in stats {
//!         println!("Memory: {}MB free, Processes: {} running, {} blocked",
//!             stat.memory.free_mb,
//!             stat.procs.running,
//!             stat.procs.blocked);
//!     }
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;
use thiserror::Error;

/// Errors that can occur during vmstat operations
#[derive(Debug, Error)]
pub enum VmstatError {
    /// Error executing the vmstat command
    #[error("Failed to execute vmstat command: {0}")]
    ExecutionError(String),

    /// Error parsing vmstat command output
    #[error("Failed to parse vmstat output: {0}")]
    ParseError(String),

    /// Invalid interval specification
    #[error("Invalid interval specified: {0}")]
    InvalidInterval(String),
}

/// Process statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStats {
    /// Number of processes in runnable state
    pub running: u32,
    /// Number of processes blocked for resources
    pub blocked: u32,
}

/// Memory statistics in megabytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Free memory in MB
    pub free_mb: u64,
    /// Buffer memory in MB
    pub buffer_mb: u64,
    /// Cache memory in MB
    pub cache_mb: u64,
    /// Swap memory used in MB
    pub swap_used_mb: u64,
    /// Free swap memory in MB
    pub swap_free_mb: u64,
}

/// Swap activity statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapStats {
    /// Pages swapped in per second
    pub pages_in_per_sec: f64,
    /// Pages swapped out per second
    pub pages_out_per_sec: f64,
}

/// I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoStats {
    /// Blocks received from device per second
    pub blocks_in_per_sec: f64,
    /// Blocks sent to device per second
    pub blocks_out_per_sec: f64,
}

/// System statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    /// Number of interrupts per second
    pub interrupts_per_sec: f64,
    /// Number of context switches per second
    pub context_switches_per_sec: f64,
}

/// CPU utilization percentages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    /// Percentage of CPU time spent in user space
    pub user: f64,
    /// Percentage of CPU time spent in system space
    pub system: f64,
    /// Percentage of CPU time spent idle
    pub idle: f64,
    /// Percentage of CPU time spent waiting for I/O
    pub iowait: f64,
}

/// Complete system statistics from vmstat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmstatData {
    /// Process statistics
    pub procs: ProcessStats,
    /// Memory statistics
    pub memory: MemoryStats,
    /// Swap activity statistics
    pub swap: SwapStats,
    /// I/O statistics
    pub io: IoStats,
    /// System statistics
    pub system: SystemStats,
    /// CPU statistics
    pub cpu: CpuStats,
    /// Unix timestamp when these statistics were collected
    pub timestamp: i64,
}

/// Configuration options for vmstat collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmstatConfig {
    /// Time interval between samples
    pub interval: Duration,
    /// Number of samples to collect (None for continuous collection)
    pub count: Option<u32>,
}

impl Default for VmstatConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            count: Some(1),
        }
    }
}

/// Collector for virtual memory and system statistics using vmstat
#[derive(Debug)]
pub struct VmstatCollector {
    config: VmstatConfig,
}

impl VmstatCollector {
    /// Creates a new VmstatCollector with the specified configuration
    pub fn new(config: VmstatConfig) -> Self {
        Self { config }
    }

    /// Collects system statistics according to the configured options
    ///
    /// # Returns
    ///
    /// Returns a Result containing a vector of VmstatData on success, or
    /// a VmstatError on failure
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - The vmstat command fails to execute
    /// - The command output cannot be parsed
    /// - No valid statistics are found in the output
    pub async fn collect(&self) -> Result<Vec<VmstatData>, VmstatError> {
        let output = self.execute_vmstat().await?;
        self.parse_vmstat_output(&output)
    }

    /// Executes the vmstat command with the configured options
    async fn execute_vmstat(&self) -> Result<String, VmstatError> {
        let mut cmd = tokio::process::Command::new("vmstat");

        // Add -n flag to disable header in output
        cmd.arg("-n");

        cmd.arg(self.config.interval.as_secs().to_string());
        if let Some(count) = self.config.count {
            cmd.arg(count.to_string());
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| VmstatError::ExecutionError(e.to_string()))?;

        String::from_utf8(output.stdout).map_err(|e| VmstatError::ExecutionError(e.to_string()))
    }

    /// Parses the raw vmstat output into VmstatData structs
    fn parse_vmstat_output(&self, output: &str) -> Result<Vec<VmstatData>, VmstatError> {
        let mut stats = Vec::new();
        let timestamp = chrono::Utc::now().timestamp();

        for line in output.lines().skip(2) {
            // Skip headers
            let fields: Vec<&str> = line.split_whitespace().collect();

            if fields.len() < 17 {
                continue;
            }

            let parse_num = |idx: usize| -> Result<f64, VmstatError> {
                fields[idx].parse().map_err(|e| {
                    VmstatError::ParseError(format!("Failed to parse field {}: {}", idx, e))
                })
            };

            let stat = VmstatData {
                procs: ProcessStats {
                    running: parse_num(0)? as u32,
                    blocked: parse_num(1)? as u32,
                },
                memory: MemoryStats {
                    free_mb: parse_num(3)? as u64,
                    buffer_mb: parse_num(4)? as u64,
                    cache_mb: parse_num(5)? as u64,
                    swap_used_mb: parse_num(2)? as u64,
                    swap_free_mb: parse_num(3)? as u64,
                },
                swap: SwapStats {
                    pages_in_per_sec: parse_num(6)?,
                    pages_out_per_sec: parse_num(7)?,
                },
                io: IoStats {
                    blocks_in_per_sec: parse_num(8)?,
                    blocks_out_per_sec: parse_num(9)?,
                },
                system: SystemStats {
                    interrupts_per_sec: parse_num(10)?,
                    context_switches_per_sec: parse_num(11)?,
                },
                cpu: CpuStats {
                    user: parse_num(12)?,
                    system: parse_num(13)?,
                    idle: parse_num(14)?,
                    iowait: parse_num(15)?,
                },
                timestamp,
            };

            stats.push(stat);
        }

        if stats.is_empty() {
            return Err(VmstatError::ParseError("No valid statistics found".into()));
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vmstat_collector() {
        let config = VmstatConfig {
            interval: Duration::from_secs(1),
            count: Some(1),
        };

        let collector = VmstatCollector::new(config);
        let result = collector.collect().await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_parse_vmstat_output() {
        let collector = VmstatCollector::new(VmstatConfig::default());
        let sample_output = "
procs -----------memory---------- ---swap-- -----io---- --system-- -----cpu------
 r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st
 1  0      0 781916 195988 457784    0    0     0     0   95  142  1  1 98  0  0
";
        let result = collector.parse_vmstat_output(sample_output);
        assert!(result.is_ok());
    }
}
