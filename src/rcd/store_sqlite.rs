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

fn has_role_name(role_name: &str, conn: &sqlite::Connection) -> bool {
    let mut has_role = false;

    let cmd_string_base =
        String::from("SELECT count(*) AS ROLECOUNT FROM RCD_ROLE WHERE ROLENAME = ?");
    let mut statement = conn.prepare(cmd_string_base).unwrap();
    statement.bind(1, role_name).unwrap();

    while let State::Row = statement.next().unwrap() {
        let count = statement.read::<i64>(0).unwrap();
        if count > 0 {
            has_role = true;
            break;
        }
    }

    return has_role;
}

fn execute_write(statement: &str, conn: &sqlite::Connection) {
    conn.execute(statement).unwrap();
}
