use super::{
    execute_read_at_participant, execute_read_on_connection_for_row, get_db_conn_with_result,
    get_scalar_as_string, get_scalar_as_u32, get_scalar_as_u64, get_table_col_names,
};
use crate::dbi::sqlite::{
    execute_write, get_table_col_names_with_data_type_as_string, has_table, sql_text,
};
use crate::dbi::{
    get_data_log_table_name, get_data_queue_table_name, get_metadata_table_name, DbiConfigSqlite,
    PartialDataResult,
};
use rcd_core::rcd_enum::ColumnType;
use crate::table::Table;
use chrono::Utc;
use rcdproto::rcdp::{ColumnSchema, Contract, PendingStatement, TableSchema};
use rusqlite::types::Type;
use rusqlite::{Connection, Result};
use std::path::Path;

pub mod delete;
pub mod insert;
pub mod update;

pub fn accept_pending_action_at_participant(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let mut action_result = PartialDataResult {
        is_successful: false,
        row_id: 0,
        data_hash: None,
        partial_data_status: None,
        action: None,
    };

    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let queue_table_name = get_data_queue_table_name(table_name);
    let mut cmd = String::from("SELECT STATEMENT FROM :table_name WHERE ID = :rid");
    cmd = cmd.replace(":table_name", &queue_table_name);
    cmd = cmd.replace(":rid", &row_id.to_string());

    let sql_update_statement = get_scalar_as_string(cmd, &conn);
    let mut cmd = String::from("SELECT WHERE_CLAUSE FROM :table_name WHERE ID = :rid");
    cmd = cmd.replace(":table_name", &queue_table_name);
    cmd = cmd.replace(":rid", &row_id.to_string());
    let where_clause = get_scalar_as_string(cmd, &conn);

    cmd = String::from("SELECT ACTION FROM :table_name WHERE ID = :rid");
    cmd = cmd.replace(":table_name", &queue_table_name);
    cmd = cmd.replace(":rid", &row_id.to_string());

    let action = get_scalar_as_string(cmd, &conn);

    if action == "UPDATE" {
        action_result = update::handle_update_pending_action(
            db_name,
            table_name,
            &sql_update_statement,
            &where_clause,
            row_id,
            config,
        );
    }

    if action == "DELETE" {
        action_result = delete::handle_delete_pending_action(
            db_name,
            table_name,
            &sql_update_statement,
            &where_clause,
            row_id,
            config,
        );
    }

    return action_result;
}

pub fn get_data_hash_at_participant(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> u64 {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let metadata_table_name = get_metadata_table_name(table_name);
    let mut cmd = String::from("SELECT HASH FROM :metadata WHERE ROW_ID = :row_id");
    cmd = cmd.replace(":metadata", &metadata_table_name);
    cmd = cmd.replace(":row_id", &row_id.to_string());

    return get_scalar_as_u64(cmd, &conn).unwrap();
}

pub fn get_pending_actions(
    db_name: &str,
    table_name: &str,
    action: &str,
    config: &DbiConfigSqlite,
) -> Vec<PendingStatement> {
    let update_queue = get_data_queue_table_name(table_name);

    let mut pending_statements: Vec<PendingStatement> = Vec::new();

    let mut cmd = String::from(
        "
        SELECT 
            ID,
            STATEMENT,
            REQUESTED_TS_UTC,
            HOST_ID
        FROM
            :table
        WHERE
            ACTION = ':action'
        ;",
    );
    cmd = cmd.replace(":table", &update_queue);
    cmd = cmd.replace(":action", &action);

    let pending_rows = execute_read_at_participant(db_name, &cmd.to_string(), &config).unwrap();

    for row in &pending_rows.rows {
        let mut rid: u32 = 0;
        let mut statement: String = String::from("");
        let mut ts: String = String::from("");
        let mut host_id: String = String::from("");

        for val in &row.vals {
            if val.col.name == "ID" {
                rid = val
                    .data
                    .as_ref()
                    .unwrap()
                    .data_string
                    .parse::<u32>()
                    .unwrap();
            }

            if val.col.name == "STATEMENT" {
                statement = val.data.as_ref().unwrap().data_string.clone();
            }

            if val.col.name == "REQUESTED_TS_UTC" {
                ts = val.data.as_ref().unwrap().data_string.clone();
            }

            if val.col.name == "HOST_ID" {
                host_id = val.data.as_ref().unwrap().data_string.clone();
            }
        }

        let ps = PendingStatement {
            row_id: rid,
            statement,
            requested_ts_utc: ts,
            host_id,
            action: action.to_string(),
        };

        pending_statements.push(ps);
    }

    return pending_statements;
}

pub fn get_row_from_partial_database(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> rcdproto::rcdp::Row {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut cmd = String::from("SELECT * from :table_name WHERE ROWID = :rid");

    cmd = cmd.replace(":table_name", table_name);
    cmd = cmd.replace(":rid", &row_id.to_string());

    return execute_read_on_connection_for_row(db_name, table_name, row_id, cmd, &conn).unwrap();
}

pub fn create_partial_database_from_contract(
    contract: &Contract,
    config: &DbiConfigSqlite,
) -> bool {
    println!("{:?}", config);

    let db_name = contract.schema.as_ref().unwrap().database_name.clone();
    let _ = create_partial_database(&db_name, config);

    let conn = get_partial_db_connection(&db_name, &config.root_folder);

    for table in &contract.schema.as_ref().unwrap().tables {
        create_table_from_schema(table, &conn);
    }

    return true;
}

pub fn create_partial_database(
    db_name: &str,
    config: &DbiConfigSqlite,
) -> Result<Connection, rusqlite::Error> {
    let mut db_part_name = db_name.replace(".db", "");
    db_part_name = db_part_name.replace(".dbpart", "");
    db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
    return get_db_conn_with_result(config, &db_part_name);
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn get_db_id(db_name: &str, config: &DbiConfigSqlite) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn get_table_id(db_name: &str, table_name: &str, config: &DbiConfigSqlite) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn create_table_in_partial_database(
    db_name: &str,
    table_name: &str,
    schema: Vec<ColumnSchema>,
    config: &DbiConfigSqlite,
) -> Result<bool> {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn add_row_to_partial_database(db_name: &str, table_name: &str, row_data: Table) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn update_row_in_partial_database(db_name: &str, table_name: &str, row_data: Table) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn save_contract(db_name: &str, table_name: &str, row_data: Table) -> String {
    unimplemented!();
}

pub fn read_row_id_from_part_db(
    db_name: &str,
    table_name: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> u32 {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause");
    cmd = cmd.replace(":table_name", table_name);
    cmd = cmd.replace(":where_clause", where_clause);

    let row_id = get_scalar_as_u32(cmd, &conn);

    return row_id;
}

pub fn get_partial_db_connection(db_name: &str, cwd: &str) -> Connection {
    let mut db_part_name = db_name.replace(".db", "");
    db_part_name = db_part_name.replace(".dbpart", "");
    db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
    let db_path = Path::new(&cwd).join(&db_part_name);
    let conn = Connection::open(&db_path).unwrap();
    return conn;
}

fn create_table_from_schema(table_schema: &TableSchema, conn: &Connection) {
    let table_name = table_schema.table_name.clone();
    let mut cmd = String::from("CREATE TABLE IF NOT EXISTS :tablename ");
    cmd = cmd.replace(":tablename", &table_name);
    cmd = cmd + " ( ";

    for column in &table_schema.columns {
        let col_name = column.column_name.clone();
        let col_type = ColumnType::from_u32(column.column_type).data_type_as_string_sqlite();
        let mut col_length = String::from("");

        if column.column_length > 0 {
            col_length = col_length + " ( ";
            col_length = col_length + &column.column_length.to_string();
            col_length = col_length + " ) ";
        }

        let mut col_nullable = String::from("");

        if !column.is_nullable {
            col_nullable = String::from("NOT NULL");
        }

        let col_statement: String;

        let last_column = &table_schema.columns.last().unwrap().column_name;

        if last_column.to_string() == column.column_name {
            col_statement = format!(
                " {} {} {} {} ",
                col_name, col_type, col_length, col_nullable
            );
        } else {
            col_statement = format!(
                " {} {} {} {} , ",
                col_name, col_type, col_length, col_nullable
            );
        }

        cmd = cmd + &col_statement;
    }
    cmd = cmd + " ) ";
    execute_write(conn, &cmd);
}

fn add_record_to_log_table(
    db_name: &str,
    table_name: &str,
    where_clause: &str,
    action: &str,
    config: &DbiConfigSqlite,
) -> bool {
    let data_log_table = get_data_log_table_name(table_name);
    let conn = &get_partial_db_connection(db_name, &config.root_folder);

    if !has_table(data_log_table.clone(), conn) {
        let mut cmd = sql_text::COOP::text_create_data_log_table();
        let table_col_names =
            get_table_col_names_with_data_type_as_string(db_name, table_name, config);
        cmd = cmd.replace(":column_list", &table_col_names);
        cmd = cmd.replace(":table_name", &data_log_table);

        execute_write(conn, &cmd);
    }

    // we first need to determine the rows that we're about to overwrite and get them so we can insert them
    let col_names_vec = get_table_col_names(table_name.to_string(), conn);
    let mut col_names = String::from("");
    let mut original_col_names = String::from("");

    for name in &col_names_vec {
        let item = format!("{}{}", name, ",");
        col_names = format!("{}{}", col_names, item);
        original_col_names = format!("{}{}", original_col_names, item);
    }

    // remove the final comma from the list of original column names
    original_col_names = original_col_names[0..original_col_names.len() - 1].to_string();

    // for the list of column names, add rowid as a column to get from the db
    col_names = format!("{}{}", col_names, "ROWID");

    let mut select_cmd = String::from("SELECT :col_names FROM :table_name WHERE :where_clause");
    select_cmd = select_cmd.replace(":col_names", &col_names);
    select_cmd = select_cmd.replace(":table_name", table_name);
    select_cmd = select_cmd.replace(":where_clause", where_clause);

    let mut stmt = conn.prepare(&select_cmd).unwrap();
    let mut rows = stmt.query([]).unwrap();

    // for every row that we find that we're going to change, we want to insert a copy of it into the data_log_table
    while let Some(row) = rows.next().unwrap() {
        let mut insert_cmd = String::from("INSERT INTO :data_log_table ( :cols, ROW_ID, ACTION, TS_UTC ) VALUES ( :col_vals, :rid, ':action', ':ts_utc') ");
        insert_cmd = insert_cmd.replace(":data_log_table", &data_log_table);
        insert_cmd = insert_cmd.replace(":cols", &original_col_names);

        // need to build the rest of the insert statement - the column values, rowid, etc.
        let mut col_vals = String::from("");
        let total_cols = col_names_vec.len();

        // iterate over the column names and get the value for each as a string
        // remember, the last item is the ROWID, which is not in this list and we will need to get
        for i in 0..col_names_vec.len() {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => {
                    format!("'{}'", row.get_ref_unwrap(i).as_str().unwrap().to_string())
                }
                _ => String::from(""),
            };

            // add the value to the list of values to insert
            col_vals = format!("{}{}{}", col_vals, string_value, ",");
        }

        col_vals = col_vals[0..col_vals.len() - 1].to_string();
        insert_cmd = insert_cmd.replace(":col_vals", &col_vals);

        println!("{:?}", insert_cmd);

        let row_id_val = row.get_ref_unwrap(total_cols).as_i64().unwrap().to_string();

        insert_cmd = insert_cmd.replace(":rid", &row_id_val);
        insert_cmd = insert_cmd.replace(":action", action);
        insert_cmd = insert_cmd.replace(":ts_utc", &Utc::now().to_string());

        println!("{:?}", insert_cmd);

        execute_write(conn, &insert_cmd);
    }

    return true;
}
