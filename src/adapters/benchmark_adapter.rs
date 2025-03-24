use anyhow::Result;
use std::process::Command;
use crate::ports::benchmark_port::BenchmarkPort;

pub struct BenchmarkAdapter {
    command: String,
    args: Vec<String>,
}

impl BenchmarkAdapter {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }
}

impl BenchmarkPort for BenchmarkAdapter {
    fn run(&self) -> Result<String> {
        let output = Command::new(&self.command)
            .args(&self.args)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(anyhow::anyhow!(
                "Benchmark command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn run_fio(&self) -> Result<String> {
        // implement fio command
        Ok(String::from("fio output"))
    }
}
    
