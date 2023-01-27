use log::{debug, error, info, trace, warn, LevelFilter};
use rcd_sqlite_log::SqliteLog;

fn main() {
    SqliteLog::init(LevelFilter::Trace).unwrap();

    debug!("this is a debug");
    warn!("this is a warn");
    info!("this is an info");
    error!("this is an error");
    trace!("this is a trace");

    let entries = SqliteLog::default_get_last_x_logs(10);

    for e in &entries {
        let message = format!("{} {} {} {}", e.dt, e.dt_utc, e.level, e.message);
        println!("{message}");
    }
}
