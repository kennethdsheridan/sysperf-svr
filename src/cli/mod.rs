use crate::adapters::database_adapter::DatabaseAdapter;
use crate::application::Application;
use anyhow::Result;

pub fn run() -> Result<()> {
    let db = DatabaseAdapter::new();
    let mut app = Application::new(db);
    app.some_method()?;
    println!("CLI is running");
    Ok(())
}
