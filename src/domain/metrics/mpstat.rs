//! CPU performance statistics collection using mpstat
//!
//! This module provides functionality to collect and parse CPU performance statistics
//! using the mpstat command-line tool. It supports both system-wide and per-CPU statistics
//! collection with configurable sampling intervals.
//!
//! # Example
//!
//! ```rust
//! use sysperf_svr::domain::metrics::mpstat::{MpstatCollector, MpstatConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = MpstatConfig {
//!         interval: Duration::from_secs(1),
//!         count: Some(10),
//!         per_cpu: true,
//!         include_idle: true,
//!     };
//!
//!     let collector = MpstatCollector::new(config);
//!     let stats = collector.collect().await?;
//!
//!     for stat in stats {
//!         println!("CPU {}: User {}%, System {}%, Idle {}%",
//!             stat.cpu_id, stat.usr, stat.sys, stat.idle);
//!     }
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Errors that can occur during mpstat operations
#[derive(Debug, Error)]
pub enum MpstatError {
    /// Error executing the mpstat command
    #[error("Failed to execute mpstat command: {0}")]
    ExecutionError(String),

    /// Error parsing mpstat command output
    #[error("Failed to parse mpstat output: {0}")]
    ParseError(String),

    /// Invalid interval specification
    #[error("Invalid interval specified: {0}")]
    InvalidInterval(String),
}

/// Statistics for a single CPU or all CPUs combined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    /// CPU identifier ("all" for system-wide or CPU number)
    pub cpu_id: String,

    /// Percentage of CPU utilization at the user level
    pub usr: f64,

    /// Percentage of CPU utilization for user processes with nice priority
    pub nice: f64,

    /// Percentage of CPU utilization at the system level
    pub sys: f64,

    /// Percentage of time the CPU was waiting for I/O to complete
    pub iowait: f64,

    /// Percentage of time the CPU was servicing hardware interrupts
    pub irq: f64,

    /// Percentage of time the CPU was servicing software interrupts
    pub soft: f64,

    /// Percentage of time the CPU was in steal mode (hypervisor servicing another virtual processor)
    pub steal: f64,

    /// Percentage of time the CPU was running a virtual processor
    pub guest: f64,

    /// Percentage of time the CPU was running a niced guest
    pub gnice: f64,

    /// Percentage of time the CPU was idle
    pub idle: f64,

    /// Unix timestamp when these statistics were collected
    pub timestamp: i64,
}

/// Configuration options for mpstat collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpstatConfig {
    /// Time interval between samples
    pub interval: Duration,

    /// Number of samples to collect (None for continuous collection)
    pub count: Option<u32>,

    /// Whether to collect statistics for each CPU separately
    pub per_cpu: bool,

    /// Whether to include idle time statistics
    pub include_idle: bool,
}

impl Default for MpstatConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            count: Some(1),
            per_cpu: true,
            include_idle: true,
        }
    }
}

/// Collector for CPU performance statistics using mpstat
#[derive(Debug)]
pub struct MpstatCollector {
    config: MpstatConfig,
}

impl MpstatCollector {
    /// Creates a new MpstatCollector with the specified configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration options for the collector
    ///
    /// # Example
    ///
    /// ```rust
    /// use sysperf_svr::domain::metrics::mpstat::{MpstatCollector, MpstatConfig};
    ///
    /// let config = MpstatConfig::default();
    /// let collector = MpstatCollector::new(config);
    /// ```
    pub fn new(config: MpstatConfig) -> Self {
        Self { config }
    }

    /// Collects CPU statistics according to the configured options
    ///
    /// # Returns
    ///
    /// Returns a Result containing a vector of CpuStats on success, or
    /// an MpstatError on failure
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - The mpstat command fails to execute
    /// - The command output cannot be parsed
    /// - No valid statistics are found in the output
    pub async fn collect(&self) -> Result<Vec<CpuStats>, MpstatError> {
        let output = self.execute_mpstat().await?;
        self.parse_mpstat_output(&output)
    }

    /// Executes the mpstat command with the configured options
    ///
    /// # Returns
    ///
    /// Returns a Result containing the command output as a String on success,
    /// or an MpstatError on failure
    async fn execute_mpstat(&self) -> Result<String, MpstatError> {
        let mut cmd = tokio::process::Command::new("mpstat");

        cmd.arg(self.config.interval.as_secs().to_string());
        if let Some(count) = self.config.count {
            cmd.arg(count.to_string());
        }

        if self.config.per_cpu {
            cmd.arg("-P").arg("ALL");
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| MpstatError::ExecutionError(e.to_string()))?;

        String::from_utf8(output.stdout).map_err(|e| MpstatError::ExecutionError(e.to_string()))
    }

    /// Parses the raw mpstat output into structured CPU statistics
    ///
    /// # Arguments
    ///
    /// * `output` - Raw output string from mpstat command
    ///
    /// # Returns
    ///
    /// Returns a Result containing a vector of CpuStats on success,
    /// or an MpstatError on failure
    fn parse_mpstat_output(&self, output: &str) -> Result<Vec<CpuStats>, MpstatError> {
        let mut stats = Vec::new();
        let timestamp = chrono::Utc::now().timestamp();

        for line in output.lines() {
            if line.trim().is_empty() || line.starts_with("Linux") || line.starts_with("Average") {
                continue;
            }

            if let Ok(stat) = self.parse_stat_line(line, timestamp) {
                stats.push(stat);
            }
        }

        if stats.is_empty() {
            return Err(MpstatError::ParseError("No valid statistics found".into()));
        }

        Ok(stats)
    }

    /// Parses a single line of mpstat output into a CpuStats struct
    ///
    /// # Arguments
    ///
    /// * `line` - Single line from mpstat output
    /// * `timestamp` - Unix timestamp to associate with the statistics
    ///
    /// # Returns
    ///
    /// Returns a Result containing a CpuStats instance on success,
    /// or an MpstatError on failure
    fn parse_stat_line(&self, line: &str, timestamp: i64) -> Result<CpuStats, MpstatError> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 12 {
            return Err(MpstatError::ParseError(format!(
                "Invalid line format: {}",
                line
            )));
        }

        let cpu_id = if parts[2] == "all" {
            "all".to_string()
        } else {
            parts[2].to_string()
        };

        Ok(CpuStats {
            cpu_id,
            usr: parts[3]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            nice: parts[4]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            sys: parts[5]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            iowait: parts[6]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            irq: parts[7]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            soft: parts[8]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            steal: parts[9]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            guest: parts[10]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            gnice: parts[11]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            idle: parts[12]
                .parse::<f64>()
                .map_err(|e| MpstatError::ParseError(e.to_string()))?,
            timestamp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mpstat_collector() {
        let config = MpstatConfig {
            interval: Duration::from_secs(1),
            count: Some(1),
            per_cpu: true,
            include_idle: true,
        };

        let collector = MpstatCollector::new(config);
        let result = collector.collect().await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_parse_stat_line() {
        let collector = MpstatCollector::new(MpstatConfig::default());
        let timestamp = chrono::Utc::now().timestamp();

        let line = "12:00:00 PM  all    2.34    0.00    1.23    0.45    0.12    0.34    0.00    0.00    0.00   95.52";
        let result = collector.parse_stat_line(line, timestamp);

        assert!(result.is_ok());
        let stat = result.unwrap();
        assert_eq!(stat.cpu_id, "all");
        assert_eq!(stat.usr, 2.34);
        assert_eq!(stat.idle, 95.52);
    }
}
