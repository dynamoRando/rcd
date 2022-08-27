/// represents all the actions for admin'ing an rcd sqlite database
use crate::{cdata::Contract, dbi::Dbi, host_info::HostInfo, sql_text::CDS};
use log::info;
use rusqlite::{named_params, Connection, Result};

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub username: String,
    pub hash: String,
}
#[allow(dead_code)]
/// Configures an rcd backing store
pub fn configure(dbi: &Dbi) {
    dbi.configure_rcd_db();
}

#[allow(dead_code, unused_variables)]
/// Generates the host info and saves it to our rcd_db if it has not alraedy been generated.
/// Will always return the current `HostInfo`
pub fn generate_and_get_host_info(host_name: &str, conn: &Connection) -> HostInfo {
    if !HostInfo::exists(conn) {
        HostInfo::generate(host_name, conn);
    }

    return HostInfo::get(conn);
}

#[allow(dead_code)]
/// Creates an admin login if one does not already exist and adds it to the `SysAdmin` role
pub fn configure_admin(login: &str, pw: &str, dbi: &Dbi) {
    dbi.configure_admin(login, pw);
}

#[allow(dead_code, unused_variables)]
pub fn save_contract(contract: Contract, conn: &Connection) -> bool {
    unimplemented!();
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

#[allow(dead_code, unused_variables)]
pub fn verify_host_by_id(host_id: &str, token: Vec<u8>) -> bool {
    /*
        "CREATE TABLE IF NOT EXISTS CDS_HOSTS
        (
            HOST_ID CHAR(36) NOT NULL,
            HOST_NAME VARCHAR(50),
            TOKEN BLOB,
            IP4ADDRESS VARCHAR(25),
            IP6ADDRESS VARCHAR(25),
            PORT INT,
            LAST_COMMUNICATION_UTC DATETIME
        );",
    */
    unimplemented!();
}

#[allow(dead_code, unused_variables)]
pub fn verify_host_by_name(host_name: &str, token: Vec<u8>) -> bool {
    unimplemented!();
}

#[allow(dead_code, unused_variables)]
pub fn verify_login(login: &str, pw: &str, dbi: &Dbi) -> bool {
    return dbi.verify_login(login, pw);
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
