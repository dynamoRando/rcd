use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackingApiError {
    #[error("Failed to add user: `{0}` ")]
    AddUser(String),
    #[error("Failed to add participant: `{0}` ")]
    AddParticipant(String),
    #[error("Failed to create account for: `{0}` ")]
    CreateAccountFailed(String),
    #[error("Failed to send contract to: `{0}` ")]
    SendContract(String),
    #[error("No host id for user: `{0}` ")]
    HostIdMissing(String),
    #[error("Unknown Error")]
    Unknown,
}