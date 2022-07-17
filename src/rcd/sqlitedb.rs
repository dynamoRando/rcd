#[allow(unused_imports)]
use rusqlite::{named_params, Connection, Error, Result};
use std::path::Path;

pub fn create_database(db_name: &str, cwd: &str) -> Result<Connection, Error> {
    let db_path = Path::new(&cwd).join(&db_name);
    Connection::open(&db_path)
}

pub fn execute_write(db_name: &str, cwd: &str, cmd: &str) -> usize {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path).unwrap();
    let mut statement = conn.prepare(cmd).unwrap();
    let total_rows = statement.execute([]).unwrap();

    return total_rows;
}

#[allow(dead_code)]
pub fn execute_read(db_name: &str, cwd: &str, cmd: &str) {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path).unwrap();
    let mut _statement = conn.prepare(cmd).unwrap();

    unimplemented!("need to figure out how to return a table");
}