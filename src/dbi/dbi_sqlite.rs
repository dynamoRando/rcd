use super::DbiConfigSqlite;
use crate::{rcd_db::User, sql_text::CDS};
use log::info;
use rusqlite::{named_params, Connection, Result};
use std::path::Path;

pub fn if_rcd_host_info_exists(config: DbiConfigSqlite) -> bool {
    let cmd = String::from("SELECT COUNT(*) TOTALCOUNT FROM CDS_HOST_INFO");
        return has_any_rows(cmd, &get_rcd_conn(config));
}

pub fn configure_admin(login: &str, pw: &str, config: DbiConfigSqlite) {
    let conn = get_rcd_conn(config);

    if !has_login(login, &conn).unwrap() {
        create_login(login, pw, &conn);
    }

    if !login_is_in_role(login, &String::from("SysAdmin"), &conn).unwrap() {
        add_login_to_role(login, &String::from("SysAdmin"), &conn);
    }
}

pub fn configure_rcd_db(config: DbiConfigSqlite) {
    let _init = env_logger::try_init();

    let root = config.root_folder;
    let db_name = config.rcd_db_name;

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
        create_contracts_table(&db_conn);
        create_cds_hosts_table(&db_conn);

        let db_has_role = has_role_name(&String::from("SysAdmin"), &db_conn).unwrap();

        if !db_has_role {
            let statement = String::from("INSERT INTO CDS_ROLE (ROLENAME) VALUES ('SysAdmin');");
            execute_write(&statement, &db_conn);
        }
    }
}

pub fn verify_login(login: &str, pw: &str, config: DbiConfigSqlite) -> bool {
    let mut is_verified = false;

    let cmd = &String::from(CDS::text_get_user());
    let conn = get_rcd_conn(config);

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

fn get_rcd_conn(config: DbiConfigSqlite) -> Connection {
    let db_path = Path::new(&config.root_folder).join(&config.rcd_db_name);
    return Connection::open(&db_path).unwrap();
}

#[allow(dead_code)]
fn get_db_conn(config: DbiConfigSqlite, db_name: &str) -> Connection {
    let db_path = Path::new(&config.root_folder).join(&db_name);
    return Connection::open(&db_path).unwrap();
}

fn execute_write(statement: &str, conn: &Connection) {
    conn.execute(statement, []).unwrap();
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

#[allow(dead_code)]
pub fn has_role_name(role_name: &str, conn: &Connection) -> Result<bool> {
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

#[allow(dead_code)]
pub fn create_login(login: &str, pw: &str, conn: &Connection) {
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

#[allow(dead_code)]
pub fn login_is_in_role(login: &str, role_name: &str, conn: &Connection) -> Result<bool> {
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

#[allow(dead_code)]
pub fn add_login_to_role(login: &str, role_name: &str, conn: &Connection) {
    let cmd = &String::from(&CDS::text_add_user_role());
    let mut statement = conn.prepare(cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":rolename": role_name })
        .unwrap();
}

#[allow(dead_code, unused_variables)]
/// Takes a SELECT COUNT(*) SQL statement and returns if the result is > 0. Usually used to see if a table that has been
/// created has also populated any data in it.
pub fn has_any_rows(cmd: String, conn: &Connection) -> bool {
    return total_count(cmd, conn) > 0;
}

#[allow(dead_code, unused_variables)]
/// Takes a SELECT COUNT(*) SQL statement and returns the value
fn total_count(cmd: String, conn: &Connection) -> u32 {
    return get_scalar_as_u32(cmd, conn);
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