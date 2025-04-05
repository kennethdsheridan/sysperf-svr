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
pub struct Application<DB, B, M>
where
    DB: DatabasePort,
    B: BenchmarkPort,
    M: MetricsPort,
{
    pub db: DB,
    pub benchmark: B,
    pub metrics: M,
    pub logger: Arc<dyn LoggerPort>,
}

impl<DB, B, M> Application<DB, B, M>
where
    DB: DatabasePort,
    B: BenchmarkPort,
    M: MetricsPort,
{
    pub fn new(db: DB, benchmark: B, metrics: M, logger: Arc<dyn LoggerPort>) -> Self {
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

