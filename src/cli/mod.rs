use crate::adapters::database_adapter::DatabaseAdapter;
use crate::application::Application;
use anyhow::Result;

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
    let db = DatabaseAdapter::new();
    let mut app = Application::new(db);
    app.some_method()?;
    println!("CLI is running");
    Ok(())
}

