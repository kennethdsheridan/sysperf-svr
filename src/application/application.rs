//! Core application implementing the hexagonal architecture pattern.
//! This module serves as the primary orchestrator for the system performance testing application.

use std::sync::Arc;

use crate::ports::benchmark_port::BenchmarkPort;
use crate::ports::database_port::DatabasePort;
use crate::ports::log_port::LoggerPort;
use crate::ports::metrics_port::MetricsPort;

/// Main application struct that coordinates all core functionality.
///
/// # Type Parameters
/// * `DB` - Database adapter type that implements DatabasePort
/// * `B`  - Benchmark adapter type that implements benchmark_port
/// * `M`  - Metrics adapter type that implements MetricsPort
/// * `L`  - Logger adapter type that implements LoggerPort
///
/// # Generic Constraints
/// All type parameters must implement their respective port traits, enabling
/// dependency inversion and making the application independent of specific implementations.
pub struct Application {
    pub db: Arc<dyn DatabasePort>,
    pub benchmark: Arc<dyn BenchmarkPort>,
    pub metrics: Arc<dyn MetricsPort>,
    pub logger: Arc<dyn LoggerPort>,
}

impl Application {
    pub fn new(
        db: Arc<dyn DatabasePort>,
        benchmark: Arc<dyn BenchmarkPort>,
        metrics: Arc<dyn MetricsPort>,
        logger: Arc<dyn LoggerPort>,
    ) -> Self {
        Self {
            db,
            benchmark,
            metrics,
            logger,
        }
    }

    pub fn run_benchmark(&self) -> anyhow::Result<()> {
        self.benchmark.run()?;
        Ok(())
    }
}
