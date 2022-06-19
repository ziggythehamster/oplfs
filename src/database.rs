use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use std::error::Error;
use std::path::PathBuf;

embed_migrations!();

/// Opens the database and initializes it if needed
///
/// # Examples
///
/// ```
/// use oplfs::database;
/// use std::path::PathBuf;
///
/// let database_path = PathBuf::from("test.sqlite");
/// # let database_path: PathBuf = [std::env::temp_dir(), PathBuf::from("test.sqlite")].iter().collect();
/// # let database_path_to_delete = database_path.clone();
/// let connection    = database::open(database_path)?;
///
/// // use Diesel on this connection
///
/// # std::fs::remove_file(database_path_to_delete.as_path());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn open(database_path: PathBuf) -> Result<SqliteConnection, Box<dyn Error>> {
    let connection = SqliteConnection::establish(&database_path.to_string_lossy())?;
    embedded_migrations::run(&connection)?;

    Ok(connection)
}
