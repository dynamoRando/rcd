use log::{LevelFilter, debug, warn, info, error, trace};
use rcd_sqlite_log::SqliteLog;

fn main() {
    SqliteLog::init(LevelFilter::Trace, "log.db".to_string()).unwrap();

    debug!("this is a debug");
    warn!("this is a warn");
    info!("this is an info");
    error!("this is an error");
    trace!("this is a trace");
}

