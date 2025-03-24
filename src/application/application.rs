use crate::ports::benchmark_port::BenchmarkPort;
use crate::ports::database_port::DatabasePort;
use crate::ports::metrics_port::MetricsPort;

pub struct Application<DB, B, M>
where
    DB: DatabasePort,
    B: BenchmarkPort,
    M: MetricsPort,
{
    db: DB,
    benchmark: B,
    metrics: M,
}

impl<DB, B, M> Application<DB, B, M>
where
    DB: DatabasePort,
    B: BenchmarkPort,
    M: MetricsPort,
{
    pub fn new(db: DB, benchmark: B, metrics: M) -> Self {
        Self {
            db,
            benchmark,
            metrics,
        }
    }
    pub fn run_benchmark(&self) -> anyhow::Result<()> {
        self.benchmark.run()?;
        Ok(())
    }

    pub fn some_method(&mut self) -> anyhow::Result<()> {
        // Change &self to &mut self
        self.db.set("key", "value")?;
        let value = self.db.get("key")?;
        println!("Value: {:?}", value);
        Ok(())
    }
}
