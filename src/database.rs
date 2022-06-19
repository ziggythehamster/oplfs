use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use std::error::Error;
use std::path::PathBuf;

embed_migrations!();

/// Opens the database and then initializes it if needed
pub fn open(database_path: PathBuf) -> Result<SqliteConnection, Box<dyn Error>> {
    let connection = SqliteConnection::establish(&database_path.to_string_lossy())?;
    embedded_migrations::run(&connection)?;

    Ok(connection)
}
