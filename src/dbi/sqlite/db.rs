use super::{
    execute_read_on_connection, execute_write, execute_write_on_connection, get_db_conn,
    get_scalar_as_string, get_scalar_as_u32, has_table, sql_text, DbiConfigSqlite,
};
use crate::{
    cdata::{ColumnSchema, DatabaseSchema, Participant, TableSchema},
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::{CoopDatabaseParticipant, CoopDatabaseParticipantData},
    dbi::sqlite::has_any_rows,
    defaults, query_parser,
    rcd_enum::{
        self, ColumnType, ContractStatus, DatabaseType, LogicalStoragePolicy, RcdDbError,
        RcdGenerateContractError, RemoteDeleteBehavior,
    },
    table::Table,
};
use chrono::{TimeZone, Utc};
use guid_create::GUID;

use rusqlite::{named_params, Connection, Error, Result};

pub fn insert_metadata_into_host_db(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    hash: u64,
    internal_participant_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);

    if !has_table(metadata_table_name.clone(), &conn) {
        //  need to create table
        let mut cmd = sql_text::COOP::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name.clone());
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::COOP::text_insert_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name.clone());
    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement
        .execute(named_params! {":row": row_id, ":hash" : hash.to_ne_bytes(), ":pid" : internal_participant_id })
        .unwrap();

    return rows > 0;
}

pub fn update_participant_accepts_contract(
    db_name: &str,
    participant: CoopDatabaseParticipant,
    participant_message: Participant,
    accepted_contract_version_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);

    let internal_id = participant.internal_id.clone();
    let participant_id = participant_message.participant_guid.clone();
    let token = participant_message.token.clone();

    let cmd = String::from(
        "
    UPDATE 
        COOP_PARTICIPANT
    SET 
        CONTRACT_STATUS = 3, 
        ACCEPTED_CONTRACT_VERSION_ID = :cid,
        PARTICIPANT_ID = :pid,
        TOKEN = :token
    WHERE 
        INTERNAL_PARTICIPANT_ID = :iid
    ;
    ",
    );

    let mut statement = conn.prepare(&cmd).unwrap();

    let rows_affected = statement
        .execute(named_params! {
            ":cid" : accepted_contract_version_id.to_string(),
            ":pid" : participant_id,
            ":token" : token,
            ":iid" : internal_id.to_string(),
        })
        .unwrap();

    return rows_affected > 0;
}

pub fn create_database(db_name: &str, config: DbiConfigSqlite) -> Result<Connection, Error> {
    return Ok(get_db_conn(&config, db_name));
}

/// Attempts to generate a contract for the user database. This will first validate if all user
/// tables have a logical storage policy set. If not it will return a generate contract error.
/// If there is no existing contract, it will generate one. If there is an already existing active contract,
/// it will retire that contract and generate a new one.
pub fn generate_contract(
    db_name: &str,
    desc: &str,
    remote_delete_behavior: RemoteDeleteBehavior,
    config: DbiConfigSqlite,
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

    // println!("generate contract: start for {}", db_name);

    let conn = &get_db_conn(&config, db_name);
    let policies = get_logical_storage_policy_for_all_user_tables(db_name, config);

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
        // println!("generate contract: first_contract");
        let contract = CoopDatabaseContract {
            contract_id: GUID::rand(),
            generated_date: Utc::now(),
            description: desc.to_string(),
            retired_date: None,
            version_id: GUID::rand(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };
        save_contract_at_connection(contract, conn);
    } else {
        // there are other contracts, we need to find the active one and retire it
        // then generate a new contract
        let contracts = get_all_database_contracts(&conn);
        // println!("generate contract: retire contracts");
        // println!(
        //     "generate contract: retire contracts count: {}",
        //     contracts.len().to_string()
        //  );
        for con in contracts {
            if !con.is_retired() {
                println!(
                    "generate contract: retire contract {}",
                    &con.contract_id.to_string()
                );
                retire_contract(con.version_id, &conn);
                println!(
                    "generate contract: save retired contract {}",
                    &con.contract_id.to_string()
                );
                save_contract_at_connection(con, conn);
            }
        }

        println!("generate contract: retired. create new contract");
        let new_contract = CoopDatabaseContract {
            contract_id: GUID::rand(),
            generated_date: Utc::now(),
            description: desc.to_string(),
            retired_date: None,
            version_id: GUID::rand(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };
        save_contract_at_connection(new_contract, &conn);
    }
    Ok(true)
}

fn save_contract_at_connection(contract: CoopDatabaseContract, conn: &Connection) {
    let mut cmd = String::from(
        "SELECT COUNT(*) TOTALCOUNT FROM COOP_DATABASE_CONTRACT WHERE VERSION_ID = ':vid'",
    );
    cmd = cmd.replace(":vid", &contract.version_id.to_string());
    if has_any_rows(cmd, &conn) {
        // this is an update
        if contract.is_retired() {
            let mut cmd = String::from(
                "
            UPDATE COOP_DATABASE_CONTRACT 
            SET 
                CONTRACT_ID = ':cid',
                GENERATED_DATE_UTC = ':gen_date',
                DESCRIPTION = ':desc',
                RETIRED_DATE_UTC = ':ret_date',
                REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
            WHERE
                VERSION_ID = ':vid'",
            );
            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            let ret = &contract.retired_date.unwrap().to_string();
            cmd = cmd.replace(":ret_date", ret);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(&conn, &cmd);
        } else {
            let mut cmd = String::from(
                "
            UPDATE COOP_DATABASE_CONTRACT 
            SET 
                CONTRACT_ID = ':cid',
                GENERATED_DATE_UTC = ':gen_date',
                DESCRIPTION = ':desc',
                REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
            WHERE
                VERSION_ID = ':vid'",
            );
            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(&conn, &cmd);
        }
    } else {
        // this is an insert
        if contract.is_retired() {
            let mut cmd = String::from(
                "
            INSERT INTO COOP_DATABASE_CONTRACT
            (
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                RETIRED_DATE_UTC,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR
            )
            VALUES
            (
                ':cid',
                ':gen_date',
                ':desc',
                ':ret_date',
                ':vid',
                ':remote_behavior'
            );
            ",
            );
            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            let ret = &contract.retired_date.unwrap().to_string();
            cmd = cmd.replace(":ret_date", ret);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(&conn, &cmd);
        } else {
            let mut cmd = String::from(
                "
            INSERT INTO COOP_DATABASE_CONTRACT
            (
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR
            )
            VALUES
            (
                ':cid',
                ':gen_date',
                ':desc',
                ':vid',
                ':remote_behavior'
            );
            ",
            );

            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            println!("{}", &contract.generated_date);
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(&conn, &cmd);
        }
    }
}

fn get_all_database_contracts(conn: &Connection) -> Vec<CoopDatabaseContract> {
    let mut result: Vec<CoopDatabaseContract> = Vec::new();

    /*
        "CREATE TABLE IF NOT EXISTS COOP_DATABASE_CONTRACT
        (
            CONTRACT_ID CHAR(36) NOT NULL,
            GENERATED_DATE_UTC DATETIME NOT NULL,
            DESCRIPTION VARCHAR(255),
            RETIRED_DATE_UTC DATETIME,
            VERSION_ID CHAR(36) NOT NULL,
            REMOTE_DELETE_BEHAVIOR INT
        );",
    */

    let cmd = String::from(
        "SELECT 
        CONTRACT_ID,
        GENERATED_DATE_UTC,
        DESCRIPTION,
        RETIRED_DATE_UTC,
        VERSION_ID,
        REMOTE_DELETE_BEHAVIOR
    FROM
        COOP_DATABASE_CONTRACT
        ;
        ",
    );

    let table = execute_read_on_connection(cmd, conn).unwrap();

    for row in table.rows {
        for val in &row.vals {
            let mut cid = GUID::rand();
            let mut gen_date = Utc::now();
            let mut desc = String::from("");
            let mut is_retired = false;
            let mut ret_date = Utc::now();
            let mut vid = GUID::rand();
            let mut delete_behavior: u32 = 0;

            if val.col.name == "CONTRACT_ID" {
                let vcid = val.data.as_ref().unwrap().data_string.clone();
                let tcid = GUID::parse(&vcid).unwrap();
                cid = tcid;
            }

            if val.col.name == "GENERATED_DATE_UTC" {
                let vgen_date = val.data.as_ref().unwrap().data_string.clone();
                // println!("{}", vgen_date);
                let tgen_date =
                    Utc::datetime_from_str(&Utc, &vgen_date, defaults::DATETIME_STRING_FORMAT);
                gen_date = tgen_date.unwrap();
            }

            if val.col.name == "DESCRIPTION" {
                let vdesc = val.data.as_ref().unwrap().data_string.clone();
                desc = vdesc.clone();
            }

            if val.col.name == "RETIRED_DATE_UTC" {
                if val.is_null() {
                    is_retired = false;
                } else {
                    let vret_date = val.data.as_ref().unwrap().data_string.clone();
                    if vret_date.len() > 0 {
                        // println!("{}", vret_date);
                        let tret_date = Utc::datetime_from_str(
                            &Utc,
                            &vret_date,
                            defaults::DATETIME_STRING_FORMAT,
                        );
                        ret_date = tret_date.unwrap();
                        is_retired = true;
                    } else {
                        is_retired = false;
                    }
                }
            }

            if val.col.name == "VERSION_ID" {
                let vvid = val.data.as_ref().unwrap().data_string.clone();
                let tvid = GUID::parse(&vvid).unwrap();
                vid = tvid;
            }

            if val.col.name == "REMOTE_DELETE_BEHAVIOR" {
                let vbehavior = val.data.as_ref().unwrap().data_string.clone();
                delete_behavior = vbehavior.parse().unwrap();
            }

            let item = CoopDatabaseContract {
                contract_id: cid,
                generated_date: gen_date,
                description: desc,
                retired_date: if is_retired { Some(ret_date) } else { None },
                version_id: vid,
                remote_delete_behavior: delete_behavior,
            };

            result.push(item);
        }
    }

    return result;
}

/// Marks this contract as retired in the database with today's UTC date
pub fn retire_contract(version_id: GUID, conn: &Connection) {
    let mut cmd = String::from("UPDATE COOP_DATABASE_CONTRACT SET RETIRED_DATE_UTC = ':retire_date' WHERE VERSION_ID = ':vid'");
    cmd = cmd.replace(":retire_date", &Utc::now().to_string());
    cmd = cmd.replace(":vid", &version_id.to_string());
    execute_write(conn, &cmd);
}

/// Returns a vector of tuples representing the name of the user table and the logical storage policy
/// attached to it.
fn get_logical_storage_policy_for_all_user_tables(
    db_name: &str,
    config: DbiConfigSqlite,
) -> Vec<(String, LogicalStoragePolicy)> {
    let conn = get_db_conn(&config, db_name);

    let mut result: Vec<(String, LogicalStoragePolicy)> = Vec::new();

    let table_names = get_all_user_table_names_in_db(&conn);

    for table_name in &table_names {
        let l_policy =
            get_logical_storage_policy(db_name, &table_name.to_string(), &config).unwrap();
        let item = (table_name.to_string(), l_policy);
        result.push(item);
    }

    return result;
}

#[allow(dead_code)]
pub fn has_table_client_service(db_name: &str, table_name: &str, config: DbiConfigSqlite) -> bool {
    let conn = get_db_conn(&config, db_name);
    return has_table(table_name.to_string(), &conn);
}

#[allow(dead_code, unused_variables, unused_assignments)]
pub fn get_participants_for_table(
    db_name: &str,
    table_name: &str,
    config: DbiConfigSqlite,
) -> Vec<CoopDatabaseParticipantData> {
    // note - we will need another table to track the remote row id
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
    let conn = get_db_conn(&config, db_name);

    let mut result: Vec<CoopDatabaseParticipantData> = Vec::new();

    let mut cmd = String::from(
        "
        SELECT DISTINCT 
            INTERNAL_PARTICIPANT_ID 
        FROM 
            :table_name
        ;",
    );
    cmd = cmd.replace(":table_name", &metadata_table_name);

    let mut statement = conn.prepare(&cmd).unwrap();
    let mut participant_ids: Vec<String> = Vec::new();
    let mut db_participants: Vec<CoopDatabaseParticipant> = Vec::new();

    let row_to_id = |participant_id: String| -> Result<String> { Ok(participant_id) };

    let participants = statement
        .query_and_then([], |row| row_to_id(row.get(0).unwrap()))
        .unwrap();

    for p in participants {
        participant_ids.push(p.unwrap());
    }

    for pid in &participant_ids {
        let participant = get_participant_by_internal_id(db_name, pid, &config);
        db_participants.push(participant);
    }

    let row_to_data = |row_id: u32, hash: Vec<u8>| -> Result<(u32, Vec<u8>)> { Ok((row_id, hash)) };

    for p in &db_participants {
        cmd = String::from(
            "
            SELECT 
                ROW_ID, 
                HASH
            FROM 
                :table_name
            WHERE
                INTERNAL_PARTICIPANT_ID = ':pid'
            ;",
        );
        cmd = cmd.replace(":table_name", &metadata_table_name);
        cmd = cmd.replace(":pid", &p.internal_id.to_string());

        statement = conn.prepare(&cmd).unwrap();

        let row_data = statement
            .query_and_then([], |row| {
                row_to_data(row.get(0).unwrap(), row.get(1).unwrap())
            })
            .unwrap();

        let mut row_data_results: Vec<(u32, Vec<u8>)> = Vec::new();

        for data in row_data {
            row_data_results.push(data.unwrap());
        }

        let participant_data = CoopDatabaseParticipantData {
            participant: p.clone(),
            db_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_data: row_data_results,
        };

        result.push(participant_data);
    }

    return result;
}

pub fn has_cooperative_tables(db_name: &str, cmd: &str, config: &DbiConfigSqlite) -> bool {
    let mut has_cooperative_tables = false;

    let tables = query_parser::get_table_names(&cmd, DatabaseType::Sqlite);

    for table in tables {
        let result = get_logical_storage_policy(db_name, &table, &config);

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

    let tables = query_parser::get_table_names(&cmd, DatabaseType::Sqlite);

    for table in &tables {
        let result = get_logical_storage_policy(db_name, &table.to_string(), &config);

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
                cmd = cmd.replace(":col_name", &col_name);
                conn.execute(&cmd, []).unwrap();
            }
        }
    }
}

/// Returns a table describing the schema of the table
/// # Columns:
/// 1. columnId
/// 2. name
/// 3. type
/// 4. NotNull
/// 5. defaultValue
/// 6. IsPK
fn get_schema_of_table(table_name: String, conn: &Connection) -> Result<Table> {
    let mut cmd = String::from("PRAGMA table_info(\":table_name\")");
    cmd = cmd.replace(":table_name", &table_name);

    return Ok(execute_read_on_connection(cmd, conn).unwrap());
}

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
        save_schema_to_data_host_tables(table_id.to_string(), &schema.unwrap(), &conn);
    }
}

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

pub fn set_logical_storage_policy(
    db_name: &str,
    table_name: &str,
    policy: LogicalStoragePolicy,
    config: DbiConfigSqlite,
) -> Result<bool, RcdDbError> {
    let conn = get_db_conn(&config, db_name);

    if has_table(table_name.to_string(), &conn) {
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
            execute_write_on_connection(db_name, &cmd, &config);
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
            execute_write_on_connection(db_name, &cmd, &config);
        }

        populate_data_host_tables(db_name, &conn);
    } else {
        let error_message = format!("table {} not in {}", table_name, db_name);
        let err = RcdDbError::TableNotFound(error_message);
        return Err(err);
    }
    return Ok(true);
}

/// Returns the logical storage policy for the specified table. If the table does not exist in the database, it will
/// return an error. If the table exist but does not have a logical storage policy defined for it, it will default
/// to `LogicalStoragePolicy::None`
#[allow(unused_assignments)]
pub fn get_logical_storage_policy(
    db_name: &str,
    table_name: &str,
    config: &DbiConfigSqlite,
) -> Result<LogicalStoragePolicy, RcdDbError> {
    let conn = get_db_conn(&config, db_name);
    let mut policy = LogicalStoragePolicy::None;

    if has_table(table_name.to_string(), &conn) {
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

pub fn save_participant(participant: CoopDatabaseParticipant, conn: Connection) {
    if has_participant_at_conn(&participant.alias, &conn) {
        // this is an update
        let cmd = String::from(
            "
        UPDATE COOP_PARTICIPANT
        SET
            IP4ADDRESS = ':ip4addr',
            IP6ADDRESS = ':ip6addr',
            PORT = ':db_port',
            CONTRACT_STATUS = ':contract_status',
            ACCEPTED_CONTRACT_VERSION_ID = ':accepted_contract_version',
            TOKEN = ':token',
            PARTICIPANT_ID = ':id'
        WHERE
            ALIAS = ':alias'
        ;
        ",
        );

        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {
                    ":ip4addr": participant.ip4addr,
                    ":ip6addr": participant.ip6addr,
                    ":db_port": participant.db_port.to_string(),
                    ":contract_status": &ContractStatus::to_u32(participant.contract_status),
                    ":accepted_contract_version": &participant.accepted_contract_version.to_string(),
                    ":token": &participant.token,
                    ":id": &participant.id.to_string(),
                    ":alias": &participant.alias,
            })
            .unwrap();
    } else {
        // this is an insert

        // println!("{:?}", &self);

        let cmd = String::from(
            "
        INSERT INTO COOP_PARTICIPANT
        (
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            CONTRACT_STATUS,
            ACCEPTED_CONTRACT_VERSION_ID,
            TOKEN,
            PARTICIPANT_ID
        )
        VALUES
        (
            :internal_id,
            :alias,
            :ip4addr,
            :ip6addr,
            :db_port,
            :contract_status,
            :accepted_contract_version,
            :token,
            :id
        );
        ",
        );

        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {
                    ":internal_id": &participant.internal_id.to_string(),
                    ":alias": &participant.alias,
                    ":ip4addr": &participant.ip4addr,
                    ":ip6addr": &participant.ip6addr,
                    ":db_port": &participant.db_port.to_string(),
                    ":contract_status": &ContractStatus::to_u32(participant.contract_status),
                    ":accepted_contract_version": &participant.accepted_contract_version.to_string(),
                    ":token": &participant.token,
                    ":id": &participant.id.to_string()
            })
            .unwrap();
    }
}

pub fn add_participant(
    db_name: &str,
    alias: &str,
    ip4addr: &str,
    db_port: u32,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);
    let is_added: bool;

    if has_participant(db_name, alias, config) {
        is_added = false;
    } else {
        let participant = CoopDatabaseParticipant {
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
        save_participant(participant, conn);
        is_added = true;
    }

    return is_added;
}

pub fn get_participant_by_internal_id(
    db_name: &str,
    internal_id: &str,
    config: &DbiConfigSqlite,
) -> CoopDatabaseParticipant {
    let conn = get_db_conn(&config, db_name);
    let cmd = String::from(
        "
        SELECT 
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            CONTRACT_STATUS,
            ACCEPTED_CONTRACT_VERSION_ID,
            TOKEN,
            PARTICIPANT_ID
        FROM
            COOP_PARTICIPANT
        WHERE
            INTERNAL_PARTICIPANT_ID = :pid
        ;
        ",
    );
    // cmd = cmd.replace(":alias", &alias);

    let row_to_participant = |internal_id: String,
                              alias: String,
                              ip4addr: String,
                              ip6addr: String,
                              port: u32,
                              contract_status: u32,
                              accepted_contract_version_id: String,
                              token: Vec<u8>,
                              id: String|
     -> Result<CoopDatabaseParticipant> {
        let participant = CoopDatabaseParticipant {
            internal_id: GUID::parse(&internal_id).unwrap(),
            alias: alias,
            ip4addr: ip4addr,
            ip6addr: ip6addr,
            db_port: port,
            contract_status: ContractStatus::from_i64(contract_status as i64),
            accepted_contract_version: GUID::parse(&accepted_contract_version_id).unwrap(),
            token: token,
            id: GUID::parse(&id).unwrap(),
        };

        Ok(participant)
    };

    let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

    let mut statement = conn.prepare(&cmd).unwrap();
    let participants = statement
        .query_and_then(&[(":pid", &internal_id)], |row| {
            row_to_participant(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
                row.get(8).unwrap(),
            )
        })
        .unwrap();

    for participant in participants {
        results.push(participant.unwrap());
    }

    return results.first().unwrap().clone();
}

pub fn get_participant_by_alias(
    db_name: &str,
    alias: &str,
    config: DbiConfigSqlite,
) -> CoopDatabaseParticipant {
    let conn = get_db_conn(&config, db_name);
    let cmd = String::from(
        "
        SELECT 
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            CONTRACT_STATUS,
            ACCEPTED_CONTRACT_VERSION_ID,
            TOKEN,
            PARTICIPANT_ID
        FROM
            COOP_PARTICIPANT
        WHERE
            ALIAS = :alias
        ;
        ",
    );
    // cmd = cmd.replace(":alias", &alias);

    let row_to_participant = |internal_id: String,
                              alias: String,
                              ip4addr: String,
                              ip6addr: String,
                              port: u32,
                              contract_status: u32,
                              accepted_contract_version_id: String,
                              token: Vec<u8>,
                              id: String|
     -> Result<CoopDatabaseParticipant> {
        let participant = CoopDatabaseParticipant {
            internal_id: GUID::parse(&internal_id).unwrap(),
            alias: alias,
            ip4addr: ip4addr,
            ip6addr: ip6addr,
            db_port: port,
            contract_status: ContractStatus::from_i64(contract_status as i64),
            accepted_contract_version: GUID::parse(&accepted_contract_version_id).unwrap(),
            token: token,
            id: GUID::parse(&id).unwrap(),
        };

        Ok(participant)
    };

    let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

    let mut statement = conn.prepare(&cmd).unwrap();
    let participants = statement
        .query_and_then(&[(":alias", &alias)], |row| {
            row_to_participant(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
                row.get(8).unwrap(),
            )
        })
        .unwrap();

    for participant in participants {
        results.push(participant.unwrap());
    }

    return results.first().unwrap().clone();
}

pub fn has_participant_at_conn(alias: &str, conn: &Connection) -> bool {
    let mut cmd =
        String::from("SELECT COUNT(*) TOTALCOUNT FROM COOP_PARTICIPANT WHERE ALIAS = ':alias'");
    cmd = cmd.replace(":alias", alias);
    return has_any_rows(cmd, &conn);
}

pub fn has_participant(db_name: &str, alias: &str, config: DbiConfigSqlite) -> bool {
    let conn = &get_db_conn(&config, db_name);
    let mut cmd =
        String::from("SELECT COUNT(*) TOTALCOUNT FROM COOP_PARTICIPANT WHERE ALIAS = ':alias'");
    cmd = cmd.replace(":alias", alias);
    return has_any_rows(cmd, conn);
}

pub fn get_db_schema(db_name: &str, config: DbiConfigSqlite) -> DatabaseSchema {
    let conn = &get_db_conn(&config, db_name);

    let mut cmd = String::from("SELECT DATABASE_ID FROM COOP_DATA_HOST");
    let db_id = get_scalar_as_string(cmd, conn);

    let mut db_schema = DatabaseSchema {
        database_id: db_id.clone(),
        database_name: db_name.to_string(),
        tables: Vec::new(),
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
        let policy = get_logical_storage_policy(db_name, &t.1, &config).unwrap();

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

    return db_schema;
}

pub fn get_active_contract(db_name: &str, config: DbiConfigSqlite) -> CoopDatabaseContract {
    let conn = &get_db_conn(&config, db_name);

    let cmd = String::from(
        "
        SELECT 
            CONTRACT_ID,
            GENERATED_DATE_UTC,
            DESCRIPTION,
            VERSION_ID,
            REMOTE_DELETE_BEHAVIOR 
        FROM 
            COOP_DATABASE_CONTRACT 
        WHERE 
            RETIRED_DATE_UTC IS NULL
        ;",
    );

    let row_to_active_contract = |contract_id: String,
                                  generated_date_utc: String,
                                  description: String,
                                  version_id: String,
                                  remote_delete_behavior: u32|
     -> Result<CoopDatabaseContract> {
        let contract = CoopDatabaseContract {
            contract_id: GUID::parse(&contract_id).unwrap(),
            generated_date: Utc::datetime_from_str(
                &Utc,
                &generated_date_utc,
                defaults::DATETIME_STRING_FORMAT,
            )
            .unwrap(),
            description: description,
            retired_date: None,
            version_id: GUID::parse(&version_id).unwrap(),
            remote_delete_behavior: remote_delete_behavior,
        };

        Ok(contract)
    };

    let mut results: Vec<CoopDatabaseContract> = Vec::new();

    let mut statement = conn.prepare(&cmd).unwrap();
    let contracts = statement
        .query_and_then([], |row| {
            row_to_active_contract(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
            )
        })
        .unwrap();

    for contract in contracts {
        results.push(contract.unwrap());
    }

    return results.first().unwrap().clone();
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
