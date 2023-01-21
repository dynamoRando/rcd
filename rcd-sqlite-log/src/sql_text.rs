use chrono::{DateTime, Local, Utc};

pub fn create_log_table() -> String {
    String::from(
        "
    CREATE TABLE IF NOT EXISTS log
    (
        log_dt text not null,
        log_dt_utc text not null,
        log_level string not null,
        log_message text not null
    );",
    )
}

pub fn get_last_x_logs(x: u32) -> String {
    let mut result = String::from(
        "
    SELECT 
        log_dt,
        log_dt_utc,
        log_level,
        log_message
    FROM 
        log 
    ORDER BY
        log_dt_utc DESC
    LIMIT :x
    ;",
    );

    result = result.replace(":x", &x.to_string());
    result
}

pub fn add_log(level: &str, message: &str) -> String {
    let mut cmd = String::from(
        "
    INSERT INTO log (
        log_dt,
        log_dt_utc,
        log_level,
        log_message
    )
    VALUES
    (
        ':dt',
        ':utc',
        ':level',
        ':message'
    )
    ;",
    );

    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    let dt: String = local.to_string();
    let dt_utc: String = utc.to_string();

    cmd = cmd.replace(":dt", &dt);
    cmd = cmd.replace(":utc", &dt_utc);
    cmd = cmd.replace(":level", &level.to_string());
    cmd = cmd.replace(":message", message);
    cmd
}
