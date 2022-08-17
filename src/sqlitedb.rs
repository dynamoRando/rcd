use crate::database_contract::DatabaseContract;
use crate::database_participant::DatabaseParticipant;
use crate::rcd_enum::{ContractStatus, RcdGenerateContractError, RemoteDeleteBehavior};
#[allow(unused_imports)]
use crate::table::{Column, Data, Row, Table, Value};
#[allow(unused_imports)]
use crate::{
    defaults,
    rcd_enum::{self, LogicalStoragePolicy, RcdDbError},
    sql_text, table,
};
use crate::{query_parser, rcd_db};
use chrono::Utc;
#[allow(unused_imports)]
use guid_create::GUID;
use log::info;
use rusqlite::types::Type;
#[allow(unused_imports)]
use rusqlite::{named_params, Connection, Error, Result};
use std::path::Path;

pub fn create_database(db_name: &str, cwd: &str) -> Result<Connection, Error> {
    let db_path = Path::new(&cwd).join(&db_name);
    Connection::open(&db_path)
}

pub fn execute_write(db_name: &str, cwd: &str, cmd: &str) -> usize {
    let conn = get_connection(db_name, cwd);

    // println!("cmd: {}", cmd);

    let result = match conn.execute(&cmd, []) {
        Ok(updated) => updated,
        Err(err) => panic!("{:?}", err),
    };

    return result;
}

pub fn execute_write_on_connection(cmd: String, conn: &Connection) {
    conn.execute(&cmd, []).unwrap();
}

#[allow(unused_variables, dead_code)]
/// Attempts to generate a contract for the user database. This will first validate if all user
/// tables have a logical storage policy set. If not it will return a generate contract error.
/// If there is no existing contract, it will generate one. If there is an already existing active contract,
/// it will retire that contract and generate a new one.
pub fn generate_contract(
    db_name: &str,
    cwd: &str,
    host_name: &str,
    desc: &str,
    remote_delete_behavior: RemoteDeleteBehavior,
    rcd_db_name: &str,
) -> Result<bool, RcdGenerateContractError> {
    /*
       First, we should check to see if there is a logical storage policy
       defined on all user tables. If any are not set, then this should return
       a RcdGenerateContractError.

       We need to see if there are other database contracts.
       If there are none, then this is the first contract ever.

       If there are existing contracts, we need to find the active one
       and retire it, then generate the current one.
    */

    println!("generate contract: start");

    let rcd_db_conn = get_rcd_db(rcd_db_name, cwd);
    rcd_db::generate_and_get_host_info(host_name, &rcd_db_conn);

    let conn = get_connection(db_name, cwd);
    let policies = get_logical_storage_policy_for_all_user_tables(db_name, cwd);

    // check to see if all user tables have a logical storage policy set
    // if any don't, return an error.
    if policies.iter().any(|p| p.1 == LogicalStoragePolicy::None) {
        let mut missing_policies = String::from("policy not set for ");

        for p in policies {
            if p.1 == LogicalStoragePolicy::None {
                let message = format!("table {}, ", p.0);
                missing_policies.push_str(&message);
            }
        }

        let error = RcdGenerateContractError::NotAllTablesSet(missing_policies);
        return Err(error);
    }

    let cmd = String::from("SELECT COUNT(*) TOTALCONTRACTS FROM COOP_DATABASE_CONTRACT");
    if !has_any_rows(cmd, &conn) {
        // this is the first contract
        println!("generate contract: first_contract");
        let contract = DatabaseContract {
            contract_id: GUID::rand(),
            generated_date: Utc::now(),
            description: desc.to_string(),
            retired_date: None,
            version_id: GUID::rand(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };
        contract.save(&conn);
    } else {
        // there are other contracts, we need to find the active one and retire it
        // then generate a new contract
        let contracts = get_all_database_contracts(&conn);
        println!("generate contract: retire contracts");
        println!(
            "generate contract: retire contracts count: {}",
            contracts.len().to_string()
        );
        for con in contracts {
            if !con.is_retired() {
                println!(
                    "generate contract: retire contract {}",
                    &con.contract_id.to_string()
                );
                con.retire(&conn);
                println!(
                    "generate contract: save retired contract {}",
                    &con.contract_id.to_string()
                );
                con.save(&conn);
            }
        }

        println!("generate contract: retired. create new contract");
        let new_contract = DatabaseContract {
            contract_id: GUID::rand(),
            generated_date: Utc::now(),
            description: desc.to_string(),
            retired_date: None,
            version_id: GUID::rand(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };
        new_contract.save(&conn);
    }
    Ok(true)
}

pub fn execute_read_on_connection(cmd: String, conn: &Connection) -> Result<Table> {
    let mut statement = conn.prepare(&cmd).unwrap();
    let total_columns = statement.column_count();
    let cols = statement.columns();
    let mut table = Table::new();

    for col in cols {
        let col_idx = statement.column_index(col.name()).unwrap();
        let empty_string = String::from("");
        let col_type = match col.decl_type() {
            Some(c) => c,
            None => &empty_string,
        };

        let c = Column {
            name: col.name().to_string(),
            is_nullable: false,
            idx: col_idx,
            data_type: col_type.to_string(),
            is_primary_key: false,
        };

        info!("adding col {}", c.name);

        table.add_column(c);
    }

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let mut data_row = Row::new();

        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from(""),
            };

            let string_value = string_value;
            let col = table.get_column_by_index(i).unwrap();

            let data_item = Data {
                data_string: string_value,
                data_byte: Vec::new(),
            };

            let data_value = Value {
                data: Some(data_item),
                col: col,
            };

            data_row.add_value(data_value);
        }

        table.add_row(data_row);
    }

    return Ok(table);
}

#[allow(dead_code)]
pub fn has_table_client_service(db_name: &str, cwd: &str, table_name: &str) -> bool {
    let conn = get_connection(db_name, cwd);
    return has_table(table_name.to_string(), &conn);
}

#[allow(dead_code, unused_variables)]
pub fn has_cooperative_tables_mock(db_name: &str, cwd: &str, cmd: &str) -> bool {
    return false;
}

#[allow(dead_code, unused_variables)]
pub fn get_participants_for_table(
    db_name: &str,
    cwd: &str,
    table_name: &str,
) -> Vec<DatabaseParticipant> {
    unimplemented!();

    // note - we will need another table to track the remote row id

    /*
    internal const string CREATE_SHADOW_TABLE = $@"
       CREATE TABLE IF NOT EXISTS {TableNames.COOP.SHADOWS}
       (
           PARTICIPANT_ID CHAR(36) NOT NULL,
           IS_PARTICIPANT_DELETED INT,
           PARTICIPANT_DELETE_DATE_UTC DATETIME,
           DATA_HASH_LENGTH INT,
           DATA_HASH BLOB
       );
       ";
    */
}

#[allow(dead_code)]
pub fn get_cooperative_tables(db_name: &str, cwd: &str, cmd: &str) -> Vec<String> {
    let mut cooperative_tables: Vec<String> = Vec::new();

    let tables = query_parser::get_table_names(&cmd);

    for table in &tables {
        let result = get_logical_storage_policy(db_name, cwd, table.to_string());

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

#[allow(dead_code, unused_variables)]
pub fn has_cooperative_tables(db_name: &str, cwd: &str, cmd: &str) -> bool {
    let mut has_cooperative_tables = false;

    let tables = query_parser::get_table_names(&cmd);

    for table in tables {
        let result = get_logical_storage_policy(db_name, cwd, table);

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

#[allow(dead_code)]
pub fn execute_read(db_name: &str, cwd: &str, cmd: &str) -> Result<Table> {
    let conn = get_connection(db_name, cwd);
    let mut statement = conn.prepare(cmd).unwrap();
    let total_columns = statement.column_count();
    let cols = statement.columns();
    let mut table = Table::new();

    for col in cols {
        let col_idx = statement.column_index(col.name()).unwrap();

        let c = Column {
            name: col.name().to_string(),
            is_nullable: false,
            idx: col_idx,
            data_type: col.decl_type().unwrap().to_string(),
            is_primary_key: false,
        };

        info!("adding col {}", c.name);

        table.add_column(c);
    }

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let mut data_row = Row::new();

        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from(""),
            };

            let string_value = string_value;
            let col = table.get_column_by_index(i).unwrap();

            let data_item = Data {
                data_string: string_value,
                data_byte: Vec::new(),
            };

            let data_value = Value {
                data: Some(data_item),
                col: col,
            };

            data_row.add_value(data_value);
        }

        table.add_row(data_row);
    }

    return Ok(table);
}

#[allow(unused_variables, unused_mut, unused_assignments, dead_code)]
pub fn get_active_contract(db_name: &str, cwd: &str) -> DatabaseContract {
    let conn = &get_connection(db_name, cwd);
    return DatabaseContract::get_active_contract(conn);
}

#[allow(unused_variables, unused_mut, unused_assignments, dead_code)]
pub fn get_participant_by_alias(db_name: &str, cwd: &str, alias: &str) -> DatabaseParticipant {
    let conn = get_connection(db_name, cwd);
    return DatabaseParticipant::get(alias, &conn);
}

#[allow(unused_variables, unused_mut, unused_assignments, dead_code)]
pub fn has_participant(db_name: &str, cwd: &str, alias: &str) -> bool {
    let conn = &get_connection(db_name, cwd);
    return DatabaseParticipant::exists(alias, conn);
}

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn add_participant(db_name: &str, cwd: &str, alias: &str, ip4addr: &str, db_port: u32) -> bool {
    let conn = get_connection(db_name, cwd);
    let mut is_added = false;

    if DatabaseParticipant::exists(&alias, &conn) {
        is_added = false;
    } else {
        let participant = DatabaseParticipant {
            internal_id: GUID::rand(),
            alias: alias.to_string(),
            ip4addr: ip4addr.to_string(),
            ip6addr: String::from(""),
            db_port: db_port,
            contract_status: ContractStatus::NotSent,
            accepted_contract_version: GUID::parse(defaults::EMPTY_GUID).unwrap(),
            id: GUID::parse(defaults::EMPTY_GUID).unwrap(),
            token: Vec::new(),
        };
        participant.save(&conn);
        is_added = true;
    }

    return is_added;
}

#[allow(unused_variables)]
pub fn enable_coooperative_features(db_name: &str, cwd: &str) {
    let conn = get_connection(db_name, cwd);

    create_remotes_table(&conn);
    create_participant_table(&conn);
    create_contracts_table(&conn);
    create_data_host_tables(&conn);
    populate_data_host_tables(db_name, &conn);
}

#[allow(dead_code, unused_variables, unused_assignments)]
/// Returns the logical storage policy for the specified table. If the table does not exist in the database, it will
/// return an error. If the table exist but does not have a logical storage policy defined for it, it will default
/// to `LogicalStoragePolicy::None`
pub fn get_logical_storage_policy(
    db_name: &str,
    cwd: &str,
    table_name: String,
) -> Result<LogicalStoragePolicy, RcdDbError> {
    let conn = get_connection(db_name, cwd);
    let mut policy = LogicalStoragePolicy::None;

    if has_table(table_name.clone(), &conn) {
        // insert or update on the coop tables
        let mut cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
        );
        cmd = cmd.replace(":table_name", &table_name.clone());
        if has_any_rows(cmd, &conn) {
            // then we have a record for the policy of the table
            let mut cmd = String::from(
                "SELECT LOGICAL_STORAGE_POLICY FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
            );

            cmd = cmd.replace(":table_name", &table_name);
            let i_policy = get_scalar_as_u32(cmd.clone(), &conn);
            policy = LogicalStoragePolicy::from_i64(i_policy as i64);
        } else {
            /*
                let error_message = format!(
                    "logical storage policy not saved in COOP_REMOTES for table {} in db {}",
                    table_name, db_name
                );
                let err = RcdDbError::LogicalStoragePolicyNotSet(error_message);
                return Err(err);
            */
            return Ok(LogicalStoragePolicy::None);
        }
    } else {
        let error_message = format!("table {} not found in db {}", table_name, db_name);
        let err = RcdDbError::TableNotFound(error_message);
        return Err(err);
    }

    return Ok(policy);
}

#[allow(dead_code, unused_variables, unused_assignments)]
pub fn set_logical_storage_policy(
    db_name: &str,
    cwd: &str,
    table_name: String,
    policy: LogicalStoragePolicy,
) -> Result<bool, RcdDbError> {
    let conn = get_connection(db_name, cwd);

    if has_table(table_name.clone(), &conn) {
        // insert or update on the coop tables
        let mut cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
        );
        cmd = cmd.replace(":table_name", &table_name.clone());
        if has_any_rows(cmd, &conn) {
            // then this is an update
            let mut cmd = String::from(
                "UPDATE COOP_REMOTES
            SET LOGICAL_STORAGE_POLICY = :policy
            WHERE TABLENAME = ':table_name';
            ",
            );

            cmd = cmd.replace(":table_name", &table_name);
            cmd = cmd.replace(":policy", &LogicalStoragePolicy::to_u32(policy).to_string());
            execute_write(db_name, cwd, &cmd);
        } else {
            // then this is an insert
            let mut cmd = String::from(
                "INSERT INTO COOP_REMOTES
            (
                TABLENAME,
                LOGICAL_STORAGE_POLICY  
            )
            VALUES
            (
                ':table_name',
                :policy
            );",
            );

            cmd = cmd.replace(":table_name", &table_name);
            cmd = cmd.replace(":policy", &LogicalStoragePolicy::to_u32(policy).to_string());
            execute_write(db_name, cwd, &cmd);
        }
    } else {
        let error_message = format!("table {} not in {}", table_name, db_name);
        let err = RcdDbError::TableNotFound(error_message);
        return Err(err);
    }
    return Ok(true);
}

#[allow(dead_code, unused_variables)]
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

#[allow(dead_code, unused_variables)]
/// Creates the COOP_PARTICIPANT table if it does not exist. This holds
/// the participant information that are cooperating with this database.
fn create_participant_table(conn: &Connection) {
    let cmd = String::from(
        "CREATE TABLE IF NOT EXISTS COOP_PARTICIPANT
    (
        INTERNAL_PARTICIPANT_ID CHAR(36) NOT NULL,
        ALIAS VARCHAR(50) NOT NULL,
        IP4ADDRESS VARCHAR(25),
        IP6ADDRESS VARCHAR(25),
        PORT INT,
        CONTRACT_STATUS INT,
        ACCEPTED_CONTRACT_VERSION_ID CHAR(36),
        TOKEN BLOB NOT NULL,
        PARTICIPANT_ID CHAR(36)
    );",
    );

    conn.execute(&cmd, []).unwrap();
}

#[allow(dead_code, unused_variables)]
/// Creates the COOP_DATABASE_CONTRACT table if it does not exist. This holds
/// all the contracts we have generated for this database.
fn create_contracts_table(conn: &Connection) {
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

#[allow(dead_code, unused_variables)]
/// Creates the COOP_DATA_HOST_* tables if they do not exist in the current database. These tables are used
/// to store schema information and the database_id that we send to participants of this database. This
/// data is usually contained at the participant in the database contract.
fn create_data_host_tables(conn: &Connection) {
    let mut cmd = sql_text::COOP::text_create_data_host_table();
    conn.execute(&cmd, []).unwrap();
    cmd = sql_text::COOP::text_create_data_host_tables_table();
    conn.execute(&cmd, []).unwrap();
    cmd = sql_text::COOP::text_create_data_host_tables_columns_table();
    conn.execute(&cmd, []).unwrap();
    cmd = sql_text::COOP::text_create_data_remotes_table();
    conn.execute(&cmd, []).unwrap();
}

#[allow(dead_code, unused_variables)]
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

        let statement = sql_text::COOP::text_get_count_from_data_host_tables_for_table(&table_name);
        if !has_any_rows(statement, &conn) {
            let cmd = sql_text::COOP::text_add_table_to_data_host_table(
                table_name.to_string(),
                table_id.to_string(),
            );
            let mut statement = conn.prepare(&cmd).unwrap();
            statement.execute([]).unwrap();
        }

        // need to get schema and save it to the table
        let schema = get_schema_of_table(table_name.to_string(), &conn);
        save_schema_to_data_host_tables(table_id.to_string(), &schema, &conn);
    }
}

#[allow(dead_code, unused_variables)]
/// Checks the COOP_DATA_HOST table to see if a database id has been generated and if not, creates and saves one.
/// This is the id we will use to identify this database as having cooperative tables to participants
fn populate_database_id(db_name: &str, conn: &Connection) {
    let cmd = sql_text::COOP::text_get_count_from_data_host();
    let has_database_id = has_any_rows(cmd, conn);

    if !has_database_id {
        let cmd = sql_text::COOP::text_add_database_id_to_host();
        let db_id = GUID::rand();
        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {":database_id": db_id.to_string(), ":database_name" : db_name})
            .unwrap();
    }
}

#[allow(dead_code, unused_variables)]
/// Takes a SELECT COUNT(*) SQL statement and returns if the result is > 0. Usually used to see if a table that has been
/// created has also populated any data in it.
pub fn has_any_rows(cmd: String, conn: &Connection) -> bool {
    return total_count(cmd, conn) > 0;
}

#[allow(dead_code, unused_variables)]
/// Runs any SQL statement that returns a single value and attempts
/// to return the result as a u32
fn get_scalar_as_u32(cmd: String, conn: &Connection) -> u32 {
    let mut value: u32 = 0;
    let mut statement = conn.prepare(&cmd).unwrap();
    let rows = statement.query_map([], |row| row.get(0)).unwrap();

    for item in rows {
        value = item.unwrap();
    }

    return value;
}

#[allow(dead_code, unused_variables)]
/// Takes a SELECT COUNT(*) SQL statement and returns the value
fn total_count(cmd: String, conn: &Connection) -> u32 {
    return get_scalar_as_u32(cmd, conn);
}

#[allow(dead_code, unused_variables)]
/// Queries the COOP_REMOTES table for the table name and policy for each table in the database.
/// If this returns an empty vector it means either this is a new database or we haven't audited the
/// tables in the database. Generally, whenever we create a new table we should be adding the policy
/// to this table an defaulting the policy to NONE.
fn get_remote_status_for_tables(conn: &Connection) -> Vec<(String, LogicalStoragePolicy)> {
    let cmd = sql_text::COOP::text_get_logical_storage_policy_tables();
    let mut table_policies: Vec<(String, rcd_enum::LogicalStoragePolicy)> = Vec::new();
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

    return table_policies;
}

#[allow(dead_code, unused_variables, unused_assignments)]
/// Returns a table describing the schema of the table
/// # Columns:
/// 1. columnId
/// 2. name
/// 3. type
/// 4. NotNull
/// 5. defaultValue
/// 6. IsPK
fn get_schema_of_table(table_name: String, conn: &Connection) -> Table {
    let mut cmd = String::from("PRAGMA table_info(\"{:table_name}\")");
    cmd = cmd.replace(":table_name", &table_name);

    return execute_read_on_connection(cmd, conn).unwrap();
}

#[allow(dead_code, unused_variables, unused_assignments)]
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
                    COOP_DATA_COLUMNS
                WHERE
                    COLUMN_NAME = :col_name
            ;",
            );

            col_check = col_check.replace("col_name", &col_name);
            if !has_any_rows(col_check, conn) {
                // we need to add the column schema to the data host tables
                let col_id = GUID::rand();

                let mut cmd = String::from(
                    "
                    INSERT INTO COOP_DATA_COLUMNS
                    (
                        TABLE_ID,
                        COLUMN_ID,
                        COLUMN_NAME
                    )
                    VALUES
                    (
                        :table_id,
                        :col_id,
                        :col_name
                    )
                ;",
                );

                cmd = cmd.replace(":table_id", &table_id);
                cmd = cmd.replace(":col_id", &col_id.to_string());
                cmd = cmd.replace(":col_name", &col_name);
                conn.execute(&cmd, []).unwrap();
            }
        }
    }
}

#[allow(unused_variables, dead_code)]
fn has_table(table_name: String, conn: &Connection) -> bool {
    let mut cmd = String::from(
        "SELECT count(*) AS TABLECOUNT FROM sqlite_master WHERE type='table' AND name=':table_name'",
    );
    cmd = cmd.replace(":table_name", &table_name);
    return has_any_rows(cmd, conn);
}

#[allow(unused_variables, dead_code)]
/// Returns a vector of tuples representing the name of the user table and the logical storage policy
/// attached to it.
fn get_logical_storage_policy_for_all_user_tables(
    db_name: &str,
    cwd: &str,
) -> Vec<(String, LogicalStoragePolicy)> {
    let conn = get_connection(db_name, cwd);

    let mut result: Vec<(String, LogicalStoragePolicy)> = Vec::new();

    let table_names = get_all_user_table_names_in_db(&conn);

    for table_name in &table_names {
        let l_policy = get_logical_storage_policy(db_name, cwd, table_name.to_string()).unwrap();
        let item = (table_name.to_string(), l_policy);
        result.push(item);
    }

    return result;
}

#[allow(unused_variables, dead_code)]
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

#[allow(unused_variables, dead_code, unused_assignments)]
fn get_all_database_contracts(conn: &Connection) -> Vec<DatabaseContract> {
    return DatabaseContract::get_all(conn);
}

pub fn get_connection(db_name: &str, cwd: &str) -> Connection {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path).unwrap();
    return conn;
}

fn get_rcd_db(db_name: &str, cwd: &str) -> Connection {
    let db_path = Path::new(cwd).join(db_name);
    return Connection::open(&db_path).unwrap();
}
