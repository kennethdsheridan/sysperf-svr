//! System load average statistics
//!
//! This module provides functionality to collect and parse system load averages
//! from /proc/loadavg including 1, 5, and 15 minute averages, as well as task/process
//! statistics and the last process ID.
//!
//! # Example
//!
//! ```rust
//! use sysperf_svr::domain::metrics::loadavg::{LoadavgCollector, LoadavgConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = LoadavgConfig::default();
//!     let collector = LoadavgCollector::new(config);
//!     let stats = collector.collect().await?;
//!
//!     println!("1 min load average: {}", stats.load_1);
//!     println!("5 min load average: {}", stats.load_5);
//!     println!("15 min load average: {}", stats.load_15);
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use thiserror::Error;
use tokio::time;

/// Per-CPU load average statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuLoad {
    /// 1 minute load average for this CPU
    pub load_1: f64,

    /// 5 minute load average for this CPU
    pub load_5: f64,

    /// 15 minute load average for this CPU
    pub load_15: f64,
}

/// Errors that can occur when collecting load average statistics
#[derive(Error, Debug)]
pub enum LoadavgError {
    #[error("Failed to read /proc/loadavg: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse load average data: {0}")]
    ParseError(String),

    #[error("Invalid load average value: {0}")]
    InvalidValue(String),
}

/// Configuration options for load average collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadavgConfig {
    /// Time interval between samples
    pub interval: Duration,

    /// Number of samples to collect (None for continuous collection)
    pub count: Option<u32>,

    /// Whether to include detailed process statistics
    pub include_process_stats: bool,

    /// Whether to include per-CPU load averages
    pub per_cpu_stats: bool,
}

impl Default for LoadavgConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            count: Some(1),
            include_process_stats: true,
            per_cpu_stats: true,
        }
    }
}

/// Complete load average statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadavgStats {
    /// 1 minute load average
    pub load_1: f64,

    /// 5 minute load average
    pub load_5: f64,

    /// 15 minute load average
    pub load_15: f64,

    /// Number of currently running tasks
    pub running_tasks: u32,

    /// Total number of tasks
    pub total_tasks: u32,

    /// Last assigned process ID
    pub last_pid: u32,

    /// Per-CPU load statistics (if enabled)
    pub per_cpu_load: Option<Vec<CpuLoad>>,

    /// Process statistics (if enabled)
    pub process_stats: Option<ProcessStats>,
}

/// Process and task statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStats {
    /// Number of processes in running state
    pub running: u32,

    /// Number of processes in sleeping state
    pub sleeping: u32,

    /// Number of processes in uninterruptible sleep
    pub uninterruptible: u32,

    /// Number of zombie processes
    pub zombie: u32,

    /// Number of stopped processes
    pub stopped: u32,
}

/// Collector for load average statistics
#[derive(Debug)]
pub struct LoadavgCollector {
    config: LoadavgConfig,
}

impl LoadavgCollector {
    /// Creates a new LoadavgCollector with the specified configuration
    pub fn new(config: LoadavgConfig) -> Self {
        Self { config }
    }

    /// Collects load average statistics based on the configuration
    pub async fn collect(&self) -> Result<Vec<LoadavgStats>, LoadavgError> {
        let mut stats = Vec::new();
        let mut count = 0;

        loop {
            let stat = self.collect_single().await?;
            stats.push(stat);

            if let Some(max_count) = self.config.count {
                count += 1;
                if count >= max_count {
                    break;
                }
            }

            time::sleep(self.config.interval).await;
        }

        Ok(stats)
    }

    /// Collects a single sample of load average statistics
    async fn collect_single(&self) -> Result<LoadavgStats, LoadavgError> {
        let content = fs::read_to_string("/proc/loadavg").map_err(LoadavgError::IoError)?;

        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 5 {
            return Err(LoadavgError::ParseError(
                "Invalid /proc/loadavg format".to_string(),
            ));
        }

        let load_1 = parts[0]
            .parse::<f64>()
            .map_err(|e| LoadavgError::InvalidValue(e.to_string()))?;
        let load_5 = parts[1]
            .parse::<f64>()
            .map_err(|e| LoadavgError::InvalidValue(e.to_string()))?;
        let load_15 = parts[2]
            .parse::<f64>()
            .map_err(|e| LoadavgError::InvalidValue(e.to_string()))?;

        let tasks: Vec<&str> = parts[3].split('/').collect();
        if tasks.len() != 2 {
            return Err(LoadavgError::ParseError(
                "Invalid task count format".to_string(),
            ));
        }

        let running_tasks = tasks[0]
            .parse::<u32>()
            .map_err(|e| LoadavgError::InvalidValue(e.to_string()))?;
        let total_tasks = tasks[1]
            .parse::<u32>()
            .map_err(|e| LoadavgError::InvalidValue(e.to_string()))?;

        let last_pid = parts[4]
            .parse::<u32>()
            .map_err(|e| LoadavgError::InvalidValue(e.to_string()))?;

        let per_cpu_load = if self.config.per_cpu_stats {
            Some(self.collect_per_cpu_load().await?)
        } else {
            None
        };

        let process_stats = if self.config.include_process_stats {
            Some(self.collect_process_stats().await?)
        } else {
            None
        };

        Ok(LoadavgStats {
            load_1,
            load_5,
            load_15,
            running_tasks,
            total_tasks,
            last_pid,
            per_cpu_load,
            process_stats,
        })
    }

    /// Collects per-CPU load averages
    async fn collect_per_cpu_load(&self) -> Result<Vec<CpuLoad>, LoadavgError> {
        // Implementation would parse /proc/stat or similar to calculate per-CPU load
        // This is a placeholder that should be implemented based on the
        // actual system requirements and available information
        Ok(Vec::new())
    }

    /// Collects detailed process statistics
    async fn collect_process_stats(&self) -> Result<ProcessStats, LoadavgError> {
        // Implementation would parse /proc/stat or similar for process states
        // This is a placeholder that should be implemented based on the
        // actual system requirements and available information
        Ok(ProcessStats {
            running: 0,
            sleeping: 0,
            uninterruptible: 0,
            zombie: 0,
            stopped: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_loadavg_collector() {
        let config = LoadavgConfig::default();
        let collector = LoadavgCollector::new(config);

        let result = collector.collect().await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(!stats.is_empty());

        let first_stat = &stats[0];
        assert!(first_stat.load_1 >= 0.0);
        assert!(first_stat.load_5 >= 0.0);
        assert!(first_stat.load_15 >= 0.0);
        assert!(first_stat.running_tasks <= first_stat.total_tasks);
    }
}
