use super::{get_scalar_as_string, get_scalar_as_u32, has_any_rows, sql_text::Cds};
use crate::sqlite::get_db_conn;
use ::rcd_enum::rcd_database_type::RcdDatabaseType;
use chrono::DateTime;
use chrono::Utc;
use guid_create::GUID;
use stdext::function_name;
use tracing::instrument;
use tracing::{info, trace, warn};
use rcd_common::crypt;
use rcd_common::db::*;
use rcd_common::host_info::*;
use rcd_common::user::*;
use rcd_enum::deletes_from_host_behavior::DeletesFromHostBehavior;
use rcd_enum::deletes_to_host_behavior::DeletesToHostBehavior;
use rcd_enum::host_status::HostStatus;
use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use rcd_enum::updates_to_host_behavior::UpdatesToHostBehavior;
use rusqlite::{named_params, Connection, Result};
use std::{fs, path::Path};

pub mod contract;
pub mod role;

pub fn get_rcd_db_type(db_name: &str, config: &DbiConfigSqlite) -> RcdDatabaseType {
    if db_name == config.rcd_db_name {
        return RcdDatabaseType::Rcd;
    }

    if db_name.contains("dbpart") {
        return RcdDatabaseType::Partial;
    }

    let mut db_part_name = db_name.replace(".db", "");
    db_part_name = db_part_name.replace(".dbpart", "");
    db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
    let db_path = Path::new(&config.root_folder).join(&db_part_name);
    if db_path.exists() {
        return RcdDatabaseType::Partial;
    }

    let path = Path::new(&config.root_folder).join(db_name);
    if path.exists() {
        return RcdDatabaseType::Host;
    }

    RcdDatabaseType::Unknown
}

pub fn login_has_token(login: &str, config: &DbiConfigSqlite) -> bool {
    let conn = get_rcd_conn(config);
    let mut cmd = String::from("SELECT COUNT(*) FROM CDS_USER_TOKENS WHERE USERNAME = ':login'");
    cmd = cmd.replace(":login", login);
    has_any_rows(cmd, &conn)
}

pub fn revoke_token(token: &str, config: &DbiConfigSqlite) -> bool {
    let mut cmd = String::from("DELETE FROM CDS_USER_TOKENS WHERE TOKEN = ':token'");
    cmd = cmd.replace(":token", token);
    execute_write_on_connection(&config.rcd_db_name, &cmd, config) > 0
}

pub fn verify_token(token: &str, config: &DbiConfigSqlite) -> bool {
    let conn = get_rcd_conn(config);
    let mut cmd = String::from("SELECT COUNT(*) FROM CDS_USER_TOKENS WHERE TOKEN = ':token'");
    cmd = cmd.replace(":token", token);
    has_any_rows(cmd, &conn)
}

pub fn delete_expired_tokens(config: &DbiConfigSqlite) {
    let conn = get_rcd_conn(config);
    let now = Utc::now().to_rfc3339();

    let mut cmd = String::from("DELETE FROM CDS_USER_TOKENS WHERE EXPIRATION_UTC < ':now'");
    cmd = cmd.replace(":now", &now);

    let _ = conn.execute(&cmd, []);
    let _ = conn.close();
}

pub fn save_token(login: &str, token: &str, expiration: DateTime<Utc>, config: &DbiConfigSqlite) {
    let conn = get_rcd_conn(config);

    let cmd = String::from(
        "
            INSERT INTO CDS_USER_TOKENS
            (
                USERNAME,
                TOKEN,
                ISSUED_UTC,
                EXPIRATION_UTC
            )
            VALUES
            (
                :un,
                :token,
                :issued,
                :expiration
            );",
    );

    let issued = Utc::now().to_rfc3339();
    let expiration = expiration.to_rfc3339();

    let mut statement = conn.prepare(&cmd).unwrap();
    statement
        .execute(named_params! {
            ":un" : login.to_string(),
            ":token" : token,
            ":issued" : issued,
            ":expiration" : expiration,
        })
        .unwrap();
}

pub fn get_updates_to_host_behavior(
    db_name: &str,
    table_name: &str,
    config: &DbiConfigSqlite,
) -> UpdatesToHostBehavior {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);
    let mut cmd = String::from(
        "
        SELECT 
            UPDATES_TO_HOST_BEHAVIOR
        FROM
            CDS_CONTRACTS_TABLES 
        WHERE
            DATABASE_NAME = ':db_name'
        AND
            TABLE_NAME = ':table_name'
        ;",
    );
    cmd = cmd.replace(":db_name", &db_name);
    cmd = cmd.replace(":table_name", table_name);

    let result = get_scalar_as_u32(cmd, &conn);

    UpdatesToHostBehavior::from_u32(result)
}

pub fn get_deletes_to_host_behavior(
    db_name: &str,
    table_name: &str,
    config: &DbiConfigSqlite,
) -> DeletesToHostBehavior {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);
    let mut cmd = String::from(
        "
        SELECT 
            DELETES_TO_HOST_BEHAVIOR
        FROM
            CDS_CONTRACTS_TABLES 
        WHERE
            DATABASE_NAME = ':db_name'
        AND
            TABLE_NAME = ':table_name'
        ;",
    );
    cmd = cmd.replace(":db_name", &db_name);
    cmd = cmd.replace(":table_name", table_name);

    let result = get_scalar_as_u32(cmd, &conn);

    DeletesToHostBehavior::from_u32(result)
}

pub fn get_deletes_from_host_behavior(
    db_name: &str,
    table_name: &str,
    config: &DbiConfigSqlite,
) -> DeletesFromHostBehavior {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);
    let mut cmd = String::from(
        "
        SELECT 
            DELETES_FROM_HOST_BEHAVIOR
        FROM
            CDS_CONTRACTS_TABLES 
        WHERE
            DATABASE_NAME = ':db_name'
        AND
            TABLE_NAME = ':table_name'
        ;",
    );
    cmd = cmd.replace(":db_name", &db_name);
    cmd = cmd.replace(":table_name", table_name);

    let result = get_scalar_as_u32(cmd, &conn);

    DeletesFromHostBehavior::from_u32(result)
}

pub fn get_updates_from_host_behavior(
    db_name: &str,
    table_name: &str,
    config: &DbiConfigSqlite,
) -> UpdatesFromHostBehavior {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);
    let mut cmd = String::from(
        "
        SELECT 
            UPDATES_FROM_HOST_BEHAVIOR
        FROM
            CDS_CONTRACTS_TABLES 
        WHERE
            DATABASE_NAME = ':db_name'
        AND
            TABLE_NAME = ':table_name'
        ;",
    );
    cmd = cmd.replace(":db_name", &db_name);
    cmd = cmd.replace(":table_name", table_name);

    let result = get_scalar_as_u32(cmd, &conn);

    UpdatesFromHostBehavior::from_u32(result)
}

pub fn check_database_name_for_contract_format(db_name: &str, conn: &Connection) -> String {
    let mut db_name = db_name.to_string();

    let mut cmd =
        String::from("SELECT COUNT(*) FROM CDS_CONTRACTS_TABLES WHERE DATABASE_NAME = ':db_name'");
    cmd = cmd.replace(":db_name", &db_name);
    if !has_any_rows(cmd, conn) {
        let message = format!(
            "{}{}",
            "WARNING: check_database_name_for_contract_format no database named: ", db_name
        );
        warn!("[{}]: {}", function_name!(), message);

        if db_name.contains(".dbpart") {
            let message = "check_database_name_for_contract_format: renaming database name to contract version of database";
            info!("[{}]: {}", function_name!(), message);
            db_name = db_name.replace(".dbpart", ".db");
            let message = format!("New database name is: {db_name}");
            info!("[{}]: {}", function_name!(), message);
        }
    }

    db_name
}

pub fn change_updates_from_host_behavior(
    db_name: &str,
    table_name: &str,
    behavior: u32,
    config: &DbiConfigSqlite,
) -> bool {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);

    let cmd = String::from(
        "
        UPDATE CDS_CONTRACTS_TABLES 
        SET UPDATES_FROM_HOST_BEHAVIOR = :behavior 
        WHERE
            DATABASE_NAME = :db_name
        AND
            TABLE_NAME = :table_name
        ;",
    );

    let mut statement = conn.prepare(&cmd).unwrap();
    let result = statement
        .execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )
        .unwrap();

    result > 0
}

pub fn change_deletes_from_host_behavior(
    db_name: &str,
    table_name: &str,
    behavior: u32,
    config: &DbiConfigSqlite,
) -> bool {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);
    let cmd = String::from(
        "
        UPDATE CDS_CONTRACTS_TABLES 
        SET DELETES_FROM_HOST_BEHAVIOR = :behavior 
        WHERE
            DATABASE_NAME = :db_name
        AND
            TABLE_NAME = :table_name
        ;",
    );

    let mut statement = conn.prepare(&cmd).unwrap();
    let result = statement
        .execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )
        .unwrap();

    result > 0
}

pub fn change_updates_to_host_behavior(
    db_name: &str,
    table_name: &str,
    behavior: u32,
    config: &DbiConfigSqlite,
) -> bool {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);
    let cmd = String::from(
        "
        UPDATE CDS_CONTRACTS_TABLES 
        SET UPDATES_TO_HOST_BEHAVIOR = :behavior 
        WHERE
            DATABASE_NAME = :db_name
        AND
            TABLE_NAME = :table_name
        ;",
    );

    let mut statement = conn.prepare(&cmd).unwrap();
    let result = statement
        .execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )
        .unwrap();

    result > 0
}

pub fn change_deletes_to_host_behavior(
    db_name: &str,
    table_name: &str,
    behavior: u32,
    config: &DbiConfigSqlite,
) -> bool {
    let conn = get_rcd_conn(config);
    let db_name = check_database_name_for_contract_format(db_name, &conn);
    let cmd = String::from(
        "
        UPDATE CDS_CONTRACTS_TABLES 
        SET DELETES_TO_HOST_BEHAVIOR = :behavior 
        WHERE
            DATABASE_NAME = :db_name
        AND
            TABLE_NAME = :table_name
        ;",
    );

    let mut statement = conn.prepare(&cmd).unwrap();
    let result = statement
        .execute(
            named_params! {":behavior": behavior, ":db_name" : db_name, ":table_name" : table_name},
        )
        .unwrap();

    result > 0
}

pub fn change_host_status_by_id(host_id: &str, status: u32, config: &DbiConfigSqlite) -> bool {
    let conn = get_rcd_conn(config);

    let cmd = String::from(
        "
    UPDATE CDS_HOSTS SET HOST_STATUS = :status WHERE HOST_ID = ':hid'",
    );
    let mut statement = conn.prepare(&cmd).unwrap();
    let result = statement
        .execute(named_params! {":status": status, ":hid" : host_id})
        .unwrap();

    result > 0
}

pub fn change_host_status_by_name(host_name: &str, status: u32, config: &DbiConfigSqlite) -> bool {
    let conn = get_rcd_conn(config);

    let cmd = String::from(
        "
    UPDATE CDS_HOSTS SET HOST_STATUS = :status WHERE HOST_NAME = :name",
    );
    let mut statement = conn.prepare(&cmd).unwrap();
    let result = statement
        .execute(named_params! {":status": status, ":name" : host_name})
        .unwrap();

    result > 0
}

pub fn verify_host_by_id(host_id: &str, token: Vec<u8>, config: &DbiConfigSqlite) -> bool {
    trace!("host_id: {host_id}");

    let conn = get_rcd_conn(config);
    let mut cmd = String::from(
        "
    SELECT TOKEN FROM CDS_HOSTS WHERE HOST_ID = ':hid' AND HOST_STATUS = 1",
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

    if returned_tokens.is_empty() {
        false
    } else {
        do_vecs_match(&token, returned_tokens.last().unwrap())
    }
}

#[instrument]
pub fn verify_host_by_name(host_name: &str, token: Vec<u8>, config: &DbiConfigSqlite) -> bool {
    
    let conn = get_rcd_conn(config);
    let mut cmd = String::from(
        "
    SELECT TOKEN FROM CDS_HOSTS WHERE HOST_NAME = ':name' AND HOST_STATUS = 1",
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

    if returned_tokens.is_empty() {
        false
    } else {
        do_vecs_match(&token, returned_tokens.last().unwrap())
    }
}

pub fn create_login_with_hash(login: &str, hash: Vec<u8>, config: &DbiConfigSqlite) {
    let conn = get_rcd_conn(config);

    let cmd = Cds::text_add_user();
    let mut statement = conn.prepare(&cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":hash": hash })
        .unwrap();
}

pub fn create_login(login: &str, pw: &str, config: &DbiConfigSqlite) {
    let conn = get_rcd_conn(config);
    // https://www.reddit.com/r/rust/comments/2sipzj/is_there_an_easy_way_to_hash_passwords_in_rust/
    // https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/

    let login_hash = crypt::hash(pw);
    let cmd = Cds::text_add_user();
    let mut statement = conn.prepare(&cmd).unwrap();
    statement
        .execute(named_params! { ":username": login, ":hash": login_hash.0.as_bytes().to_vec() })
        .unwrap();
}

pub fn get_database_names(config: &DbiConfigSqlite) -> Vec<String> {
    let mut databases: Vec<String> = Vec::new();

    for file in fs::read_dir(&config.root_folder).unwrap() {
        let fname = file.unwrap().file_name();
        let name = fname.as_os_str().to_str().unwrap();

        if name.contains(".db") || name.contains(".dbpart") {
            databases.push(name.to_string());
        }
    }

    databases
}

pub fn has_login_via_config(login: &str, config: DbiConfigSqlite) -> Result<bool> {
    let conn = get_rcd_conn(&config);
    has_login(login, &conn)
}

fn get_rcd_conn(config: &DbiConfigSqlite) -> Connection {
    let db_path = Path::new(&config.root_folder).join(&config.rcd_db_name);
    Connection::open(db_path).unwrap()
}

pub fn configure_admin_with_hash(login: &str, hash: Vec<u8>, config: DbiConfigSqlite) {
    let conn = get_rcd_conn(&config);

    if !has_login(login, &conn).unwrap() {
        create_login_with_hash(login, hash, &config);
    }

    if !role::login_is_in_role(login, &String::from("SysAdmin"), &config).unwrap() {
        role::add_login_to_role(login, &String::from("SysAdmin"), &config);
    }
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
    has_any_rows(cmd, &get_rcd_conn(&config))
}

pub fn has_login(login: &str, conn: &Connection) -> Result<bool> {
    let mut has_login = false;
    let cmd =
        &String::from("SELECT count(*) AS USERCOUNT FROM CDS_USER WHERE USERNAME = :username");

    let mut statement = conn.prepare(cmd).unwrap();

    let rows = statement.query_map([(login.to_string().as_str())], |row| row.get(0))?;

    for item in rows {
        let count: u64 = item.unwrap();
        if count > 0 {
            has_login = true;
        }
    }

    Ok(has_login)
}

pub fn execute_write_on_connection(db_name: &str, cmd: &str, config: &DbiConfigSqlite) -> usize {
    let conn = get_db_conn(config, db_name);
    conn.execute(cmd, []).unwrap()
}

pub fn configure_rcd_db(config: &DbiConfigSqlite) {
    let root = &config.root_folder;
    let db_name = &config.rcd_db_name;

    trace!("[{}]: cwd is {}, db_name is {}", function_name!(), &root, &db_name);
    
    let db_path = Path::new(&root).join(db_name);
    trace!("[{}]: db_path is {}", function_name!(), db_path.as_os_str().to_str().unwrap());

    if !db_path.exists() {
        let db_conn = Connection::open(&db_path).unwrap();
        create_user_table(&db_conn);
        create_role_table(&db_conn);
        create_user_role_table(&db_conn);
        create_host_info_table(&db_conn);
        create_contracts_table(&db_conn);
        create_cds_hosts_table(&db_conn);
        create_contracts_table_table(&db_conn);
        create_contracts_table_table_schemas(&db_conn);
        create_user_tokens_table(&db_conn);

        let db_has_role = role::has_role_name(&String::from("SysAdmin"), config).unwrap();

        if !db_has_role {
            let statement = String::from("INSERT INTO CDS_ROLE (ROLENAME) VALUES ('SysAdmin');");
            execute_write_on_connection(db_name, &statement, config);
        }
    } else {
        trace!("[{}]: dir already exists: {db_path:?}", function_name!());
        trace!("[{}]: this can happen if RCD is running via a proxy", function_name!());
    }
}

pub fn get_cooperative_hosts(config: &DbiConfigSqlite) -> Vec<CdsHosts> {
    let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

    let conn = get_rcd_conn(config);
    let cmd = String::from(
        "
    SELECT 
        HOST_ID,
        HOST_NAME,
        TOKEN,
        IP4ADDRESS,
        IP6ADDRESS,
        PORT,
        LAST_COMMUNICATION_UTC,
        HTTP_ADDR,
        HTTP_PORT,
        HOST_STATUS
    FROM
        CDS_HOSTS
    ;",
    );

    let mut statement = conn.prepare(&cmd).unwrap();

    let row_to_host = |host_id: String,
                       host_name: String,
                       token: Vec<u8>,
                       ip4: String,
                       ip6: String,
                       port: u32,
                       last_comm_utc: String,
                       http_addr: String,
                       http_port: u32,
                       status: u32|
     -> Result<CdsHosts> {
        let host = CdsHosts {
            host_id,
            host_name,
            token,
            ip4,
            ip6,
            port,
            last_comm_utc,
            http_addr,
            http_port,
            status: HostStatus::from_u32(status),
        };
        Ok(host)
    };

    let table_hosts = statement
        .query_and_then([], |row| {
            row_to_host(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
                row.get(8).unwrap(),
                row.get(9).unwrap(),
            )
        })
        .unwrap();

    for h in table_hosts {
        cds_host_infos.push(h.unwrap());
    }

    cds_host_infos
}

pub fn get_cds_host_for_part_db(db_name: &str, config: &DbiConfigSqlite) -> Option<CdsHosts> {
    let conn = get_rcd_conn(config);
    let mut cmd = String::from(
        "
    SELECT 
        HOST_ID
    FROM 
        CDS_CONTRACTS 
    WHERE 
        DATABASE_NAME = ':db_name'
    ;",
    );

    cmd = cmd.replace(":db_name", db_name);
    let host_id = get_scalar_as_string(cmd, &conn);

    let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

    let cmd = String::from(
        "
        SELECT 
            HOST_ID,
            HOST_NAME,
            TOKEN,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            LAST_COMMUNICATION_UTC,
            HTTP_ADDR,
            HTTP_PORT,
            HOST_STATUS
        FROM
            CDS_HOSTS
        WHERE
            HOST_ID = :hid
    ;",
    );

    let mut statement = conn.prepare(&cmd).unwrap();

    let row_to_host = |host_id: String,
                       host_name: String,
                       token: Vec<u8>,
                       ip4: String,
                       ip6: String,
                       port: u32,
                       last_comm_utc: String,
                       http_addr: String,
                       http_port: u32,
                       status: u32|
     -> Result<CdsHosts> {
        let host = CdsHosts {
            host_id,
            host_name,
            token,
            ip4,
            ip6,
            port,
            last_comm_utc,
            http_addr,
            http_port,
            status: HostStatus::from_u32(status),
        };
        Ok(host)
    };

    let table_hosts = statement
        .query_and_then(&[(":hid", &host_id)], |row| {
            row_to_host(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
                row.get(8).unwrap(),
                row.get(9).unwrap(),
            )
        })
        .unwrap();

    for h in table_hosts {
        cds_host_infos.push(h.unwrap());
    }

    if !cds_host_infos.is_empty() {
        Some(cds_host_infos.first().unwrap().clone())
    } else {
        None
    }
}

fn create_user_tokens_table(conn: &Connection) {
    conn.execute(&Cds::text_create_user_tokens_table(), [])
        .unwrap();
}

fn create_user_table(conn: &Connection) {
    conn.execute(&Cds::text_create_user_table(), []).unwrap();
}

fn create_role_table(conn: &Connection) {
    conn.execute(&Cds::text_create_role_table(), []).unwrap();
}

fn create_user_role_table(conn: &Connection) {
    conn.execute(&Cds::text_create_user_role_table(), [])
        .unwrap();
}

fn create_host_info_table(conn: &Connection) {
    trace!("creating CDS_HOST_INFO on: {conn:?}");

    conn.execute(&Cds::text_create_host_info_table(), [])
        .unwrap();
}

fn create_contracts_table(conn: &Connection) {
    conn.execute(&Cds::text_create_cds_contracts_table(), [])
        .unwrap();
}

fn create_contracts_table_table(conn: &Connection) {
    conn.execute(&Cds::text_create_cds_contracts_tables_table(), [])
        .unwrap();
}

fn create_contracts_table_table_schemas(conn: &Connection) {
    conn.execute(&Cds::text_create_cds_contracts_tables_schemas_table(), [])
        .unwrap();
}

fn create_cds_hosts_table(conn: &Connection) {
    conn.execute(&Cds::text_create_cds_hosts_table(), [])
        .unwrap();
}

pub fn get_host_info(config: DbiConfigSqlite) -> Option<HostInfo> {
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

    if !results.is_empty() {
        return Some(results.first().unwrap().clone());
    }

    None
}

pub fn generate_host_info(host_name: &str, config: DbiConfigSqlite) {
    let conn = get_rcd_conn(&config);
    let token_gen = GUID::rand();
    let token = crypt::hash(&token_gen.to_string());

    let cmd = "SELECT COUNT(*) HOSTS FROM CDS_HOST_INFO".to_string();
    let has_rows = has_any_rows(cmd, &conn);

    if has_rows {
        let cmd = String::from(
            "
                UPDATE CDS_HOST_INFO
                SET 
                    HOST_NAME = :name,
                    TOKEN = :token
                ;",
        );
        let mut statement = conn.prepare(&cmd).unwrap();
        let total_rows = statement
            .execute(named_params! {":name" : host_name, ":token" : token.0 })
            .unwrap();

        if total_rows == 0 {
            panic!("host info not updated")
        }
    } else {
        let id = GUID::rand();
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
            .execute(
                named_params! {":id" : id.to_string(), ":name" : host_name, ":token" : token.0 },
            )
            .unwrap();
    }
}

pub fn verify_login(login: &str, pw: &str, config: DbiConfigSqlite) -> bool {
    let mut is_verified = false;

    let cmd = Cds::text_get_user();
    let conn = get_rcd_conn(&config);

    let mut statement = conn.prepare(&cmd).unwrap();

    let user_iter = statement
        .query_map([login.to_string().as_str()], |row| {
            Ok(User {
                username: row.get(0).unwrap(),
                hash: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for user in user_iter {
        let returned_value = user.unwrap();

        let mut padded = [0u8; 128];
        returned_value.hash.iter().enumerate().for_each(|(i, val)| {
            padded[i] = *val;
        });

        if crypt::verify(padded, pw) {
            is_verified = true;
            break;
        }
    }

    is_verified
}

/// checks rcd_db's CDS_CONTRACTS table to see if there already is a record
/// for this contract by contract_id
fn has_contract(contract_id: &str, conn: &Connection) -> bool {
    let mut cmd =
        String::from("SELECT COUNT(*) TOTALCOUNT FROM CDS_CONTRACTS WHERE CONTRACT_ID = ':cid'");
    cmd = cmd.replace(":cid", contract_id);

    has_any_rows(cmd, conn)
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
