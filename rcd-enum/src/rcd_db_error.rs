use thiserror::Error;

#[derive(Error, Debug)]
pub enum RcdDbError {
    #[error("`{0}`")]
    General(String),
    #[error("the database `{0}` does not exist")]
    DbNotFound(String),
    #[error("the table `{0}` does not exist in database `{1}`")]
    TableNotFoundInDatabase(String, String),
    #[error("storage policy not defined for table `{0}`")]
    LogicalStoragePolicyNotSet(String),
}

impl From<rusqlite::Error> for RcdDbError {
    fn from(error: rusqlite::Error) -> Self {
        RcdDbError::General(error.to_string())
    }
}