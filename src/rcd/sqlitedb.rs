#[allow(unused_imports)]
use rusqlite::{named_params, Connection, Error, Result};
use std::path::Path;
use chrono::{DateTime, UTC};

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

pub fn execute_read_example(db_name: &str, cwd: &str, cmd: &str) {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path).unwrap();
    let mut statement = conn.prepare(cmd).unwrap();

    fn c(row: &rusqlite::Row) -> DateTime<UTC> {
        row.get(0)
    }

    let results = statement.query_map([], c as fn(&rusqlite::Row) -> DateTime<UTC>).unwrap();
    let v_results = results.collect();
}