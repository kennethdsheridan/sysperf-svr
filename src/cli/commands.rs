use crate::application::Application;
use crate::ports::database_port::DatabasePort;
use crate::ports::benchmark_port::BenchmarkPort;

use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

pub fn run_benchmark<DB: DatabasePort, B: BenchmarkPort>(
    app: &mut Application<DB, B>,
    tool: &Option<String>,
) -> Result<()> {
    app.run_benchmark(tool)?;
    println!("Benchmark completed succesfully!");
    Ok(())
}
