use std::path::Path;
use std::env;
use std::ffi::OsString;
use sqlite;

const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS RCD_USER 
(
    USERNAME VARCHAR(25) UNIQUE,
    BYTELENGTH INT NOT NULL,
    SALT BLOB NOT NULL,
    HASH BLOB NOT NULL,
    WORKFACTOR INT NOT NULL
);";


/// Configures an rcd backing store in sqlite
pub fn configure(root: &str, db_name: &str) {
    println!("cwd is {}", root);
    println!("db_name is {}", db_name);

    let db_path = Path::new(&root).join(&db_name);
    
    println!("db_path is {}", db_path.as_os_str().to_str().unwrap());

    if !db_path.exists() {
        let db_conn = sqlite::open(&db_path).unwrap();
        create_user_table(db_conn);
    }
    
}

fn create_user_table(conn: sqlite::Connection) {
    conn.execute(CREATE_USER_TABLE).unwrap();
}
