use std::{error::Error, fmt};

#[derive(Debug)]
pub enum RcdGenerateContractError {
    General(String),
    NotAllTablesSet(String),
}

impl Error for RcdGenerateContractError {}

impl fmt::Display for RcdGenerateContractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RcdGenerateContractError")
    }
}
