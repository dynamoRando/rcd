use sqlite;
use sqlite::State;
use std::path::Path;

const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_USER 
(
    USERNAME VARCHAR(25) UNIQUE,
    BYTELENGTH INT NOT NULL,
    SALT BLOB NOT NULL,
    HASH BLOB NOT NULL,
    WORKFACTOR INT NOT NULL
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

const ADD_LOGIN: &str = "INSERT INTO RCD_USER (USERNAME, BYTELENGTH, SALT, HASH, WORKFACTOR) VALUES (@username, @bytelength, @salt, @hash, @workfactor);";

/// Configures an rcd backing store in sqlite
pub fn configure(root: &str, db_name: &str) {
    println!("cwd is {}", root);
    println!("db_name is {}", db_name);

    let db_path = Path::new(&root).join(&db_name);
    println!("db_path is {}", db_path.as_os_str().to_str().unwrap());

    if !db_path.exists() {
        let db_conn = sqlite::open(&db_path).unwrap();
        create_user_table(&db_conn);
        create_role_table(&db_conn);
        create_user_role_table(&db_conn);
        create_host_info_table(&db_conn);
        create_contracts_table(&db_conn);

        if !has_role_name(&String::from("SysAdmin"), &db_conn) {
            let statement = String::from("INSERT INTO RCD_ROLE (ROLENAME) VALUES ('SysAdmin');");
            execute_write(&statement, &db_conn);
        }
    }
}

pub fn configure_admin(login: &str, pw: &str, db_path: &str) {
    let conn = sqlite::open(&db_path).unwrap();

    if !has_login(login, &conn) {
        create_login(login, pw);
    }

    if !login_is_in_role(login, &String::from("SysAdmin")) {
        add_login_to_role(login, &String::from("SysAdmin"));
    }

    unimplemented!("not written");
}

pub fn has_login(login: &str, conn: &sqlite::Connection) -> bool {
    let cmd = &String::from("SELECT count(*) AS USERCOUNT FROM RCD_USER WHERE USERNAME = ?");
    return has_item(cmd, login, conn);
}

pub fn create_login(login: &str, pw: &str) {
    // https://www.reddit.com/r/rust/comments/2sipzj/is_there_an_easy_way_to_hash_passwords_in_rust/
    // https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/
    unimplemented!("not written");
}

pub fn login_is_in_role(login: &str, role_name: &str) -> bool {
    unimplemented!("not written");
}

pub fn add_login_to_role(login: &str, role_name: &str) {
    unimplemented!("not written");
}

pub fn has_role_name(role_name: &str, conn: &sqlite::Connection) -> bool {
    let cmd = &String::from("SELECT count(*) AS ROLECOUNT FROM RCD_ROLE WHERE ROLENAME = ?");
    return has_item(cmd, role_name, conn);
}

fn has_item(cmd_text: &str, item: &str, conn: &sqlite::Connection) -> bool {
    let mut has_item = false;

    let mut statement = conn.prepare(cmd_text).unwrap();
    statement.bind(1, item).unwrap();

    while let State::Row = statement.next().unwrap() {
        let count = statement.read::<i64>(0).unwrap();
        if count > 0 {
            has_item = true;
            break;
        }
    }

    return has_item;
}

fn execute_write(statement: &str, conn: &sqlite::Connection) {
    conn.execute(statement).unwrap();
}
fn create_user_table(conn: &sqlite::Connection) {
    conn.execute(CREATE_USER_TABLE).unwrap();
}

fn create_role_table(conn: &sqlite::Connection) {
    conn.execute(CREATE_ROLE_TABLE).unwrap();
}

fn create_user_role_table(conn: &sqlite::Connection) {
    conn.execute(CREATE_USER_ROLE_TABLE).unwrap();
}

fn create_host_info_table(conn: &sqlite::Connection) {
    conn.execute(CREATE_HOST_INFO_TABLE).unwrap();
}

fn create_contracts_table(conn: &sqlite::Connection) {
    conn.execute(CREATE_CONTRACTS_TABLE).unwrap();
}
