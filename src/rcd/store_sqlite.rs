use crate::rcd::crypt;
use rusqlite::{params, Connection, Result};
use std::path::Path;

const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_USER 
(
    USERNAME VARCHAR(25) UNIQUE
    HASH BLOB NOT NULL
);";

const CREATE_ROLE_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_ROLE
(
    ROLENAME VARCHAR(25) UNIQUE
);";

const CREATE_USER_ROLE_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_USER_ROLE
(
    USERNAME VARCHAR(25) NOT NULL,
    ROLENAME VARCHAR(25) NOT NULL   
);";

const CREATE_HOST_INFO_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_HOST_INFO
(
    HOST_ID CHAR(36) NOT NULL,
    HOST_NAME VARCHAR(50) NOT NULL,
    TOKEN BLOB NOT NULL
);";

const CREATE_CONTRACTS_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_CONTRACTS
(
    HOST_ID CHAR(36) NOT NULL,
    CONTRACT_ID CHAR(36) NOT NULL,
    CONTRACT_VERSION_ID CHAR(36) NOT NULL,
    DATABASE_NAME VARCHAR(50) NOT NULL,
    DATABASE_ID CHAR(36) NOT NULL,
    DESCRIPTION VARCHAR(255),
    GENERATED_DATE_UTC DATETIME,
    CONTRACT_STATUS INT
);";

const ADD_LOGIN: &str = "INSERT INTO RCD_USER (USERNAME, HASH) VALUES (:username, :hash);";

/// Configures an rcd backing store in sqlite
pub fn configure(root: &str, db_name: &str) {
    println!("cwd is {}", root);
    println!("db_name is {}", db_name);

    let db_path = Path::new(&root).join(&db_name);
    println!("db_path is {}", db_path.as_os_str().to_str().unwrap());

    if !db_path.exists() {
        let db_conn = Connection::open(&db_path).unwrap();
        create_user_table(&db_conn);
        create_role_table(&db_conn);
        create_user_role_table(&db_conn);
        create_host_info_table(&db_conn);
        create_contracts_table(&db_conn);

        if !has_role_name(&String::from("SysAdmin"), &db_conn).unwrap() {
            let statement = String::from("INSERT INTO RCD_ROLE (ROLENAME) VALUES ('SysAdmin');");
            execute_write(&statement, &db_conn);
        }
    }
}

pub fn configure_admin(login: &str, pw: &str, db_path: &str) {
    let conn = Connection::open(&db_path).unwrap();

    if !has_login(login, &conn).unwrap() {
        create_login(login, pw, &conn);
    }

    if !login_is_in_role(login, &String::from("SysAdmin")) {
        add_login_to_role(login, &String::from("SysAdmin"));
    }

    unimplemented!("not written");
}

pub fn has_login(login: &str, conn: &Connection) -> Result<bool> {
    let mut has_login = false;
    let cmd = &String::from("SELECT count(*) AS USERCOUNT FROM RCD_USER WHERE USERNAME = :user");
    
    let mut statement = conn.prepare(cmd).unwrap();

    let rows = statement.query_map(&[(":user", login.to_string().as_str())], |row| {
        row.get(0)
    })?;

    for item in rows {
        let count: u64 = item.unwrap();
        if count > 0 {
            has_login = true;
        }
    }

    return Ok(has_login);
}

pub fn create_login(login: &str, pw: &str, conn: &Connection) {
    // https://www.reddit.com/r/rust/comments/2sipzj/is_there_an_easy_way_to_hash_passwords_in_rust/
    // https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/

    let login_hash = crypt::hash(&pw);
    unimplemented!("not written");
}

pub fn login_is_in_role(login: &str, role_name: &str) -> bool {
    unimplemented!("not written");
}

pub fn add_login_to_role(login: &str, role_name: &str) {
    unimplemented!("not written");
}

pub fn has_role_name(role_name: &str, conn: &Connection) -> Result<bool> {
    let mut has_role = false;

    let cmd =
        &String::from("SELECT count(*) AS ROLECOUNT FROM RCD_ROLE WHERE ROLENAME = :rolename");
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

fn execute_write(statement: &str, conn: &Connection) {
    conn.execute(statement, []).unwrap();
}
fn create_user_table(conn: &Connection) {
    conn.execute(CREATE_USER_TABLE, []).unwrap();
}

fn create_role_table(conn: &Connection) {
    conn.execute(CREATE_ROLE_TABLE, []).unwrap();
}

fn create_user_role_table(conn: &Connection) {
    conn.execute(CREATE_USER_ROLE_TABLE, []).unwrap();
}

fn create_host_info_table(conn: &Connection) {
    conn.execute(CREATE_HOST_INFO_TABLE, []).unwrap();
}

fn create_contracts_table(conn: &Connection) {
    conn.execute(CREATE_CONTRACTS_TABLE, []).unwrap();
}
