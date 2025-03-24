mod cli_struct;
mod commands;

use self::cli_struct::{Cli, Commands};
use crate::adapters::database_adapter::DatabaseAdapter;
use crate::adapters::{benchmark_adapter::BenchmarkAdapter, metrics_adapter::MetricsAdapter};
use crate::application::Application;
use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

/// Initializes and runs the CLI application.
///
/// This function sets up the database adapter, creates an application instance,
/// calls a method on the application, and prints a status message.
///
/// # Returns
///
/// Returns a `Result<()>` which is:
/// - `Ok(())` if the application runs successfully.
/// - `Err(e)` if an error occurs during execution.
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // setup the adapters
    let db = DatabaseAdapter::new();
    let benchmark = BenchmarkAdapter::new(
        String::from("benchmark_command"),  // Default command
        vec![String::from("--default-arg")], // Default arguments
    );
    let metrics = MetricsAdapter::new();

    // create application instance
    let mut app = Application::new(db, benchmark, metrics);

    // handle the debug flag
    if cli.debug > 0 {
        println!("{}", "Debug mode is on".yellow());
        if cli.debug > 1 {
            println!("Debug level: {}", cli.debug);
        }
    }

    // handle subcommand or run interactive mode
    match &cli.command {
        Some(Commands::Benchmark { tool }) => {
            commands::run_benchmark(&mut app, tool)?;
        }
        Some(Commands::Collect { metric }) => {
            commands::collect_metrics(&mut app, &metric)?;
        }
        None => {
            commands::run_interactive(&mut app)?;
        }
    }
    println!("CLI is running");
    Ok(())
}
