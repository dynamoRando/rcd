use log::SetLoggerError;
use log::{set_max_level, LevelFilter, Metadata, Record};
use log_entry::LogEntry;
use rusqlite::{Connection, Result};
use sql_text::{create_log_table, get_last_x_logs};
use std::env;
use std::path::Path;

pub mod log_entry;
mod sql_text;

static DEFAULT_DB_NAME: &str = "log.db";

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

    pub fn default_db_conn() -> Connection {
        let cwd = env::current_dir().unwrap();
        let cwd = cwd.as_os_str();
        let db_path = Path::new(&cwd).join(DEFAULT_DB_NAME);
        Connection::open(db_path).unwrap()
    }

    pub fn default_db_location() -> String {
        let cwd = env::current_dir().unwrap();
        let cwd = cwd.as_os_str();
        Path::new(&cwd)
            .join(DEFAULT_DB_NAME)
            .into_os_string()
            .into_string()
            .unwrap()
    }

    pub fn default_get_last_x_logs(x: u32) -> Vec<LogEntry> {
        let cmd = get_last_x_logs(x);
        let conn = Self::default_db_conn();

        let mut statement = conn.prepare(&cmd).unwrap();
        let mut result_entries: Vec<LogEntry> = Vec::new();

        let row_to_entry =
            |dt: String, dt_utc: String, level: String, message: String| -> Result<LogEntry> {
                Ok(LogEntry {
                    dt,
                    dt_utc,
                    level,
                    message,
                })
            };

        let entries = statement
            .query_and_then([], |row| {
                row_to_entry(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                )
            })
            .unwrap();

        for e in entries {
            result_entries.push(e.unwrap());
        }

        result_entries
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

    pub fn init_with_name(
        log_level: LevelFilter,
        database_name: String,
    ) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        let logger = SqliteLog::new(database_name, log_level);
        logger.configure();
        log::set_boxed_logger(Box::new(logger))
    }

    pub fn init(log_level: LevelFilter) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        let logger = SqliteLog::new(DEFAULT_DB_NAME.to_string(), log_level);
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
