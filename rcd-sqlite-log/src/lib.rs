use chrono::{DateTime, Local, Utc};
use indexmap::IndexMap;
use log::SetLoggerError;
use log::{set_max_level, LevelFilter, Metadata, Record};
use log_entry::LogEntry;
use rcd_markdown::markdown_kv_table::build_table;
use rusqlite::{Connection, Result};
use sql_text::{create_log_table, get_last_x_logs};
use std::env;
use std::path::Path;
use std::thread;

pub mod log_entry;
mod sql_text;

static DEFAULT_DB_NAME: &str = "log.sqlite";

#[derive(Debug)]
pub struct SqliteLog {
    level: LevelFilter,
    database_name: String,
    output_to_stdout: bool,
}

impl SqliteLog {
    pub fn new(database_name: String, level: LevelFilter) -> SqliteLog {
        SqliteLog {
            level,
            database_name,
            output_to_stdout: true,
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
            let db_path = self.get_db_location();
            let level: String = record.level().to_string();
            let args = record.args();
            let message = format!("{}", format_args!("{}", args));

            let sql_message = message.clone();
            let sql_level = level.clone();
            let stdout_message = message.clone();
            let stdout_level = level.clone();

            if self.output_to_stdout {
                thread::spawn(move || {
                    log_stdout(stdout_level, stdout_message);
                });
            }

            thread::spawn(move || {
                log_sql(db_path, sql_level, sql_message);
            });
        }
    }

    fn flush(&self) {}
}

fn log_sql(db_location: String, level: String, message: String) {
    let conn = Connection::open(db_location).unwrap();

    let cmd = sql_text::add_log(&level, &message);
    conn.execute(&cmd, []).unwrap();
}

fn log_stdout(level: String, message: String) {
    let message = format_message(&level, &message);
    println!("{}", message)
}

fn format_message(level: &str, message: &str) -> String {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    let dt: String = local.to_string();
    let dt_utc: String = utc.to_string();

    let mut kv: IndexMap<String, String> = IndexMap::new();

    kv.insert("Local DT".to_string(), dt);
    kv.insert("UTC DT".to_string(), dt_utc);
    kv.insert("Level".to_string(), level.to_string());
    kv.insert("Message".to_string(), message.to_string());

    build_table(kv)
}
