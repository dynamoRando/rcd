use thiserror::Error;

#[derive(Error, Debug)]
pub enum RcdDbError {
    #[error("`{0}`")]
    General(String),
    #[error("the database `{0}` does not exist")]
    DbNotFound(String),
    #[error("the table `{0}` does not exist")]
    TableNotFound(String),
    #[error("storage policy not defined for table `{0}`")]
    LogicalStoragePolicyNotSet(String),
}

