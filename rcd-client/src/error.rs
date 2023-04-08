use thiserror::Error;

#[derive(Error, Debug)]
pub enum RcdClientError {
    #[error("Grpc Connection Failed: `{0}` ")]
    GrpcError(String),
    #[error("Http Connection Failed: `{0}` ")]
    HttpError(String),
    #[error("Unknown Error")]
    Unknown,
}
