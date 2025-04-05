use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub name: String,
    pub tool: BenchmarkTool,
    pub params: BenchmarkParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BenchmarkTool {
    FIO,
    StressNg,
    // Easy to add more benchmark tools
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BenchmarkParams {
    FIO(FIOParams),
    StressNg(StressNgParams),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FIOParams {
    pub directory: String,
    pub block_size: String,
    pub io_type: IOType,
    pub size: String,
    pub runtime: u32,
    pub num_jobs: u32,
    pub io_depth: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StressNgParams {
    pub cpu_load: u32,
    pub duration: u32,
    pub workers: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IOType {
    SequentialRead,
    RandomRead,
    SequentialWrite,
    RandomWrite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub tool: BenchmarkTool,
    pub metrics: BenchmarkMetrics,
    pub raw_output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BenchmarkMetrics {
    FIO(FIOMetrics),
    StressNg(StressNgMetrics),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FIOMetrics {
    pub iops: f64,
    pub bandwidth: f64,
    pub latency: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StressNgMetrics {
    pub bogo_ops: f64,
    pub cpu_usage: f64,
}

#[async_trait]

pub trait BenchmarkPort {
    fn run(&self) -> Result<()>;
    //    fn run_stress_ng(&self) -> Result<String>;
    fn run_fio(&self) -> Result<String>;
    fn validate(&self) -> Result<()>;
    fn run_command(&self, command: &str, args: &str) -> Result<String>;
}
