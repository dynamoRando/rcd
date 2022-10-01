use rcdproto::rcdp::Contract;

/// represents all the actions for admin'ing an rcd sqlite database
use crate::{dbi::Dbi, host_info::HostInfo};
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub hash: String,
}

/// Configures an rcd backing store
pub fn configure(dbi: &Dbi) {
    dbi.configure_rcd_db();
}

/// Generates the host info and saves it to our rcd_db if it has not alraedy been generated.
/// Will always return the current `HostInfo`
pub fn generate_and_get_host_info(host_name: &str, dbi: Dbi) -> HostInfo {
    return dbi.generate_and_get_host_info(host_name);
}

/// Creates an admin login if one does not already exist and adds it to the `SysAdmin` role
pub fn configure_admin(login: &str, pw: &str, dbi: &Dbi) {
    dbi.configure_admin(login, pw);
}

#[allow(dead_code, unused_variables)]
pub fn save_contract(contract: Contract, conn: &Connection) -> bool {
    unimplemented!();
}

pub fn has_login(login: &str, dbi: &Dbi) -> Result<bool> {
    return Ok(dbi.has_login(login));
}

pub fn verify_host_by_id(host_id: &str, token: Vec<u8>, dbi: &Dbi) -> bool {
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
    return dbi.verify_host_by_id(host_id, token);
}

pub fn verify_host_by_name(host_name: &str, token: Vec<u8>, dbi: &Dbi) -> bool {
    return dbi.verify_host_by_name(host_name, token);
}

pub fn verify_login(login: &str, pw: &str, dbi: &Dbi) -> bool {
    return dbi.verify_login(login, pw);
}

pub fn create_login(login: &str, pw: &str, dbi: &Dbi) {
    dbi.create_login(login, pw);
}

pub fn login_is_in_role(login: &str, role_name: &str, dbi: &Dbi) -> bool {
    return dbi.login_is_in_role(login, role_name);
}

pub fn add_login_to_role(login: &str, role_name: &str, dbi: &Dbi) {
    dbi.add_login_to_role(login, role_name);
}

pub fn has_role_name(role_name: &str, dbi: &Dbi) -> bool {
    return dbi.has_role_name(role_name);
}
