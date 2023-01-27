use thiserror::Error;

#[derive(Error, Debug)]
pub enum RcdDbError {
    #[error("the table {table} does not exist")]
    TableDoesNotExist(String)
}