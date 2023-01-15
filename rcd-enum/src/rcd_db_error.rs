use std::{error::Error, fmt};

#[derive(Debug)]
pub enum RcdDbError {
    General(String),
    DbNotFound(String),
    TableNotFound(String),
    LogicalStoragePolicyNotSet(String),
}

impl Error for RcdDbError {}

impl fmt::Display for RcdDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RcdDbError")
    }
}
