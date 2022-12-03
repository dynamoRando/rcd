

pub fn url_read_sql_at_host() -> &'static str {
    return "/client/sql/host/read/"
}


#[allow(dead_code)]
pub fn url_read_sql_at_participant() -> &'static str {
    return "/client/sql/participant/read/"
}


pub fn url_write_sql_at_host() -> &'static str {
    return "/client/sql/host/write/"
}

#[allow(dead_code)]
pub fn url_write_sql_at_participant() -> &'static str {
    return "/client/sql/participant/write/"
}

pub fn url_generate_contract() -> &'static str {
    return "/client/databases/contract/generate/"
}

pub fn url_add_participant() -> &'static str {
    return "/client/databases/participant/add/"
}