use log::{LevelFilter, set_max_level, Metadata, Record};
use rusqlite::Connection;
use sql_text::create_log_table;
use std::env;
use std::path::Path;
use log::SetLoggerError;

mod sql_text;
pub mod log_entry;

#[derive(Debug)]
pub struct SqliteLog {
    level: LevelFilter,
    database_name: String,
}

impl SqliteLog {
    pub fn new(database_name: String, level: LevelFilter) -> SqliteLog {
        SqliteLog {
            level,
            database_name,
        }
    }

    pub fn configure(&self) {
        let connection = self.get_db_conn();
        connection.execute(&create_log_table(), []).unwrap();
    }

    pub fn get_db_conn(&self) -> Connection {
        let cwd = env::current_dir().unwrap();
        let cwd = cwd.as_os_str();
        let db_path = Path::new(&cwd).join(&self.database_name);
        Connection::open(db_path).unwrap()
    }

    pub fn get_db_location(&self) -> String {
        let cwd = env::current_dir().unwrap();
        let cwd = cwd.as_os_str();
        Path::new(&cwd)
            .join(&self.database_name)
            .into_os_string()
            .into_string()
            .unwrap()
    }

    pub fn init(log_level: LevelFilter, database_name: String) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        let logger = SqliteLog::new(database_name, log_level);
        logger.configure();
        log::set_boxed_logger(Box::new(logger))
    }
}

impl log::Log for SqliteLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let conn = self.get_db_conn();
            let level: String = record.level().to_string();
            let args = record.args();
            let message = format!("{}", format_args!("{}", args));
            let cmd = sql_text::add_log(&level, &message);
            conn.execute(&cmd, []).unwrap();
        }
    }

    fn flush(&self) {}
}
