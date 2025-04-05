use clap::{Parser, Subcommand};

/// System performance benchmarking and metrics collection tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// optiional name operate on
    #[arg(short, long)]
    pub name: Option<String>,

    /// set a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<std::path::PathBuf>,

    /// turn debugging informaiton on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs all benchmarks
    Benchmark {
        /// Specify which benchmark to run
        #[arg(short, long)]
        tool: Option<String>,
    },
    /// Collects system metrics
    Collect {
        /// Specify which metric to collect
        #[arg(short, long)]
        metric: Option<String>,
    },
}
