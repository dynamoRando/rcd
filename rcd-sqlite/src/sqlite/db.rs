use self::participant::create_participant_table;

use super::{
    execute_read_on_connection, get_db_conn, get_scalar_as_string, get_schema_of_table, has_table,
    sql_text, DbiConfigSqlite,
};
use crate::sqlite::has_any_rows;
use guid_create::GUID;
use rcd_common::table::*;
use rcd_enum::{
    column_type::ColumnType, database_type::DatabaseType,
    logical_storage_policy::LogicalStoragePolicy, rcd_database_type::RcdDatabaseType,
};
use rcdproto::rcdp::{ColumnSchema, DatabaseSchema, TableSchema};
use rusqlite::{named_params, Connection, Error, Result};

pub mod contract;
pub mod logical_storage_policy;
pub mod metadata;
pub mod participant;

pub fn create_database(db_name: &str, config: DbiConfigSqlite) -> Result<Connection, Error> {
    return Ok(get_db_conn(&config, db_name));
}

#[allow(dead_code)]
pub fn has_table_client_service(db_name: &str, table_name: &str, config: DbiConfigSqlite) -> bool {
    let conn = get_db_conn(&config, db_name);
    return has_table(table_name.to_string(), &conn);
}

pub fn has_cooperative_tables(db_name: &str, cmd: &str, config: &DbiConfigSqlite) -> bool {
    let mut has_cooperative_tables = false;

    let tables = rcd_query::query_parser::get_table_names(&cmd, DatabaseType::Sqlite);

    for table in tables {
        let result = logical_storage_policy::get_logical_storage_policy(db_name, &table, &config);

        if !result.is_err() {
            let policy = result.unwrap();
            match policy {
                LogicalStoragePolicy::Mirror => {
                    has_cooperative_tables = true;
                    break;
                }
                LogicalStoragePolicy::ParticpantOwned => {
                    has_cooperative_tables = true;
                    break;
                }
                LogicalStoragePolicy::Shared => {
                    has_cooperative_tables = true;
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }

    return has_cooperative_tables;
}

pub fn get_cooperative_tables(db_name: &str, cmd: &str, config: DbiConfigSqlite) -> Vec<String> {
    let mut cooperative_tables: Vec<String> = Vec::new();

    let tables = rcd_query::query_parser::get_table_names(&cmd, DatabaseType::Sqlite);

    for table in &tables {
        let result = logical_storage_policy::get_logical_storage_policy(
            db_name,
            &table.to_string(),
            &config,
        );

        if !result.is_err() {
            let policy = result.unwrap();
            match policy {
                LogicalStoragePolicy::Mirror => {
                    cooperative_tables.push(table.clone());
                }
                LogicalStoragePolicy::ParticpantOwned => {
                    cooperative_tables.push(table.clone());
                }
                LogicalStoragePolicy::Shared => {
                    cooperative_tables.push(table.clone());
                }
                _ => {}
            }
        } else {
            break;
        }
    }

    return cooperative_tables;
}

fn get_all_user_table_names_in_db(conn: &Connection) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let cmd = String::from(
        "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'COOP_%'",
    );
    let names = execute_read_on_connection(cmd, conn).unwrap();

    for row in names.rows {
        for val in row.vals {
            let name = val.data.unwrap().data_string;
            result.push(name);
        }
    }

    return result;
}

fn save_schema_to_data_host_tables(table_id: String, schema: &Table, conn: &Connection) {
    /*
    Columns:
        columnId
        name
        type
        NotNull
        defaultValue
        IsPK
     */

    let rows = &schema.rows;
    for row in rows {
        if row.vals[1].col.name == "name" {
            let col_name = &row.vals[1].data.as_ref().unwrap().data_string;

            let mut col_check = String::from(
                "SELECT 
                    COUNT(*) COUNT
                FROM 
                    COOP_DATA_HOST_TABLE_COLUMNS
                WHERE
                    COLUMN_NAME = ':col_name'
            ;",
            );

            col_check = col_check.replace(":col_name", &col_name);
            if !has_any_rows(col_check, conn) {
                // we need to add the column schema to the data host tables
                let col_id = GUID::rand();

                let mut cmd = String::from(
                    "
                    INSERT INTO COOP_DATA_HOST_TABLE_COLUMNS
                    (
                        TABLE_ID,
                        COLUMN_ID,
                        COLUMN_NAME
                    )
                    VALUES
                    (
                        ':table_id',
                        ':col_id',
                        ':col_name'
                    )
                ;",
                );

                cmd = cmd.replace(":table_id", &table_id);
                cmd = cmd.replace(":col_id", &col_id.to_string());
                cmd = cmd.replace(":col_name", col_name);
                conn.execute(&cmd, []).unwrap();
            }
        }
    }
}

/// Queries the COOP_REMOTES table for the table name and policy for each table in the database.
/// If this returns an empty vector it means either this is a new database or we haven't audited the
/// tables in the database. Generally, whenever we create a new table we should be adding the policy
/// to this table an defaulting the policy to NONE.
fn get_remote_status_for_tables(conn: &Connection) -> Vec<(String, LogicalStoragePolicy)> {
    let cmd = sql_text::Coop::text_get_logical_storage_policy_tables();
    let mut table_policies: Vec<(String, LogicalStoragePolicy)> = Vec::new();
    let mut statement = conn.prepare(&cmd).unwrap();

    let row_to_tuple =
        |table_name: String, policy: i64| -> Result<(String, LogicalStoragePolicy)> {
            Ok((table_name, LogicalStoragePolicy::from_i64(policy)))
        };

    let statuses = statement
        .query_and_then([], |row| {
            row_to_tuple(row.get(0).unwrap(), row.get(1).unwrap())
        })
        .unwrap();

    for status in statuses {
        table_policies.push(status.unwrap());
    }

    table_policies
}

/// Checks the COOP_DATA_HOST table to see if a database id has been generated and if not, creates and saves one.
/// This is the id we will use to identify this database as having cooperative tables to participants
fn populate_database_id(db_name: &str, conn: &Connection) {
    let cmd = sql_text::Coop::text_get_count_from_data_host();
    let has_database_id = has_any_rows(cmd, conn);

    if !has_database_id {
        let cmd = sql_text::Coop::text_add_database_id_to_host();
        let db_id = GUID::rand();
        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {":database_id": db_id.to_string(), ":database_name" : db_name})
            .unwrap();
    }
}

/// Populates the COOP_DATA_HOST_* tables with the needed information such as database_id and
/// the current database schema, if applicable.
fn populate_data_host_tables(db_name: &str, conn: &Connection) {
    populate_database_id(db_name, conn);
    let table_statuses = get_remote_status_for_tables(conn);

    for status in table_statuses {
        // for each table that we have a logical storage policy
        // we want to make sure that the contract tables (COOP_DATA_HOST_*)
        // have the latest correct schema for each table. Note that
        // we add tables even if the logical storage policy is NONE, because in rcd
        // we want to be transparent about all the tables in the database

        let table_name = &status.0;
        let table_id = GUID::rand();

        let statement = sql_text::Coop::text_get_count_from_data_host_tables_for_table(table_name);
        if !has_any_rows(statement, conn) {
            let cmd = sql_text::Coop::text_add_table_to_data_host_table(
                table_name.to_string(),
                table_id.to_string(),
            );
            let mut statement = conn.prepare(&cmd).unwrap();
            statement.execute([]).unwrap();
        }

        // need to get schema and save it to the table
        let schema = get_schema_of_table(table_name.to_string(), conn);
        save_schema_to_data_host_tables(table_id.to_string(), &schema.unwrap(), conn);
    }
}

/// Creates the COOP_DATA_HOST_* tables if they do not exist in the current database. These tables are used
/// to store schema information and the database_id that we send to participants of this database. This
/// data is usually contained at the participant in the database contract.
fn create_data_host_tables(conn: &Connection) {
    let mut cmd = sql_text::Coop::text_create_data_host_table();
    conn.execute(&cmd, []).unwrap();
    cmd = sql_text::Coop::text_create_data_host_tables_table();
    conn.execute(&cmd, []).unwrap();
    cmd = sql_text::Coop::text_create_data_host_tables_columns_table();
    conn.execute(&cmd, []).unwrap();
    cmd = sql_text::Coop::text_create_data_remotes_table();
    conn.execute(&cmd, []).unwrap();
}

/// Creates the COOP_REMOTES table if it does not exist. This holds
/// the logical storage policy for every table in the database.
fn create_remotes_table(conn: &Connection) {
    let cmd = String::from(
        "CREATE TABLE IF NOT EXISTS COOP_REMOTES
    (
        TABLENAME VARCHAR(255) NOT NULL,
        LOGICAL_STORAGE_POLICY INT NOT NULL
    );",
    );

    conn.execute(&cmd, []).unwrap();
}

pub fn get_db_schema(db_name: &str, config: DbiConfigSqlite) -> DatabaseSchema {
    println!("get_db_schema");
    println!("{:?}", db_name);

    let conn = &get_db_conn(&config, db_name);

    println!("{:?}", conn);

    // if this is a host db
    if has_table("COOP_DATA_HOST".to_string(), conn) {
        println!("get_db_schema for host_db");
        let mut cmd = String::from("SELECT DATABASE_ID FROM COOP_DATA_HOST");
        let db_id = get_scalar_as_string(cmd, conn);

        let mut db_schema = DatabaseSchema {
            database_id: db_id.clone(),
            database_name: db_name.to_string(),
            tables: Vec::new(),
            database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
            rcd_database_type: RcdDatabaseType::to_u32(RcdDatabaseType::Host),
        };

        cmd = String::from("SELECT TABLE_ID, TABLE_NAME FROM COOP_DATA_TABLES");

        let row_to_tuple = |table_id: String, table_name: String| -> Result<(String, String)> {
            Ok((table_id, table_name))
        };

        let mut tables_in_db: Vec<(String, String)> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();

        let tables = statement
            .query_and_then([], |row| {
                row_to_tuple(row.get(0).unwrap(), row.get(1).unwrap())
            })
            .unwrap();

        for table in tables {
            tables_in_db.push(table.unwrap());
        }

        // println!("tables_in_db: {:?}", tables_in_db);

        for t in &tables_in_db {
            let policy =
                logical_storage_policy::get_logical_storage_policy(db_name, &t.1, &config).unwrap();

            let mut ts = TableSchema {
                table_name: t.1.clone(),
                table_id: t.0.clone(),
                database_id: db_id.clone(),
                database_name: db_name.to_string(),
                columns: Vec::new(),
                logical_storage_policy: LogicalStoragePolicy::to_u32(policy),
            };

            let schema = get_schema_of_table(t.1.to_string(), conn);

            // # Columns:
            // 1. columnId
            // 2. name
            // 3. type
            // 4. NotNull
            // 5. defaultValue
            // 6. IsPK

            // println!("schema_of_table:{}, {:?}", t.1.to_string(), schema);

            for row in schema.unwrap().rows {
                let mut cs = ColumnSchema {
                    column_id: String::from(""),
                    column_name: String::from(""),
                    column_type: 0,
                    column_length: 0,
                    is_nullable: false,
                    ordinal: 0,
                    table_id: t.0.to_string(),
                    is_primary_key: false,
                };

                for val in row.vals {
                    if val.col.name == "columnId" {
                        let item = val.data.clone().unwrap();
                        cs.ordinal = item.data_string.parse().unwrap();
                    }

                    if val.col.name == "name" {
                        let item = val.data.clone().unwrap();
                        cs.column_name = item.data_string.parse().unwrap();
                    }

                    if val.col.name == "type" {
                        let item = val.data.clone().unwrap();
                        let ct = ColumnType::data_type_to_enum_u32(item.data_string.clone());
                        let len = ColumnType::data_type_len(item.data_string.clone());

                        cs.column_type = ct;
                        cs.column_length = len;
                    }

                    if val.col.name == "NotNull" {
                        let item = val.data.clone().unwrap();
                        cs.is_nullable = item.data_string.parse().unwrap();
                    }

                    if val.col.name == "IsPK" {
                        let item = val.data.clone().unwrap();
                        cs.is_primary_key = item.data_string.parse().unwrap();
                    }
                }

                ts.columns.push(cs);
            }

            db_schema.tables.push(ts);
        }

        // println!("db_schema: {:?}", db_schema);

        // get all remaining tables that don't have a policy defined, because we may want to set them
        let table_names = get_all_user_table_names_in_db(conn);

        let mut existing_tables: Vec<String> = Vec::new();
        for t in &tables_in_db {
            existing_tables.push(t.1.clone());
        }

        for table_name in &table_names {
            if !existing_tables.contains(table_name) {
                let mut ts = TableSchema {
                    table_name: table_name.clone(),
                    table_id: String::from(""),
                    database_id: String::from(""),
                    database_name: db_name.to_string(),
                    columns: Vec::new(),
                    logical_storage_policy: LogicalStoragePolicy::to_u32(
                        LogicalStoragePolicy::None,
                    ),
                };

                let schema = get_schema_of_table(table_name.clone().to_string(), conn);

                // # Columns:
                // 1. columnId
                // 2. name
                // 3. type
                // 4. NotNull
                // 5. defaultValue
                // 6. IsPK

                // println!("schema_of_table:{}, {:?}", t.1.to_string(), schema);

                for row in schema.unwrap().rows {
                    let mut cs = ColumnSchema {
                        column_id: String::from(""),
                        column_name: String::from(""),
                        column_type: 0,
                        column_length: 0,
                        is_nullable: false,
                        ordinal: 0,
                        table_id: String::from(""),
                        is_primary_key: false,
                    };

                    for val in row.vals {
                        println!("{:?}", val);

                        if val.col.name == "columnId" {
                            let item = val.data.clone().unwrap();
                            cs.ordinal = item.data_string.parse().unwrap();
                        }

                        if val.col.name == "name" {
                            let item = val.data.clone().unwrap();
                            cs.column_name = item.data_string.parse().unwrap();
                        }

                        if val.col.name == "type" {
                            let item = val.data.clone().unwrap();
                            let ct = ColumnType::data_type_to_enum_u32(item.data_string.clone());
                            let len = ColumnType::data_type_len(item.data_string.clone());

                            cs.column_type = ct;
                            cs.column_length = len;
                        }

                        if val.col.name == "NotNull" {
                            let item = val.data.clone().unwrap();
                            cs.is_nullable = item.data_string.parse().unwrap();
                        }

                        if val.col.name == "IsPK" {
                            let item = val.data.clone().unwrap();
                            cs.is_primary_key = item.data_string.parse().unwrap();
                        }
                    }

                    ts.columns.push(cs);
                }

                db_schema.tables.push(ts);
            }
        }

        return db_schema;
    }

    let mut db_schema = DatabaseSchema {
        database_id: String::from(""),
        database_name: db_name.to_string(),
        tables: Vec::new(),
        database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
        rcd_database_type: RcdDatabaseType::to_u32(RcdDatabaseType::Partial),
    };

    let table_names = get_all_user_table_names_in_db(conn);

    for table_name in &table_names {
        let mut ts = TableSchema {
            table_name: table_name.clone(),
            table_id: String::from(""),
            database_id: String::from(""),
            database_name: db_name.to_string(),
            columns: Vec::new(),
            logical_storage_policy: LogicalStoragePolicy::to_u32(LogicalStoragePolicy::None),
        };

        let schema = get_schema_of_table(table_name.clone().to_string(), conn);

        // # Columns:
        // 1. columnId
        // 2. name
        // 3. type
        // 4. NotNull
        // 5. defaultValue
        // 6. IsPK

        // println!("schema_of_table:{}, {:?}", t.1.to_string(), schema);

        for row in schema.unwrap().rows {
            let mut cs = ColumnSchema {
                column_id: String::from(""),
                column_name: String::from(""),
                column_type: 0,
                column_length: 0,
                is_nullable: false,
                ordinal: 0,
                table_id: String::from(""),
                is_primary_key: false,
            };

            for val in row.vals {
                println!("{:?}", val);

                if val.col.name == "columnId" {
                    let item = val.data.clone().unwrap();
                    cs.ordinal = item.data_string.parse().unwrap();
                }

                if val.col.name == "name" {
                    let item = val.data.clone().unwrap();
                    cs.column_name = item.data_string.parse().unwrap();
                }

                if val.col.name == "type" {
                    let item = val.data.clone().unwrap();
                    let ct = ColumnType::data_type_to_enum_u32(item.data_string.clone());
                    let len = ColumnType::data_type_len(item.data_string.clone());

                    cs.column_type = ct;
                    cs.column_length = len;
                }

                if val.col.name == "NotNull" {
                    let item = val.data.clone().unwrap();
                    cs.is_nullable = item.data_string.parse().unwrap();
                }

                if val.col.name == "IsPK" {
                    let item = val.data.clone().unwrap();
                    cs.is_primary_key = item.data_string.parse().unwrap();
                }
            }

            ts.columns.push(cs);
        }

        db_schema.tables.push(ts);
    }

    db_schema
}

pub fn enable_coooperative_features(db_name: &str, config: DbiConfigSqlite) {
    let conn = get_db_conn(&config, db_name);

    create_remotes_table(&conn);
    create_participant_table(&conn);
    create_coop_contracts_table(&conn);
    create_data_host_tables(&conn);
    populate_data_host_tables(db_name, &conn);
}

fn create_coop_contracts_table(conn: &Connection) {
    let cmd = String::from(
        "CREATE TABLE IF NOT EXISTS COOP_DATABASE_CONTRACT
    (
        CONTRACT_ID CHAR(36) NOT NULL,
        GENERATED_DATE_UTC DATETIME NOT NULL,
        DESCRIPTION VARCHAR(255),
        RETIRED_DATE_UTC DATETIME,
        VERSION_ID CHAR(36) NOT NULL,
        REMOTE_DELETE_BEHAVIOR INT
    );",
    );
    conn.execute(&cmd, []).unwrap();
}
