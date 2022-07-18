/// represents all the actions for admin'ing an rcd sqlite database
use log::info;
use rusqlite::{named_params, Connection, Result};
use std::path::Path;

#[path = "crypt.rs"]
pub mod crypt;

#[path = "sql_text.rs"]
pub mod sql_text;

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    hash: String,
}
#[allow(dead_code)]
/// Configures an rcd backing store in sqlite
pub fn configure(root: &str, db_name: &str) {
    let _init = env_logger::try_init();

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
            let statement = String::from("INSERT INTO CDS_ROLE (ROLENAME) VALUES ('SysAdmin');");
            execute_write(&statement, &db_conn);
        }
    }
}

#[allow(dead_code)]
pub fn configure_admin(login: &str, pw: &str, db_path: &str) {
    let conn = Connection::open(&db_path).unwrap();

    if !has_login(login, &conn).unwrap() {
        create_login(login, pw, &conn);
    }

    if !login_is_in_role(login, &String::from("SysAdmin"), &conn).unwrap() {
        add_login_to_role(login, &String::from("SysAdmin"), &conn);
    }
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
pub fn verify_login(login: &str, pw: &str, conn: &Connection) -> bool {
    let mut is_verified = false;

    let cmd = &String::from(sql_text::CDS::text_get_user());

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

#[allow(dead_code)]
pub fn create_login(login: &str, pw: &str, conn: &Connection) {
    // https://www.reddit.com/r/rust/comments/2sipzj/is_there_an_easy_way_to_hash_passwords_in_rust/
    // https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/

    info!("un and pw: {} {}", login, pw);

    let login_hash = crypt::hash(&pw);
    let cmd = &String::from(sql_text::CDS::text_add_user());
    let mut statement = conn.prepare(cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":hash": login_hash.0 })
        .unwrap();
}

#[allow(dead_code)]
pub fn login_is_in_role(login: &str, role_name: &str, conn: &Connection) -> Result<bool> {
    let mut login_is_in_role = false;
    let cmd = &sql_text::CDS::text_get_user_role();
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
    let cmd = &String::from(&sql_text::CDS::text_add_user_role());
    let mut statement = conn.prepare(cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":rolename": role_name })
        .unwrap();
}

#[allow(dead_code)]
pub fn has_role_name(role_name: &str, conn: &Connection) -> Result<bool> {
    let mut has_role = false;

    let cmd = &String::from(&sql_text::CDS::text_get_role());
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
    conn.execute(&sql_text::CDS::text_create_user_table(), [])
        .unwrap();
}

fn create_role_table(conn: &Connection) {
    conn.execute(&sql_text::CDS::text_create_role_table(), [])
        .unwrap();
}

fn create_user_role_table(conn: &Connection) {
    conn.execute(&sql_text::CDS::text_create_user_role_table(), [])
        .unwrap();
}

fn create_host_info_table(conn: &Connection) {
    conn.execute(&sql_text::CDS::text_create_host_info_table(), [])
        .unwrap();
}

fn create_contracts_table(conn: &Connection) {
    conn.execute(&sql_text::CDS::text_create_cds_contracts_table(), [])
        .unwrap();
}
