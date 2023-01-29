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

