use anyhow::Result;

pub trait BenchmarkPort {
    fn run(&self) -> Result<()>;
    //    fn run_stress_ng(&self) -> Result<String>;
    fn run_fio(&self) -> Result<String>;
}
