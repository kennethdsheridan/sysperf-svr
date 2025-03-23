use anyhow::Result;

pub trait BenchmarkPort {
    fn run_stress_ng(&self) -> Result<String>;
    fn run_fio(&self) -> Result<String>;
    fn run_pgbench(&self) -> Result<String>;
    fn run_ibnetdiscover(&self) -> Result<String>;
}
