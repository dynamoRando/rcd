use crate::rcd_enum::{DatabaseType, DmlType};

pub mod sqlite;

#[allow(dead_code, unused_variables)]
/// Takes a SQL statement and returns a list of tables involved in that SQL statement
pub fn get_table_names(cmd: &str, db_type: DatabaseType) -> Vec<String> {
    unimplemented!();
}

#[allow(dead_code, unused_variables)]
pub fn get_values_from_insert_statement(
    insert_statement: &str,
    db_type: DatabaseType,
) -> Vec<String> {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => return sqlite::get_values_from_insert_statement(insert_statement),
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
    }
}

#[allow(dead_code, unused_variables)]
pub fn get_table_name(cmd: &str, db_type: DatabaseType) -> String {
    match db_type {
        DatabaseType::Unknown => todo!(),
        DatabaseType::Sqlite => return sqlite::get_table_name(cmd, db_type),
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
    }
}

#[allow(dead_code, unused_variables)]
pub fn determine_dml_type(cmd: &str, db_type: DatabaseType) -> DmlType {
    match db_type {
        DatabaseType::Mysql => unimplemented!(),
        DatabaseType::Unknown => panic!(),
        DatabaseType::Sqlite => return sqlite::determine_statement_type(cmd.to_string()),
        DatabaseType::Postgres => unimplemented!(),
        DatabaseType::Sqlserver => unimplemented!(),
    }
}
