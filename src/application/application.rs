use crate::ports::database_port::DatabasePort;
use crate::ports::benchmark_port::BenchmarkPort;

pub struct Application<DB: DatabasePort> {
    db: DB,
    benchmark: String,
    metrics: Vec<String>,
}

impl<DB: DatabasePort, B: BenchmarkPort> Application<DB, B> {
    pub fn new(db: DB, benchmark: String, metrics: Vec<String>) -> Self {
        Self { db, benchmark, metrics }
    }
    pub fn run_benchmark(&self) -> Result<()> {
        self.benchmark.run()?;
        Ok(())
    }

    pub fn some_method(&mut self) -> anyhow::Result<()> {  // Change &self to &mut self
        self.db.set("key", "value")?;
        let value = self.db.get("key")?;
        println!("Value: {:?}", value);
        Ok(())
    }
}
