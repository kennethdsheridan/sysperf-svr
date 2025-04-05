//! SysPerf: A comprehensive supercomputing benchmark framework
//!
//! SysPerf provides a modular and extensible platform for performance analysis
//! in HPC environments. It follows a hexagonal architecture pattern to maintain
//! clear separation of concerns and enable flexible component swapping.
//!
//! # Features
//! - Storage system benchmarking via FIO integration
//! - System metrics collection and analysis
//! - Configurable I/O patterns and test scenarios
//! - Persistent metrics storage
//! - Comprehensive reporting capabilities
//!
//! # Architecture
//! The system uses a ports and adapters pattern with:
//! - Storage ports for persistent data operations
//! - Metrics ports for system telemetry
//! - Benchmarking ports for performance testing
//!
//! # Example Usage
//! ```bash
//! # Run basic FIO test
//! sysperf-svr benchmark --tool fio --test-type random-read
//!
//! # Run comprehensive storage suite
//! sysperf-svr storage-suite
//!
//! # View results
//! sysperf-svr results --latest
//! ```

use anyhow::Result;
use sysperf_svr::cli;

/// Application entry point that initializes and runs the CLI interface.
///
/// This function serves as the main entry point for the SysPerf application.
/// It delegates to the CLI module for command processing and handles any
/// errors that occur during execution.
///
/// # Returns
/// - `Ok(())` if the program executes successfully
/// - `Err` if any error occurs during execution
///
/// # Error Handling
/// Uses `anyhow::Result` for comprehensive error handling, capturing both
/// low-level system errors and application-specific errors.
fn main() -> Result<()> {
    cli::run()?;
    Ok(())
}

