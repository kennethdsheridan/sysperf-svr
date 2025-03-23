mod cli_struct;

use self::cli_struct::{Cli, Commands};
use crate::adapters::database_adapter::DatabaseAdapter;
use crate::adapters::{benchmark_adapter::BenchmarkAdapter, metrics_adapter::MetricsAdatper};
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
    let cli = Cli.parse();

    // setup the adapters
    let db = DatabaseAdapter::new();
    let benchmark = BenchmarkAdapter;
    let metrics = MetricsAdatper::new();

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
            run_benchmark(&app, tool)?;
        }
        Some(Commands::Collect { metric }) => {
            collect_metrics(&app, &metric)?;
        }
        None => {
            run_interactive(&app)?;
        }
    }
    println!("CLI is running");
    Ok(())
}

fn run_benchmark(app: &Application, tool: &Option<String) -> Result<()> {
    // implement benchmarking logic here
    Ok(())
}

fn collect_metrics(app: &Application, metric: &Option<String>) -> Result<()> {
    // implement metric collection logic here
    Ok(())  
}

fn run_interactive(app: &Application) -> Result<()> {
    println!("{}", "Welcome to the System Performance Tool!".green().bold());

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(&["Run Benchmarks", "Collect Metrics", "Exit"])
            .interact()?;

        match selection {
            0 => run_interactive_benchmark(app)?,
            1 => run_interactive_metrics(app)?,
            2 => break,
            _ => unreachable!(),
        }

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to perform another action?")
            .default(true)
            .interact()?
        {
            break;
        }
    }

    println!("{}", "Thank you for using the System Performance Tool!".green().bold());
    Ok(())
}

fn run_interactive_benchmark(app: &Application) -> Result<()> {
    // Implementation for interactive benchmark selection
    // ...
    Ok(())
}

fn run_interactive_metrics(app: &Application) -> Result<()> {
    // Implementation for interactive metrics selection
    // ...
    Ok(())
}
