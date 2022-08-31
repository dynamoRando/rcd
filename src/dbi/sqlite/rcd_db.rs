use super::{has_any_rows, sql_text::CDS};
use crate::{
    crypt,
    dbi::{sqlite::get_db_conn, DbiConfigSqlite},
    host_info::HostInfo,
    rcd_db::User, cdata::Contract,
};
use guid_create::GUID;
use log::info;
use rusqlite::{named_params, Connection, Result};
use std::path::Path;

/// Saves a contract sent from a host to our local rcd_db instance. This lets us
/// later review the contract for us to accept or reject it. If we accept it
/// this means that we'll create a partial database with the contract's schema
/// and also notify the host that we are willing to be a participant of the database.
#[allow(dead_code, unused_variables)]
pub fn save_contract(contract: Contract, config: &DbiConfigSqlite) -> bool {
    let conn = get_rcd_conn(config);

    if !has_contract(&contract.contract_guid, &conn) {
        save_contract_metadata(&contract, &conn);
        save_contract_table_data(&contract, &conn);
        save_contract_table_schema_data(&contract, &conn);
        save_contract_host_data(&contract, &conn);
        return true;
    }

    return false;
}

pub fn has_role_name(role_name: &str, config: &DbiConfigSqlite) -> Result<bool> {
    let conn = get_rcd_conn(&config);
    let mut has_role = false;

    let cmd = &String::from(&CDS::text_get_role());
    let mut statement = conn.prepare(cmd).unwrap();

    let rows = statement.query_map(&[(":rolename", role_name.to_string().as_str())], |row| {
        row.get(0)
    })?;

    for item in rows {
        let count: u64 = item.unwrap();
        if count > 0 {
            has_role = true;
        }
    }

    return Ok(has_role);
}

pub fn add_login_to_role(login: &str, role_name: &str, config: &DbiConfigSqlite) {
    let conn = get_rcd_conn(&config);
    let cmd = &String::from(&CDS::text_add_user_role());
    let mut statement = conn.prepare(cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":rolename": role_name })
        .unwrap();
}

pub fn login_is_in_role(login: &str, role_name: &str, config: &DbiConfigSqlite) -> Result<bool> {
    let conn = get_rcd_conn(&config);
    let mut login_is_in_role = false;
    let cmd = &CDS::text_get_user_role();
    let mut statement = conn.prepare(cmd).unwrap();

    let params = [(":username", login), (":rolename", role_name)];

    let rows = statement.query_map(&params, |row| row.get(0))?;

    for item in rows {
        let count: u64 = item.unwrap();
        if count > 0 {
            login_is_in_role = true;
        }
    }

    return Ok(login_is_in_role);
}

pub fn create_login(login: &str, pw: &str, config: &DbiConfigSqlite) {
    let conn = get_rcd_conn(&config);
    // https://www.reddit.com/r/rust/comments/2sipzj/is_there_an_easy_way_to_hash_passwords_in_rust/
    // https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/

    info!("un and pw: {} {}", login, pw);

    let login_hash = crate::crypt::hash(&pw);
    let cmd = &String::from(CDS::text_add_user());
    let mut statement = conn.prepare(cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":hash": login_hash.0 })
        .unwrap();
}

pub fn has_login_via_config(login: &str, config: DbiConfigSqlite) -> Result<bool> {
    let conn = get_rcd_conn(&config);
    return has_login(login, &conn);
}

fn get_rcd_conn(config: &DbiConfigSqlite) -> Connection {
    let db_path = Path::new(&config.root_folder).join(&config.rcd_db_name);
    return Connection::open(&db_path).unwrap();
}

pub fn configure_admin(login: &str, pw: &str, config: DbiConfigSqlite) {
    let conn = get_rcd_conn(&config);

    if !has_login(login, &conn).unwrap() {
        create_login(login, pw, &config);
    }

    if !login_is_in_role(login, &String::from("SysAdmin"), &config).unwrap() {
        add_login_to_role(login, &String::from("SysAdmin"), &config);
    }
}

pub fn if_host_info_exists(config: DbiConfigSqlite) -> bool {
    let cmd = String::from("SELECT COUNT(*) TOTALCOUNT FROM CDS_HOST_INFO");
    return has_any_rows(cmd, &get_rcd_conn(&config));
}

#[allow(dead_code)]
pub fn has_login(login: &str, conn: &Connection) -> Result<bool> {
    let mut has_login = false;
    let cmd =
        &String::from("SELECT count(*) AS USERCOUNT FROM CDS_USER WHERE USERNAME = :username");

    let mut statement = conn.prepare(cmd).unwrap();

    let rows = statement.query_map(&[(login.to_string().as_str())], |row| row.get(0))?;

    for item in rows {
        let count: u64 = item.unwrap();
        if count > 0 {
            has_login = true;
        }
    }

    return Ok(has_login);
}

pub fn configure_rcd_db(config: &DbiConfigSqlite) {
    let _init = env_logger::try_init();

    let root = &config.root_folder;
    let db_name = &config.rcd_db_name;

    log::info!("cwd is {}", &root);
    info!("db_name is {}", &db_name);

    let db_path = Path::new(&root).join(&db_name);
    info!("db_path is {}", db_path.as_os_str().to_str().unwrap());

    if !db_path.exists() {
        let db_conn = Connection::open(&db_path).unwrap();
        create_user_table(&db_conn);
        create_role_table(&db_conn);
        create_user_role_table(&db_conn);
        create_host_info_table(&db_conn);
        pub fn execute_write_on_connection(
            db_name: &str,
            cmd: &str,
            config: &DbiConfigSqlite,
        ) -> usize {
            let conn = get_db_conn(&config, db_name);
            return conn.execute(&cmd, []).unwrap();
        }
        create_contracts_table(&db_conn);
        create_cds_hosts_table(&db_conn);

        let db_has_role = has_role_name(&String::from("SysAdmin"), config).unwrap();

        if !db_has_role {
            let statement = String::from("INSERT INTO CDS_ROLE (ROLENAME) VALUES ('SysAdmin');");
            execute_write_on_connection(&db_name, &statement, config);
        }
    }
}

fn create_user_table(conn: &Connection) {
    conn.execute(&CDS::text_create_user_table(), []).unwrap();
}

fn create_role_table(conn: &Connection) {
    conn.execute(&CDS::text_create_role_table(), []).unwrap();
}

fn create_user_role_table(conn: &Connection) {
    conn.execute(&CDS::text_create_user_role_table(), [])
        .unwrap();
}

fn create_host_info_table(conn: &Connection) {
    conn.execute(&CDS::text_create_host_info_table(), [])
        .unwrap();
}

fn create_contracts_table(conn: &Connection) {
    conn.execute(&CDS::text_create_cds_contracts_table(), [])
        .unwrap();
}

fn create_cds_hosts_table(conn: &Connection) {
    conn.execute(&&CDS::text_create_cds_hosts_table(), [])
        .unwrap();
}

pub fn get_host_info(config: DbiConfigSqlite) -> HostInfo {
    let conn = get_rcd_conn(&config);
    let cmd = String::from(
        "
    SELECT 
        HOST_ID, 
        HOST_NAME, 
        TOKEN 
    FROM 
        CDS_HOST_INFO;",
    );

    let row_to_host_info =
        |host_id: String, host_name: String, token: String| -> Result<HostInfo> {
            let host = HostInfo {
                id: host_id,
                name: host_name,
                token: token.as_bytes().to_vec(),
            };

            Ok(host)
        };

    let mut results: Vec<HostInfo> = Vec::new();

    let mut statement = conn.prepare(&cmd).unwrap();
    let host_infos = statement
        .query_and_then([], |row| {
            row_to_host_info(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
            )
        })
        .unwrap();

    for hi in host_infos {
        results.push(hi.unwrap());
    }

    return results.first().unwrap().clone();
}

pub fn generate_host_info(host_name: &str, config: DbiConfigSqlite) {
    let id = GUID::rand();
    let conn = get_rcd_conn(&config);
    let token_gen = GUID::rand();
    let token = crypt::hash(&token_gen.to_string());

    let cmd = String::from(
        "
            INSERT INTO CDS_HOST_INFO
            (
                HOST_ID,
                HOST_NAME,
                TOKEN
            )
            VALUES
            (
                :id,
                :name,
                :token
            );",
    );
    let mut statement = conn.prepare(&cmd).unwrap();
    statement
        .execute(named_params! {":id" : id.to_string(), ":name" : host_name, ":token" : token.0 })
        .unwrap();
}

pub fn verify_login(login: &str, pw: &str, config: DbiConfigSqlite) -> bool {
    let mut is_verified = false;

    let cmd = &String::from(CDS::text_get_user());
    let conn = get_rcd_conn(&config);

    let mut statement = conn.prepare(cmd).unwrap();

    let user_iter = statement
        .query_map(&[login.to_string().as_str()], |row| {
            Ok(User {
                username: row.get(0).unwrap(),
                hash: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for user in user_iter {
        let returned_value = user.unwrap();

        let mut padded = [0u8; 128];
        returned_value
            .hash
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, val)| {
                padded[i] = val.clone();
            });

        if crate::crypt::verify(padded, pw) {
            is_verified = true;
            break;
        }
    }

    return is_verified;
}


/// checks rcd_db's CDS_CONTRACTS table to see if there already is a record
/// for this contract by contract_id
#[allow(dead_code, unused_variables)]
fn has_contract(contract_id: &str, conn: &Connection) -> bool {
    let mut cmd = String::from("SELECT COUNT(*) TOTALCOUNT FROM CDS_CONTRACTS WHERE CONTRACT_ID = ':cid'");
    cmd = cmd.replace(":cid", contract_id);

    return has_any_rows(cmd, conn);
}

/// saves top level contract data to rcd_db's CDS_CONTRACTS table
#[allow(dead_code, unused_variables)]
fn save_contract_metadata(contract: &Contract, conn: &Connection) {
    unimplemented!("save_contract_metadata not implemented")
}

/// saves a contract's table information to CDS_CONTRACTS_TABLES
#[allow(dead_code, unused_variables)]
fn save_contract_table_data(contract: &Contract, conn: &Connection) {
    unimplemented!("save_contract_table_data not implemented")
}

/// save's a contract's table schema information to CDS_CONTRACTS_TABLE_SCHEMAS
#[allow(dead_code, unused_variables)]
fn save_contract_table_schema_data(contract: &Contract, conn: &Connection) {
    unimplemented!("save_contract_table_schema_data not implmeneted")
}

// save a contract's host information to CDS_HOSTS
#[allow(dead_code, unused_variables)]
fn save_contract_host_data(contract: &Contract, conn: &Connection) {
    unimplemented!("save_contract_host_data not implmeneted")
}