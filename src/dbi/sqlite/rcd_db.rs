use super::{has_any_rows, sql_text::CDS};
use crate::{
    crypt,
    dbi::{sqlite::get_db_conn, DbiConfigSqlite},
    host_info::HostInfo,
    rcd_db::User,
};
use guid_create::GUID;
use log::info;
use rusqlite::{named_params, Connection, Result};
use std::path::Path;

pub mod contract;
pub mod role;

pub fn verify_host_by_id(host_id: &str, token: Vec<u8>, config: &DbiConfigSqlite) -> bool {
    println!("host_id: {}", host_id);

    let conn = get_rcd_conn(config);
    let mut cmd = String::from(
        "
    SELECT TOKEN FROM CDS_HOSTS WHERE HOST_ID = ':hid'",
    );
    cmd = cmd.replace(":hid", host_id);

    let mut statement = conn.prepare(&cmd).unwrap();

    let mut returned_tokens: Vec<Vec<u8>> = Vec::new();

    let row_to_token = |token: Vec<u8>| -> Result<Vec<u8>> { Ok(token) };

    let tokens = statement
        .query_and_then([], |row| row_to_token(row.get(0).unwrap()))
        .unwrap();

    for t in tokens {
        returned_tokens.push(t.unwrap());
    }

    if returned_tokens.len() == 0 {
        return false;
    } else {
        return do_vecs_match(&token, returned_tokens.last().unwrap());
    }
}

pub fn verify_host_by_name(host_name: &str, token: Vec<u8>, config: &DbiConfigSqlite) -> bool {
    println!("host_name: {}", host_name);

    let conn = get_rcd_conn(config);
    let mut cmd = String::from(
        "
    SELECT TOKEN FROM CDS_HOSTS WHERE HOST_NAME = ':name'",
    );
    cmd = cmd.replace(":name", host_name);

    let mut statement = conn.prepare(&cmd).unwrap();

    let mut returned_tokens: Vec<Vec<u8>> = Vec::new();

    let row_to_token = |token: Vec<u8>| -> Result<Vec<u8>> { Ok(token) };

    let tokens = statement
        .query_and_then([], |row| row_to_token(row.get(0).unwrap()))
        .unwrap();

    for t in tokens {
        returned_tokens.push(t.unwrap());
    }

    return do_vecs_match(&token, returned_tokens.last().unwrap());
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

    if !role::login_is_in_role(login, &String::from("SysAdmin"), &config).unwrap() {
        role::add_login_to_role(login, &String::from("SysAdmin"), &config);
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
        create_contracts_table_table(&db_conn);
        create_contracts_table_table_schemas(&db_conn);

        let db_has_role = role::has_role_name(&String::from("SysAdmin"), config).unwrap();

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

fn create_contracts_table_table(conn: &Connection) {
    conn.execute(&CDS::text_create_cds_contracts_tables_table(), [])
        .unwrap();
}

fn create_contracts_table_table_schemas(conn: &Connection) {
    conn.execute(&CDS::text_create_cds_contracts_tables_schemas_table(), [])
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
fn has_contract(contract_id: &str, conn: &Connection) -> bool {
    let mut cmd =
        String::from("SELECT COUNT(*) TOTALCOUNT FROM CDS_CONTRACTS WHERE CONTRACT_ID = ':cid'");
    cmd = cmd.replace(":cid", contract_id);

    return has_any_rows(cmd, conn);
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
