#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LogEntry {
    pub dt: String,
    pub dt_utc: String,
    pub level: String,
    pub message: String,
}
