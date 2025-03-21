use crate::ports::database_port::DatabasePort;

pub struct Application<DB: DatabasePort> {
    db: DB,
}

impl<DB: DatabasePort> Application<DB> {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub fn some_method(&mut self) -> anyhow::Result<()> {  // Change &self to &mut self
        self.db.set("key", "value")?;
        let value = self.db.get("key")?;
        println!("Value: {:?}", value);
        Ok(())
    }
}
