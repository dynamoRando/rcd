/// represents all the actions for admin'ing an rcd sqlite database
use log::{debug, error, info, log_enabled, Level};
use rusqlite::{named_params, Connection, Result};
use std::path::Path;

#[path = "crypt.rs"] 
pub mod crypt;

const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_USER 
(
    USERNAME VARCHAR(25) UNIQUE,
    HASH TEXT NOT NULL
);";

#[derive(Debug)]
struct User {
    username: String,
    hash: String,
}

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
const GET_LOGIN: &str = "SELECT USERNAME, HASH FROM RCD_USER WHERE USERNAME = :un";
const USER_WITH_ROLE: &str = "SELECT count(*) AS TOTALCOUNT FROM RCD_USER_ROLE WHERE USERNAME = :username AND ROLENAME = :rolename;";
const ADD_USER_TO_ROLE: &str =
    "INSERT INTO RCD_USER_ROLE (USERNAME, ROLENAME) VALUES (:username, :rolename);";

/// Configures an rcd backing store in sqlite
pub fn configure(root: &str, db_name: &str) {
    env_logger::try_init();

    log::info!("cwd is {}", root);
    info!("db_name is {}", db_name);

    let db_path = Path::new(&root).join(&db_name);
    info!("db_path is {}", db_path.as_os_str().to_str().unwrap());

    if !db_path.exists() {
        let db_conn = Connection::open(&db_path).unwrap();
        create_user_table(&db_conn);
        create_role_table(&db_conn);
        create_user_role_table(&db_conn);
        create_host_info_table(&db_conn);
        create_contracts_table(&db_conn);

        let db_has_role = has_role_name(&String::from("SysAdmin"), &db_conn).unwrap();

        if !db_has_role {
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

    if !login_is_in_role(login, &String::from("SysAdmin"), &conn).unwrap() {
        add_login_to_role(login, &String::from("SysAdmin"), &conn);
    }
}

pub fn has_login(login: &str, conn: &Connection) -> Result<bool> {
    let mut has_login = false;
    let cmd =
        &String::from("SELECT count(*) AS USERCOUNT FROM RCD_USER WHERE USERNAME = :username");

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

pub fn verify_login(login: &str, pw: &str, conn: &Connection) -> bool {
    let mut is_verified = false;

    let cmd = &String::from(GET_LOGIN);

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

        if crypt::verify(padded, pw) {
            is_verified = true;
            break;
        }
    }

    return is_verified;
}

pub fn create_login(login: &str, pw: &str, conn: &Connection) {
    // https://www.reddit.com/r/rust/comments/2sipzj/is_there_an_easy_way_to_hash_passwords_in_rust/
    // https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/

    info!("un and pw: {} {}", login, pw);

    let login_hash = crypt::hash(&pw);
    let cmd = &String::from(ADD_LOGIN);
    let mut statement = conn.prepare(cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":hash": login_hash.0 })
        .unwrap();
}

pub fn login_is_in_role(login: &str, role_name: &str, conn: &Connection) -> Result<bool> {
    let mut login_is_in_role = false;
    let cmd = USER_WITH_ROLE;
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

pub fn add_login_to_role(login: &str, role_name: &str, conn: &Connection) {
    let cmd = &String::from(ADD_USER_TO_ROLE);
    let mut statement = conn.prepare(cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":rolename": role_name })
        .unwrap();
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
