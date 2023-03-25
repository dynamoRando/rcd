#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcdError {
    #[prost(uint32, tag = "1")]
    pub number: u32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub help: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcdLogEntry {
    #[prost(string, tag = "1")]
    pub dt: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dt_utc: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub level: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsByLastNumberRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(uint32, tag = "2")]
    pub number_of_logs: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsByLastNumberReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<RcdLogEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(string, tag = "2")]
    pub settings_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCooperativeHostsRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCooperativeHostsReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<HostInfoStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesToHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesToHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag = "2")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesFromHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesFromHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag = "2")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesToHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesToHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag = "2")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesFromHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesFromHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag = "2")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(string, tag = "2")]
    pub rcdx: ::prost::alloc::string::String,
    /// ... so on
    #[prost(string, tag = "3")]
    pub rcd_core: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInfoReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, optional, tag = "2")]
    pub host_info: ::core::option::Option<Host>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeReply {
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenReply {
    #[prost(bool, tag = "1")]
    pub is_successful: bool,
    #[prost(string, tag = "2")]
    pub expiration_utc: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub jwt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, optional, tag = "2")]
    pub contract: ::core::option::Option<Contract>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantsRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantsReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<ParticipantStatus>,
    #[prost(bool, tag = "3")]
    pub is_error: bool,
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<RcdError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabasesRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabasesReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag = "2")]
    pub databases: ::prost::alloc::vec::Vec<DatabaseSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingActionRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub row_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingActionReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPendingActionsRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub action: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPendingActionsReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag = "2")]
    pub pending_statements: ::prost::alloc::vec::Vec<PendingStatement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingStatement {
    #[prost(uint32, tag = "1")]
    pub row_id: u32,
    #[prost(string, tag = "2")]
    pub statement: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub requested_ts_utc: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub host_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub action: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataLogTableStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub use_data_log: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataLogTableStatusReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLogTableStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLogTableStatusReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub use_data_log: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadRowIdsRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub where_clause: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadRowIdsReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, repeated, tag = "2")]
    pub row_ids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataHashRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub row_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataHashReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint64, tag = "2")]
    pub data_hash: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesToHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesToHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesToHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesToHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesFromHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesFromHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesFromHostBehaviorRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangesUpdatesFromHostBehaviorReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthAtParticipantRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub participant_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub participant_alias: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub db_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthAtPartipantReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHostStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub host_alias: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub host_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub status: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHostStatusReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(uint32, tag = "3")]
    pub status: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHostInfoRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHostInfoReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendParticipantContractRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub participant_alias: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendParticipantContractReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_sent: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// a message representing the results of a SQL query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatementResultset {
    #[prost(bool, tag = "1")]
    pub is_error: bool,
    #[prost(string, tag = "2")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub number_of_rows_affected: u64,
    #[prost(message, repeated, tag = "4")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    #[prost(string, tag = "5")]
    pub execution_error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserDatabaseRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserDatabaseReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_created: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sql_statement: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub database_type: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint64, tag = "2")]
    pub total_resultsets: u64,
    #[prost(message, repeated, tag = "3")]
    pub results: ::prost::alloc::vec::Vec<StatementResultset>,
    #[prost(bool, tag = "4")]
    pub is_error: bool,
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<RcdError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteWriteRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sql_statement: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub database_type: u32,
    #[prost(string, tag = "5")]
    pub where_clause: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteWriteReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(uint32, tag = "3")]
    pub total_rows_affected: u32,
    #[prost(bool, tag = "4")]
    pub is_error: bool,
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<RcdError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasTableRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasTableReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub has_table: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContractRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub remote_delete_behavior: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContractReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLogicalStoragePolicyRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub policy_mode: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLogicalStoragePolicyReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogicalStoragePolicyRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogicalStoragePolicyReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag = "2")]
    pub policy_mode: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteCooperativeWriteRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sql_statement: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub database_type: u32,
    #[prost(string, tag = "5")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub participant_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub where_clause: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteCooperativeWriteReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(uint32, tag = "3")]
    pub total_rows_affected: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParticipantRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ip4_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub port: u32,
    #[prost(string, tag = "6")]
    pub http_addr: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub http_port: u32,
    #[prost(string, optional, tag = "8")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParticipantReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPendingContractsRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPendingContractsReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag = "2")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingContractRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub host_alias: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingContractReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectPendingContractRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub host_alias: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectPendingContractReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCoooperativeFeaturesRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCoooperativeFeaturesReply {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthResult {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
}
/// a message for creating a table in a database
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    /// The user requesting the table creation
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database in which to create the table
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    /// The database GUID in which to create the table
    #[prost(string, tag = "3")]
    pub database_guid: ::prost::alloc::string::String,
    /// The name of the table to create
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
    /// a list of columns for the table
    #[prost(message, repeated, tag = "5")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// a message for describing the result of a CreateTableRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableResult {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub table_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowInfo {
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub rowid: u32,
    #[prost(uint64, tag = "4")]
    pub data_hash: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertDataRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cmd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertDataResult {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(uint64, tag = "3")]
    pub data_hash: u64,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub row_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub where_clause: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataResult {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub rows: ::prost::alloc::vec::Vec<RowInfo>,
    /// 0 - unknown
    /// 1 - success (overwrite or overwrite with log)
    /// 2 - pending (queue for review)
    /// 3 - ignored (ignore)
    #[prost(uint32, tag = "5")]
    pub update_status: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag = "2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub where_clause: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataResult {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub rows: ::prost::alloc::vec::Vec<RowInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowFromPartialDatabaseRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag = "2")]
    pub row_address: ::core::option::Option<RowParticipantAddress>,
    #[prost(message, optional, tag = "3")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowFromPartialDatabaseResult {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub row: ::core::option::Option<Row>,
}
/// a message from a host to a participant to save a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveContractRequest {
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Contract>,
    #[prost(message, optional, tag = "2")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(string, optional, tag = "3")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveContractResult {
    #[prost(bool, tag = "1")]
    pub is_saved: bool,
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantAcceptsContractRequest {
    #[prost(message, optional, tag = "1")]
    pub participant: ::core::option::Option<Participant>,
    #[prost(string, tag = "2")]
    pub contract_guid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contract_version_guid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(string, optional, tag = "6")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantAcceptsContractResult {
    #[prost(bool, tag = "1")]
    pub contract_acceptance_is_acknowledged: bool,
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRowDataHashForHostRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag = "2")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(message, optional, tag = "3")]
    pub host_info: ::core::option::Option<Host>,
    #[prost(string, tag = "4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub table_id: u32,
    #[prost(uint32, tag = "8")]
    pub row_id: u32,
    #[prost(uint64, tag = "9")]
    pub updated_hash_value: u64,
    #[prost(bool, tag = "10")]
    pub is_deleted_at_participant: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRowDataHashForHostResponse {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyHostOfRemovedRowRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag = "2")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(message, optional, tag = "3")]
    pub host_info: ::core::option::Option<Host>,
    #[prost(string, tag = "4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub table_id: u32,
    #[prost(uint32, tag = "8")]
    pub row_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyHostOfRemovedRowResponse {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
}
/// A message for basic online testing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestRequest {
    #[prost(string, tag = "1")]
    pub request_time_utc: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_origin_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub request_origin_ip4: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub request_origin_ip6: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub request_port_number: u32,
    #[prost(string, tag = "6")]
    pub request_echo_message: ::prost::alloc::string::String,
}
/// A message for basic online testing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestReply {
    #[prost(string, tag = "1")]
    pub reply_time_utc: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reply_echo_message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub rcdx_version: ::prost::alloc::string::String,
}
/// a message for general information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageInfo {
    #[prost(bool, tag = "1")]
    pub is_little_endian: bool,
    #[prost(string, repeated, tag = "2")]
    pub message_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub message_generated_time_utc: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub message_guid: ::prost::alloc::string::String,
}
/// A message for authentication purposes (note: this is proof of concept, and obviously not secure)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequest {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pw: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub pw_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub jwt: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "6")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message describing the results of an authentication attempt
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResult {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_authenticated: bool,
    #[prost(string, tag = "4")]
    pub authentication_message: ::prost::alloc::string::String,
}
/// A message for creating a user database
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseRequest {
    #[prost(message, optional, tag = "1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag = "2")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
}
/// A message describing the results of a CreateDatabaseRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseResult {
    #[prost(message, optional, tag = "1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag = "2")]
    pub is_successful: bool,
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub database_id: ::prost::alloc::string::String,
}
/// an object for representing a row in a table. used for returning data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub row_id: u32,
    #[prost(message, repeated, tag = "4")]
    pub values: ::prost::alloc::vec::Vec<RowValue>,
    #[prost(bool, tag = "5")]
    pub is_remoteable: bool,
    #[prost(message, optional, tag = "6")]
    pub remote_metadata: ::core::option::Option<RowRemoteMetadata>,
    #[prost(bytes = "vec", tag = "7")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// an object for storing values for a row in a table. used for returning data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowValue {
    #[prost(message, optional, tag = "1")]
    pub column: ::core::option::Option<ColumnSchema>,
    #[prost(bool, tag = "2")]
    pub is_null_value: bool,
    /// we send the raw bytes and expect the client to convert the value based on the column type.
    /// note: this value does not include the 4 byte INT length prefix for variable length fields
    /// to ease conversion refer to the Drummersoft.DrummerDB.Common library, in particular the `DbBinaryConvert` class
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub string_value: ::prost::alloc::string::String,
}
/// describes the data status of the host in relation to the participant
/// if for example the data hash between the host and the participant do not match
/// or if the row was deleted at the participant, but the reference at the host is not
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowRemoteMetadata {
    #[prost(bool, tag = "1")]
    pub is_remote_out_of_sync_with_host: bool,
    #[prost(bool, tag = "2")]
    pub is_hash_out_of_sync_with_host: bool,
    #[prost(bool, tag = "3")]
    pub is_remote_deleted: bool,
    #[prost(bool, tag = "4")]
    pub is_local_deleted: bool,
}
/// a message for describing schema information of a column in a database table
/// see Drummersoft.DrummerDB.Core.Structures.Version.SystemSchemaConstants100 for more information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSchema {
    /// the name of the column. Max length of 50 characters
    #[prost(string, tag = "1")]
    pub column_name: ::prost::alloc::string::String,
    /// The ENUM data type of the column. See DotCommon.SQLColumnType
    #[prost(uint32, tag = "2")]
    pub column_type: u32,
    /// the max or fixed length of the column, if applicable
    #[prost(uint32, tag = "3")]
    pub column_length: u32,
    /// if the column is nullable or not
    #[prost(bool, tag = "4")]
    pub is_nullable: bool,
    /// the ordinal value of the column, i.e. the order in which the column appears in the table
    #[prost(uint32, tag = "5")]
    pub ordinal: u32,
    /// empty string in a request, populated in a response with the table GUID the column is attached to
    #[prost(string, tag = "6")]
    pub table_id: ::prost::alloc::string::String,
    /// empty string in a request, populated in a response with the column GUID value
    #[prost(string, tag = "7")]
    pub column_id: ::prost::alloc::string::String,
    /// if the column is the primary key of the table. If this is part of a list of columns, it is implied to be a composite primary key
    #[prost(bool, tag = "8")]
    pub is_primary_key: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// the unique contract id
    #[prost(string, tag = "1")]
    pub contract_guid: ::prost::alloc::string::String,
    /// a description of the rights in the contract
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the schema of the entire database
    #[prost(message, optional, tag = "3")]
    pub schema: ::core::option::Option<DatabaseSchema>,
    /// a GUID representing the version of the contract
    #[prost(string, tag = "4")]
    pub contract_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub host_info: ::core::option::Option<Host>,
    /// the status of the contract, if applicable
    #[prost(uint32, tag = "6")]
    pub status: u32,
}
/// a message representing information about a participant in the system
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    #[prost(string, tag = "1")]
    pub participant_guid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ip4_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ip6_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub database_port_number: u32,
    #[prost(bytes = "vec", tag = "6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub internal_participant_guid: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub http_addr: ::prost::alloc::string::String,
    #[prost(uint32, tag = "9")]
    pub http_port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantStatus {
    #[prost(message, optional, tag = "1")]
    pub participant: ::core::option::Option<Participant>,
    #[prost(uint32, tag = "2")]
    pub contract_status: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Host {
    #[prost(string, tag = "1")]
    pub host_guid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ip4_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ip6_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub database_port_number: u32,
    #[prost(bytes = "vec", tag = "6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub http_addr: ::prost::alloc::string::String,
    #[prost(uint32, tag = "8")]
    pub http_port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInfoStatus {
    #[prost(message, optional, tag = "1")]
    pub host: ::core::option::Option<Host>,
    #[prost(string, tag = "2")]
    pub last_communcation_utc: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub status: u32,
}
/// a message for describing the schema of a database
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseSchema {
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub tables: ::prost::alloc::vec::Vec<TableSchema>,
    #[prost(uint32, tag = "4")]
    pub database_type: u32,
    #[prost(uint32, tag = "5")]
    pub rcd_database_type: u32,
    #[prost(bool, tag = "6")]
    pub cooperation_enabled: bool,
    #[prost(bool, tag = "7")]
    pub has_participants: bool,
}
/// a message for describing the schema information of a table in a database
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSchema {
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub table_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
    /// Drummersoft.DrummerDB.Core.Structures.Enum.LogicalStoragePolicy
    #[prost(uint32, tag = "6")]
    pub logical_storage_policy: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    #[prost(string, tag = "1")]
    pub transaction_batch_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub transaction_mode: u32,
}
/// a message for identifying the location of a row in a partial database
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowParticipantAddress {
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub row_id: u32,
}
/// Generated client implementations.
pub mod sql_client_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// a service for passing cooperative SQL statements to a rcd instance
    #[derive(Debug, Clone)]
    pub struct SqlClientClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SqlClientClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SqlClientClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SqlClientClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SqlClientClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn is_online(
            &mut self,
            request: impl tonic::IntoRequest<super::TestRequest>,
        ) -> Result<tonic::Response<super::TestReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/IsOnline");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_user_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserDatabaseRequest>,
        ) -> Result<tonic::Response<super::CreateUserDatabaseReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/CreateUserDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn enable_coooperative_features(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableCoooperativeFeaturesRequest>,
        ) -> Result<
            tonic::Response<super::EnableCoooperativeFeaturesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/EnableCoooperativeFeatures",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_read_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteReadRequest>,
        ) -> Result<tonic::Response<super::ExecuteReadReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ExecuteReadAtHost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_write_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteWriteRequest>,
        ) -> Result<tonic::Response<super::ExecuteWriteReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ExecuteWriteAtHost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_cooperative_write_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteCooperativeWriteRequest>,
        ) -> Result<
            tonic::Response<super::ExecuteCooperativeWriteReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ExecuteCooperativeWriteAtHost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_read_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteReadRequest>,
        ) -> Result<tonic::Response<super::ExecuteReadReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ExecuteReadAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_write_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteWriteRequest>,
        ) -> Result<tonic::Response<super::ExecuteWriteReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ExecuteWriteAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn has_table(
            &mut self,
            request: impl tonic::IntoRequest<super::HasTableRequest>,
        ) -> Result<tonic::Response<super::HasTableReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/HasTable");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_logical_storage_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLogicalStoragePolicyRequest>,
        ) -> Result<
            tonic::Response<super::SetLogicalStoragePolicyReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/SetLogicalStoragePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_logical_storage_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLogicalStoragePolicyRequest>,
        ) -> Result<
            tonic::Response<super::GetLogicalStoragePolicyReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetLogicalStoragePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn generate_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContractRequest>,
        ) -> Result<tonic::Response<super::GenerateContractReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GenerateContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::AddParticipantRequest>,
        ) -> Result<tonic::Response<super::AddParticipantReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/AddParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_participant_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::SendParticipantContractRequest>,
        ) -> Result<
            tonic::Response<super::SendParticipantContractReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/SendParticipantContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn review_pending_contracts(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewPendingContractsRequest>,
        ) -> Result<tonic::Response<super::ViewPendingContractsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ReviewPendingContracts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn accept_pending_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptPendingContractRequest>,
        ) -> Result<tonic::Response<super::AcceptPendingContractReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/AcceptPendingContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn reject_pending_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::RejectPendingContractRequest>,
        ) -> Result<tonic::Response<super::RejectPendingContractReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/RejectPendingContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn generate_host_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateHostInfoRequest>,
        ) -> Result<tonic::Response<super::GenerateHostInfoReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GenerateHostInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_host_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeHostStatusRequest>,
        ) -> Result<tonic::Response<super::ChangeHostStatusReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ChangeHostStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn try_auth_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::TryAuthAtParticipantRequest>,
        ) -> Result<tonic::Response<super::TryAuthAtPartipantReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/TryAuthAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_updates_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeUpdatesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangesUpdatesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ChangeUpdatesFromHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_deletes_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeDeletesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangeDeletesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ChangeDeletesFromHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_updates_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeUpdatesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangeUpdatesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ChangeUpdatesToHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_deletes_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeDeletesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangeDeletesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ChangeDeletesToHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_data_hash_at_host(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataHashRequest>,
        ) -> Result<tonic::Response<super::GetDataHashReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetDataHashAtHost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_data_hash_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataHashRequest>,
        ) -> Result<tonic::Response<super::GetDataHashReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetDataHashAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read_row_id_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReadRowIdsRequest>,
        ) -> Result<tonic::Response<super::GetReadRowIdsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/ReadRowIdAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_data_log_table_status_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataLogTableStatusRequest>,
        ) -> Result<tonic::Response<super::GetDataLogTableStatusReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetDataLogTableStatusAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_data_log_table_status_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDataLogTableStatusRequest>,
        ) -> Result<tonic::Response<super::SetDataLogTableStatusReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/SetDataLogTableStatusAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pending_actions_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPendingActionsRequest>,
        ) -> Result<tonic::Response<super::GetPendingActionsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetPendingActionsAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn accept_pending_action_at_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptPendingActionRequest>,
        ) -> Result<tonic::Response<super::AcceptPendingActionReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/AcceptPendingActionAtParticipant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// admin calls
        pub async fn get_databases(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatabasesRequest>,
        ) -> Result<tonic::Response<super::GetDatabasesReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/GetDatabases");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_participants(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParticipantsRequest>,
        ) -> Result<tonic::Response<super::GetParticipantsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetParticipants",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_active_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActiveContractRequest>,
        ) -> Result<tonic::Response<super::GetActiveContractReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetActiveContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn auth_for_token(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequest>,
        ) -> Result<tonic::Response<super::TokenReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/AuthForToken");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn revoke_token(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequest>,
        ) -> Result<tonic::Response<super::RevokeReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/RevokeToken");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_host_info(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequest>,
        ) -> Result<tonic::Response<super::HostInfoReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/GetHostInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequest>,
        ) -> Result<tonic::Response<super::VersionReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/GetVersions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_updates_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUpdatesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetUpdatesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetUpdatesFromHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_updates_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUpdatesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetUpdatesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetUpdatesToHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_deletes_from_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeletesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetDeletesFromHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetDeletesFromHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_deletes_to_host_behavior(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeletesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetDeletesToHostBehaviorReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetDeletesToHostBehavior",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_cooperative_hosts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCooperativeHostsRequest>,
        ) -> Result<tonic::Response<super::GetCooperativeHostsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetCooperativeHosts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingsRequest>,
        ) -> Result<tonic::Response<super::GetSettingsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.SQLClient/GetSettings");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_logs_by_last_number(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLogsByLastNumberRequest>,
        ) -> Result<tonic::Response<super::GetLogsByLastNumberReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.SQLClient/GetLogsByLastNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// a service for communication between different rcd stores
    #[derive(Debug, Clone)]
    pub struct DataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn is_online(
            &mut self,
            request: impl tonic::IntoRequest<super::TestRequest>,
        ) -> Result<tonic::Response<super::TestReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.DataService/IsOnline");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_partial_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatabaseRequest>,
        ) -> Result<tonic::Response<super::CreateDatabaseResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/CreatePartialDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_table_in_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableRequest>,
        ) -> Result<tonic::Response<super::CreateTableResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/CreateTableInDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn insert_command_into_table(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertDataRequest>,
        ) -> Result<tonic::Response<super::InsertDataResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/InsertCommandIntoTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_command_into_table(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataRequest>,
        ) -> Result<tonic::Response<super::UpdateDataResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/UpdateCommandIntoTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_command_into_table(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataRequest>,
        ) -> Result<tonic::Response<super::DeleteDataResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/DeleteCommandIntoTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_row_from_partial_database(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRowFromPartialDatabaseRequest>,
        ) -> Result<
            tonic::Response<super::GetRowFromPartialDatabaseResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/GetRowFromPartialDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::SaveContractRequest>,
        ) -> Result<tonic::Response<super::SaveContractResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/SaveContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn accept_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipantAcceptsContractRequest>,
        ) -> Result<
            tonic::Response<super::ParticipantAcceptsContractResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/AcceptContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_row_data_hash_for_host(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRowDataHashForHostRequest>,
        ) -> Result<
            tonic::Response<super::UpdateRowDataHashForHostResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/UpdateRowDataHashForHost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn notify_host_of_removed_row(
            &mut self,
            request: impl tonic::IntoRequest<super::NotifyHostOfRemovedRowRequest>,
        ) -> Result<
            tonic::Response<super::NotifyHostOfRemovedRowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/x.DataService/NotifyHostOfRemovedRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn try_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::TryAuthRequest>,
        ) -> Result<tonic::Response<super::TryAuthResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/x.DataService/TryAuth");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
