use super::get_rcd_conn;
use crate::dbi::{sqlite::sql_text::CDS, DbiConfigSqlite};
use rusqlite::{named_params, Result};

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
