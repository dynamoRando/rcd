#[allow(unused_imports)]
use rusqlite::{named_params, Connection, Error, Result};
use rusqlite::types::{Type};
use std::path::Path;
use table::Table;
use log::{info};

#[path = "table.rs"]
mod table;

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
pub fn execute_read(db_name: &str, cwd: &str, cmd: &str) -> Table {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path).unwrap();
    let mut statement = conn.prepare(cmd).unwrap();
    let total_columns = statement.column_count();
    let col_names = statement.column_names();
    let mut table = Table::new();

    let mut curr_idx = 0;

    for name in col_names {
        let c = table::Column {
            name: name.to_string(),
            is_nullable: false,
            idx: curr_idx,
        };

        curr_idx = curr_idx + 1;

        info!("adding col {}", c.name);

        table.add_column(c);
    }

    let mut rows = statement.query([]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        println!("reading row..");
        let mut data_row = table::Row::new();

        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from("")
            };

            let string_value = string_value;
            let col = table.get_column_by_index(i).unwrap();

            let data_item = table::Data {
                data_string: string_value,
                data_byte: Vec::new(),
            };

            let data_value = table::Value {
                data: Some(data_item),
                col: col,
            };

            data_row.add_value(data_value);
        }

        table.add_row(data_row);
    }

    return table;
}