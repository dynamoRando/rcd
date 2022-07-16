#[allow(unused_imports)]
use rusqlite::{named_params, Connection, Result, Error};
use std::path::Path;

pub fn create_database(db_name: &str, cwd: &str) -> Result<Connection, Error> {
    let db_path = Path::new(&cwd).join(&db_name);
    Connection::open(&db_path)
}
