#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcdError {
    #[prost(uint32, tag="1")]
    pub number: u32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub help: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcdLogEntry {
    #[prost(string, tag="1")]
    pub dt: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub dt_utc: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub level: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsByLastNumberRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(uint32, tag="2")]
    pub number_of_logs: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsByLastNumberReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag="2")]
    pub logs: ::prost::alloc::vec::Vec<RcdLogEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(string, tag="2")]
    pub settings_json: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCooperativeHostsRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCooperativeHostsReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag="2")]
    pub hosts: ::prost::alloc::vec::Vec<HostInfoStatus>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesToHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesToHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag="2")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesFromHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeletesFromHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag="2")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesToHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesToHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag="2")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesFromHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUpdatesFromHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag="2")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(string, tag="2")]
    pub rcdx: ::prost::alloc::string::String,
    /// ... so on
    #[prost(string, tag="3")]
    pub rcd_core: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInfoReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, optional, tag="2")]
    pub host_info: ::core::option::Option<Host>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeReply {
    #[prost(bool, tag="1")]
    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenReply {
    #[prost(bool, tag="1")]
    pub is_successful: bool,
    #[prost(string, tag="2")]
    pub expiration_utc: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub jwt: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, optional, tag="2")]
    pub contract: ::core::option::Option<Contract>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantsRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantsReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag="2")]
    pub participants: ::prost::alloc::vec::Vec<ParticipantStatus>,
    #[prost(bool, tag="3")]
    pub is_error: bool,
    #[prost(message, optional, tag="4")]
    pub error: ::core::option::Option<RcdError>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabasesRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabasesReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag="2")]
    pub databases: ::prost::alloc::vec::Vec<DatabaseSchema>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingActionRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingActionReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPendingActionsRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub action: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPendingActionsReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag="2")]
    pub pending_statements: ::prost::alloc::vec::Vec<PendingStatement>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingStatement {
    #[prost(uint32, tag="1")]
    pub row_id: u32,
    #[prost(string, tag="2")]
    pub statement: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub requested_ts_utc: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub host_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub action: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataLogTableStatusRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub use_data_log: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataLogTableStatusReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLogTableStatusRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLogTableStatusReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub use_data_log: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadRowIdsRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub where_clause: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadRowIdsReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, repeated, tag="2")]
    pub row_ids: ::prost::alloc::vec::Vec<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataHashRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataHashReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint64, tag="2")]
    pub data_hash: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesToHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesToHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesToHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesToHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesFromHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDeletesFromHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeUpdatesFromHostBehaviorRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangesUpdatesFromHostBehaviorReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthAtParticipantRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub participant_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub participant_alias: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthAtPartipantReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHostStatusRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub host_alias: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub host_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub status: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHostStatusReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(uint32, tag="3")]
    pub status: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHostInfoRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub host_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHostInfoReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendParticipantContractRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub participant_alias: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendParticipantContractReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_sent: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
/// a message representing the results of a SQL query
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatementResultset {
    #[prost(bool, tag="1")]
    pub is_error: bool,
    #[prost(string, tag="2")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub number_of_rows_affected: u64,
    #[prost(message, repeated, tag="4")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    #[prost(string, tag="5")]
    pub execution_error_message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserDatabaseRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserDatabaseReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_created: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sql_statement: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub database_type: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReadReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint64, tag="2")]
    pub total_resultsets: u64,
    #[prost(message, repeated, tag="3")]
    pub results: ::prost::alloc::vec::Vec<StatementResultset>,
    #[prost(bool, tag="4")]
    pub is_error: bool,
    #[prost(message, optional, tag="5")]
    pub error: ::core::option::Option<RcdError>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteWriteRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sql_statement: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub database_type: u32,
    #[prost(string, tag="5")]
    pub where_clause: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteWriteReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(uint32, tag="3")]
    pub total_rows_affected: u32,
    #[prost(bool, tag="4")]
    pub is_error: bool,
    #[prost(message, optional, tag="5")]
    pub error: ::core::option::Option<RcdError>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasTableRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasTableReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub has_table: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContractRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub remote_delete_behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContractReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLogicalStoragePolicyRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub policy_mode: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLogicalStoragePolicyReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogicalStoragePolicyRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogicalStoragePolicyReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(uint32, tag="2")]
    pub policy_mode: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteCooperativeWriteRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sql_statement: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub database_type: u32,
    #[prost(string, tag="5")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub participant_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub where_clause: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteCooperativeWriteReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(uint32, tag="3")]
    pub total_rows_affected: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParticipantRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ip4_address: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub port: u32,
    #[prost(string, tag="6")]
    pub http_addr: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub http_port: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParticipantReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPendingContractsRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPendingContractsReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(message, repeated, tag="2")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingContractRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub host_alias: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptPendingContractReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectPendingContractRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub host_alias: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectPendingContractReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCoooperativeFeaturesRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCoooperativeFeaturesReply {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryAuthResult {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
}
/// a message for creating a table in a database
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    /// The user requesting the table creation
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database in which to create the table
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    /// The database GUID in which to create the table
    #[prost(string, tag="3")]
    pub database_guid: ::prost::alloc::string::String,
    /// The name of the table to create
    #[prost(string, tag="4")]
    pub table_name: ::prost::alloc::string::String,
    /// a list of columns for the table
    #[prost(message, repeated, tag="5")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// a message for describing the result of a CreateTableRequest
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableResult {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub table_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowInfo {
    #[prost(string, tag="1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub rowid: u32,
    #[prost(uint64, tag="4")]
    pub data_hash: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertDataRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cmd: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertDataResult {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(uint64, tag="3")]
    pub data_hash: u64,
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub where_clause: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataResult {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub rows: ::prost::alloc::vec::Vec<RowInfo>,
    /// 0 - unknown
    /// 1 - success (overwrite or overwrite with log)
    /// 2 - pending (queue for review)
    /// 3 - ignored (ignore)
    #[prost(uint32, tag="5")]
    pub update_status: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub where_clause: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataResult {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub rows: ::prost::alloc::vec::Vec<RowInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowFromPartialDatabaseRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag="2")]
    pub row_address: ::core::option::Option<RowParticipantAddress>,
    #[prost(message, optional, tag="3")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowFromPartialDatabaseResult {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub row: ::core::option::Option<Row>,
}
/// a message from a host to a participant to save a contract
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveContractRequest {
    #[prost(message, optional, tag="1")]
    pub contract: ::core::option::Option<Contract>,
    #[prost(message, optional, tag="2")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveContractResult {
    #[prost(bool, tag="1")]
    pub is_saved: bool,
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantAcceptsContractRequest {
    #[prost(message, optional, tag="1")]
    pub participant: ::core::option::Option<Participant>,
    #[prost(string, tag="2")]
    pub contract_guid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub contract_version_guid: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantAcceptsContractResult {
    #[prost(bool, tag="1")]
    pub contract_acceptance_is_acknowledged: bool,
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRowDataHashForHostRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag="2")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(message, optional, tag="3")]
    pub host_info: ::core::option::Option<Host>,
    #[prost(string, tag="4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub table_id: u32,
    #[prost(uint32, tag="8")]
    pub row_id: u32,
    #[prost(uint64, tag="9")]
    pub updated_hash_value: u64,
    #[prost(bool, tag="10")]
    pub is_deleted_at_participant: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRowDataHashForHostResponse {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyHostOfRemovedRowRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag="2")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(message, optional, tag="3")]
    pub host_info: ::core::option::Option<Host>,
    #[prost(string, tag="4")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub table_id: u32,
    #[prost(uint32, tag="8")]
    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyHostOfRemovedRowResponse {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
}
/// A message for basic online testing
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestRequest {
    #[prost(string, tag="1")]
    pub request_time_utc: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub request_origin_url: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub request_origin_ip4: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub request_origin_ip6: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub request_port_number: u32,
    #[prost(string, tag="6")]
    pub request_echo_message: ::prost::alloc::string::String,
}
/// A message for basic online testing
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestReply {
    #[prost(string, tag="1")]
    pub reply_time_utc: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub reply_echo_message: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub rcdx_version: ::prost::alloc::string::String,
}
/// a message for general information
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageInfo {
    #[prost(bool, tag="1")]
    pub is_little_endian: bool,
    #[prost(string, repeated, tag="2")]
    pub message_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub message_generated_time_utc: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub message_guid: ::prost::alloc::string::String,
}
/// A message for authentication purposes (note: this is proof of concept, and obviously not secure)
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequest {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pw: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub pw_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub jwt: ::prost::alloc::string::String,
}
/// A message describing the results of an authentication attempt
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResult {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_authenticated: bool,
    #[prost(string, tag="4")]
    pub authentication_message: ::prost::alloc::string::String,
}
/// A message for creating a user database
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseRequest {
    #[prost(message, optional, tag="1")]
    pub authentication: ::core::option::Option<AuthRequest>,
    #[prost(message, optional, tag="2")]
    pub message_info: ::core::option::Option<MessageInfo>,
    #[prost(string, tag="3")]
    pub database_name: ::prost::alloc::string::String,
}
/// A message describing the results of a CreateDatabaseRequest
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseResult {
    #[prost(message, optional, tag="1")]
    pub authentication_result: ::core::option::Option<AuthResult>,
    #[prost(bool, tag="2")]
    pub is_successful: bool,
    #[prost(string, tag="3")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub result_message: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub database_id: ::prost::alloc::string::String,
}
/// an object for representing a row in a table. used for returning data
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(string, tag="1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub row_id: u32,
    #[prost(message, repeated, tag="4")]
    pub values: ::prost::alloc::vec::Vec<RowValue>,
    #[prost(bool, tag="5")]
    pub is_remoteable: bool,
    #[prost(message, optional, tag="6")]
    pub remote_metadata: ::core::option::Option<RowRemoteMetadata>,
    #[prost(bytes="vec", tag="7")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// an object for storing values for a row in a table. used for returning data
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowValue {
    #[prost(message, optional, tag="1")]
    pub column: ::core::option::Option<ColumnSchema>,
    #[prost(bool, tag="2")]
    pub is_null_value: bool,
    /// we send the raw bytes and expect the client to convert the value based on the column type. 
    /// note: this value does not include the 4 byte INT length prefix for variable length fields
    /// to ease conversion refer to the Drummersoft.DrummerDB.Common library, in particular the `DbBinaryConvert` class
    #[prost(bytes="vec", tag="3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub string_value: ::prost::alloc::string::String,
}
/// describes the data status of the host in relation to the participant
/// if for example the data hash between the host and the participant do not match
/// or if the row was deleted at the participant, but the reference at the host is not
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowRemoteMetadata {
    #[prost(bool, tag="1")]
    pub is_remote_out_of_sync_with_host: bool,
    #[prost(bool, tag="2")]
    pub is_hash_out_of_sync_with_host: bool,
    #[prost(bool, tag="3")]
    pub is_remote_deleted: bool,
    #[prost(bool, tag="4")]
    pub is_local_deleted: bool,
}
/// a message for describing schema information of a column in a database table
/// see Drummersoft.DrummerDB.Core.Structures.Version.SystemSchemaConstants100 for more information
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSchema {
    /// the name of the column. Max length of 50 characters
    #[prost(string, tag="1")]
    pub column_name: ::prost::alloc::string::String,
    /// The ENUM data type of the column. See DotCommon.SQLColumnType
    #[prost(uint32, tag="2")]
    pub column_type: u32,
    /// the max or fixed length of the column, if applicable
    #[prost(uint32, tag="3")]
    pub column_length: u32,
    /// if the column is nullable or not
    #[prost(bool, tag="4")]
    pub is_nullable: bool,
    /// the ordinal value of the column, i.e. the order in which the column appears in the table
    #[prost(uint32, tag="5")]
    pub ordinal: u32,
    /// empty string in a request, populated in a response with the table GUID the column is attached to
    #[prost(string, tag="6")]
    pub table_id: ::prost::alloc::string::String,
    /// empty string in a request, populated in a response with the column GUID value
    #[prost(string, tag="7")]
    pub column_id: ::prost::alloc::string::String,
    /// if the column is the primary key of the table. If this is part of a list of columns, it is implied to be a composite primary key
    #[prost(bool, tag="8")]
    pub is_primary_key: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// the unique contract id
    #[prost(string, tag="1")]
    pub contract_guid: ::prost::alloc::string::String,
    /// a description of the rights in the contract 
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// the schema of the entire database
    #[prost(message, optional, tag="3")]
    pub schema: ::core::option::Option<DatabaseSchema>,
    /// a GUID representing the version of the contract
    #[prost(string, tag="4")]
    pub contract_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub host_info: ::core::option::Option<Host>,
    /// the status of the contract, if applicable
    #[prost(uint32, tag="6")]
    pub status: u32,
}
/// a message representing information about a participant in the system
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    #[prost(string, tag="1")]
    pub participant_guid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub ip4_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ip6_address: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub database_port_number: u32,
    #[prost(bytes="vec", tag="6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub internal_participant_guid: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub http_addr: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub http_port: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantStatus {
    #[prost(message, optional, tag="1")]
    pub participant: ::core::option::Option<Participant>,
    #[prost(uint32, tag="2")]
    pub contract_status: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Host {
    #[prost(string, tag="1")]
    pub host_guid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub ip4_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ip6_address: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub database_port_number: u32,
    #[prost(bytes="vec", tag="6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub http_addr: ::prost::alloc::string::String,
    #[prost(uint32, tag="8")]
    pub http_port: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInfoStatus {
    #[prost(message, optional, tag="1")]
    pub host: ::core::option::Option<Host>,
    #[prost(string, tag="2")]
    pub last_communcation_utc: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub status: u32,
}
/// a message for describing the schema of a database
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseSchema {
    #[prost(string, tag="1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub tables: ::prost::alloc::vec::Vec<TableSchema>,
    #[prost(uint32, tag="4")]
    pub database_type: u32,
    #[prost(uint32, tag="5")]
    pub rcd_database_type: u32,
    #[prost(bool, tag="6")]
    pub cooperation_enabled: bool,
    #[prost(bool, tag="7")]
    pub has_participants: bool,
}
/// a message for describing the schema information of a table in a database 
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSchema {
    #[prost(string, tag="1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub table_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
    /// Drummersoft.DrummerDB.Core.Structures.Enum.LogicalStoragePolicy
    #[prost(uint32, tag="6")]
    pub logical_storage_policy: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    #[prost(string, tag="1")]
    pub transaction_batch_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub transaction_mode: u32,
}
/// a message for identifying the location of a row in a partial database
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowParticipantAddress {
    #[prost(string, tag="1")]
    pub database_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
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
            let path = http::uri::PathAndQuery::from_static("/rcdp.SQLClient/IsOnline");
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
                "/rcdp.SQLClient/CreateUserDatabase",
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
                "/rcdp.SQLClient/EnableCoooperativeFeatures",
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
                "/rcdp.SQLClient/ExecuteReadAtHost",
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
                "/rcdp.SQLClient/ExecuteWriteAtHost",
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
                "/rcdp.SQLClient/ExecuteCooperativeWriteAtHost",
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
                "/rcdp.SQLClient/ExecuteReadAtParticipant",
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
                "/rcdp.SQLClient/ExecuteWriteAtParticipant",
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
            let path = http::uri::PathAndQuery::from_static("/rcdp.SQLClient/HasTable");
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
                "/rcdp.SQLClient/SetLogicalStoragePolicy",
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
                "/rcdp.SQLClient/GetLogicalStoragePolicy",
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
                "/rcdp.SQLClient/GenerateContract",
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
                "/rcdp.SQLClient/AddParticipant",
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
                "/rcdp.SQLClient/SendParticipantContract",
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
                "/rcdp.SQLClient/ReviewPendingContracts",
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
                "/rcdp.SQLClient/AcceptPendingContract",
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
                "/rcdp.SQLClient/RejectPendingContract",
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
                "/rcdp.SQLClient/GenerateHostInfo",
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
                "/rcdp.SQLClient/ChangeHostStatus",
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
                "/rcdp.SQLClient/TryAuthAtParticipant",
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
                "/rcdp.SQLClient/ChangeUpdatesFromHostBehavior",
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
                "/rcdp.SQLClient/ChangeDeletesFromHostBehavior",
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
                "/rcdp.SQLClient/ChangeUpdatesToHostBehavior",
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
                "/rcdp.SQLClient/ChangeDeletesToHostBehavior",
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
                "/rcdp.SQLClient/GetDataHashAtHost",
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
                "/rcdp.SQLClient/GetDataHashAtParticipant",
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
                "/rcdp.SQLClient/ReadRowIdAtParticipant",
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
                "/rcdp.SQLClient/GetDataLogTableStatusAtParticipant",
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
                "/rcdp.SQLClient/SetDataLogTableStatusAtParticipant",
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
                "/rcdp.SQLClient/GetPendingActionsAtParticipant",
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
                "/rcdp.SQLClient/AcceptPendingActionAtParticipant",
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
            let path = http::uri::PathAndQuery::from_static(
                "/rcdp.SQLClient/GetDatabases",
            );
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
                "/rcdp.SQLClient/GetParticipants",
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
                "/rcdp.SQLClient/GetActiveContract",
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
            let path = http::uri::PathAndQuery::from_static(
                "/rcdp.SQLClient/AuthForToken",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/rcdp.SQLClient/RevokeToken",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/rcdp.SQLClient/GetHostInfo",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/rcdp.SQLClient/GetVersions",
            );
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
                "/rcdp.SQLClient/GetUpdatesFromHostBehavior",
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
                "/rcdp.SQLClient/GetUpdatesToHostBehavior",
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
                "/rcdp.SQLClient/GetDeletesFromHostBehavior",
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
                "/rcdp.SQLClient/GetDeletesToHostBehavior",
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
                "/rcdp.SQLClient/GetCooperativeHosts",
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
            let path = http::uri::PathAndQuery::from_static(
                "/rcdp.SQLClient/GetSettings",
            );
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
                "/rcdp.SQLClient/GetLogsByLastNumber",
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
            let path = http::uri::PathAndQuery::from_static(
                "/rcdp.DataService/IsOnline",
            );
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
                "/rcdp.DataService/CreatePartialDatabase",
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
                "/rcdp.DataService/CreateTableInDatabase",
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
                "/rcdp.DataService/InsertCommandIntoTable",
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
                "/rcdp.DataService/UpdateCommandIntoTable",
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
                "/rcdp.DataService/DeleteCommandIntoTable",
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
                "/rcdp.DataService/GetRowFromPartialDatabase",
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
                "/rcdp.DataService/SaveContract",
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
                "/rcdp.DataService/AcceptContract",
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
                "/rcdp.DataService/UpdateRowDataHashForHost",
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
                "/rcdp.DataService/NotifyHostOfRemovedRow",
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
            let path = http::uri::PathAndQuery::from_static("/rcdp.DataService/TryAuth");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod sql_client_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SqlClientServer.
    #[async_trait]
    pub trait SqlClient: Send + Sync + 'static {
        async fn is_online(
            &self,
            request: tonic::Request<super::TestRequest>,
        ) -> Result<tonic::Response<super::TestReply>, tonic::Status>;
        async fn create_user_database(
            &self,
            request: tonic::Request<super::CreateUserDatabaseRequest>,
        ) -> Result<tonic::Response<super::CreateUserDatabaseReply>, tonic::Status>;
        async fn enable_coooperative_features(
            &self,
            request: tonic::Request<super::EnableCoooperativeFeaturesRequest>,
        ) -> Result<
            tonic::Response<super::EnableCoooperativeFeaturesReply>,
            tonic::Status,
        >;
        async fn execute_read_at_host(
            &self,
            request: tonic::Request<super::ExecuteReadRequest>,
        ) -> Result<tonic::Response<super::ExecuteReadReply>, tonic::Status>;
        async fn execute_write_at_host(
            &self,
            request: tonic::Request<super::ExecuteWriteRequest>,
        ) -> Result<tonic::Response<super::ExecuteWriteReply>, tonic::Status>;
        async fn execute_cooperative_write_at_host(
            &self,
            request: tonic::Request<super::ExecuteCooperativeWriteRequest>,
        ) -> Result<tonic::Response<super::ExecuteCooperativeWriteReply>, tonic::Status>;
        async fn execute_read_at_participant(
            &self,
            request: tonic::Request<super::ExecuteReadRequest>,
        ) -> Result<tonic::Response<super::ExecuteReadReply>, tonic::Status>;
        async fn execute_write_at_participant(
            &self,
            request: tonic::Request<super::ExecuteWriteRequest>,
        ) -> Result<tonic::Response<super::ExecuteWriteReply>, tonic::Status>;
        async fn has_table(
            &self,
            request: tonic::Request<super::HasTableRequest>,
        ) -> Result<tonic::Response<super::HasTableReply>, tonic::Status>;
        async fn set_logical_storage_policy(
            &self,
            request: tonic::Request<super::SetLogicalStoragePolicyRequest>,
        ) -> Result<tonic::Response<super::SetLogicalStoragePolicyReply>, tonic::Status>;
        async fn get_logical_storage_policy(
            &self,
            request: tonic::Request<super::GetLogicalStoragePolicyRequest>,
        ) -> Result<tonic::Response<super::GetLogicalStoragePolicyReply>, tonic::Status>;
        async fn generate_contract(
            &self,
            request: tonic::Request<super::GenerateContractRequest>,
        ) -> Result<tonic::Response<super::GenerateContractReply>, tonic::Status>;
        async fn add_participant(
            &self,
            request: tonic::Request<super::AddParticipantRequest>,
        ) -> Result<tonic::Response<super::AddParticipantReply>, tonic::Status>;
        async fn send_participant_contract(
            &self,
            request: tonic::Request<super::SendParticipantContractRequest>,
        ) -> Result<tonic::Response<super::SendParticipantContractReply>, tonic::Status>;
        async fn review_pending_contracts(
            &self,
            request: tonic::Request<super::ViewPendingContractsRequest>,
        ) -> Result<tonic::Response<super::ViewPendingContractsReply>, tonic::Status>;
        async fn accept_pending_contract(
            &self,
            request: tonic::Request<super::AcceptPendingContractRequest>,
        ) -> Result<tonic::Response<super::AcceptPendingContractReply>, tonic::Status>;
        async fn reject_pending_contract(
            &self,
            request: tonic::Request<super::RejectPendingContractRequest>,
        ) -> Result<tonic::Response<super::RejectPendingContractReply>, tonic::Status>;
        async fn generate_host_info(
            &self,
            request: tonic::Request<super::GenerateHostInfoRequest>,
        ) -> Result<tonic::Response<super::GenerateHostInfoReply>, tonic::Status>;
        async fn change_host_status(
            &self,
            request: tonic::Request<super::ChangeHostStatusRequest>,
        ) -> Result<tonic::Response<super::ChangeHostStatusReply>, tonic::Status>;
        async fn try_auth_at_participant(
            &self,
            request: tonic::Request<super::TryAuthAtParticipantRequest>,
        ) -> Result<tonic::Response<super::TryAuthAtPartipantReply>, tonic::Status>;
        async fn change_updates_from_host_behavior(
            &self,
            request: tonic::Request<super::ChangeUpdatesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangesUpdatesFromHostBehaviorReply>,
            tonic::Status,
        >;
        async fn change_deletes_from_host_behavior(
            &self,
            request: tonic::Request<super::ChangeDeletesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangeDeletesFromHostBehaviorReply>,
            tonic::Status,
        >;
        async fn change_updates_to_host_behavior(
            &self,
            request: tonic::Request<super::ChangeUpdatesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangeUpdatesToHostBehaviorReply>,
            tonic::Status,
        >;
        async fn change_deletes_to_host_behavior(
            &self,
            request: tonic::Request<super::ChangeDeletesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::ChangeDeletesToHostBehaviorReply>,
            tonic::Status,
        >;
        async fn get_data_hash_at_host(
            &self,
            request: tonic::Request<super::GetDataHashRequest>,
        ) -> Result<tonic::Response<super::GetDataHashReply>, tonic::Status>;
        async fn get_data_hash_at_participant(
            &self,
            request: tonic::Request<super::GetDataHashRequest>,
        ) -> Result<tonic::Response<super::GetDataHashReply>, tonic::Status>;
        async fn read_row_id_at_participant(
            &self,
            request: tonic::Request<super::GetReadRowIdsRequest>,
        ) -> Result<tonic::Response<super::GetReadRowIdsReply>, tonic::Status>;
        async fn get_data_log_table_status_at_participant(
            &self,
            request: tonic::Request<super::GetDataLogTableStatusRequest>,
        ) -> Result<tonic::Response<super::GetDataLogTableStatusReply>, tonic::Status>;
        async fn set_data_log_table_status_at_participant(
            &self,
            request: tonic::Request<super::SetDataLogTableStatusRequest>,
        ) -> Result<tonic::Response<super::SetDataLogTableStatusReply>, tonic::Status>;
        async fn get_pending_actions_at_participant(
            &self,
            request: tonic::Request<super::GetPendingActionsRequest>,
        ) -> Result<tonic::Response<super::GetPendingActionsReply>, tonic::Status>;
        async fn accept_pending_action_at_participant(
            &self,
            request: tonic::Request<super::AcceptPendingActionRequest>,
        ) -> Result<tonic::Response<super::AcceptPendingActionReply>, tonic::Status>;
        /// admin calls
        async fn get_databases(
            &self,
            request: tonic::Request<super::GetDatabasesRequest>,
        ) -> Result<tonic::Response<super::GetDatabasesReply>, tonic::Status>;
        async fn get_participants(
            &self,
            request: tonic::Request<super::GetParticipantsRequest>,
        ) -> Result<tonic::Response<super::GetParticipantsReply>, tonic::Status>;
        async fn get_active_contract(
            &self,
            request: tonic::Request<super::GetActiveContractRequest>,
        ) -> Result<tonic::Response<super::GetActiveContractReply>, tonic::Status>;
        async fn auth_for_token(
            &self,
            request: tonic::Request<super::AuthRequest>,
        ) -> Result<tonic::Response<super::TokenReply>, tonic::Status>;
        async fn revoke_token(
            &self,
            request: tonic::Request<super::AuthRequest>,
        ) -> Result<tonic::Response<super::RevokeReply>, tonic::Status>;
        async fn get_host_info(
            &self,
            request: tonic::Request<super::AuthRequest>,
        ) -> Result<tonic::Response<super::HostInfoReply>, tonic::Status>;
        async fn get_versions(
            &self,
            request: tonic::Request<super::AuthRequest>,
        ) -> Result<tonic::Response<super::VersionReply>, tonic::Status>;
        async fn get_updates_from_host_behavior(
            &self,
            request: tonic::Request<super::GetUpdatesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetUpdatesFromHostBehaviorReply>,
            tonic::Status,
        >;
        async fn get_updates_to_host_behavior(
            &self,
            request: tonic::Request<super::GetUpdatesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetUpdatesToHostBehaviorReply>,
            tonic::Status,
        >;
        async fn get_deletes_from_host_behavior(
            &self,
            request: tonic::Request<super::GetDeletesFromHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetDeletesFromHostBehaviorReply>,
            tonic::Status,
        >;
        async fn get_deletes_to_host_behavior(
            &self,
            request: tonic::Request<super::GetDeletesToHostBehaviorRequest>,
        ) -> Result<
            tonic::Response<super::GetDeletesToHostBehaviorReply>,
            tonic::Status,
        >;
        async fn get_cooperative_hosts(
            &self,
            request: tonic::Request<super::GetCooperativeHostsRequest>,
        ) -> Result<tonic::Response<super::GetCooperativeHostsReply>, tonic::Status>;
        async fn get_settings(
            &self,
            request: tonic::Request<super::GetSettingsRequest>,
        ) -> Result<tonic::Response<super::GetSettingsReply>, tonic::Status>;
        async fn get_logs_by_last_number(
            &self,
            request: tonic::Request<super::GetLogsByLastNumberRequest>,
        ) -> Result<tonic::Response<super::GetLogsByLastNumberReply>, tonic::Status>;
    }
    /// a service for passing cooperative SQL statements to a rcd instance
    #[derive(Debug)]
    pub struct SqlClientServer<T: SqlClient> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SqlClient> SqlClientServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SqlClientServer<T>
    where
        T: SqlClient,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rcdp.SQLClient/IsOnline" => {
                    #[allow(non_camel_case_types)]
                    struct IsOnlineSvc<T: SqlClient>(pub Arc<T>);
                    impl<T: SqlClient> tonic::server::UnaryService<super::TestRequest>
                    for IsOnlineSvc<T> {
                        type Response = super::TestReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_online(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsOnlineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/CreateUserDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserDatabaseSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::CreateUserDatabaseRequest>
                    for CreateUserDatabaseSvc<T> {
                        type Response = super::CreateUserDatabaseReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_user_database(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateUserDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/EnableCoooperativeFeatures" => {
                    #[allow(non_camel_case_types)]
                    struct EnableCoooperativeFeaturesSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<
                        super::EnableCoooperativeFeaturesRequest,
                    > for EnableCoooperativeFeaturesSvc<T> {
                        type Response = super::EnableCoooperativeFeaturesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EnableCoooperativeFeaturesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).enable_coooperative_features(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EnableCoooperativeFeaturesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ExecuteReadAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteReadAtHostSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::ExecuteReadRequest>
                    for ExecuteReadAtHostSvc<T> {
                        type Response = super::ExecuteReadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteReadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).execute_read_at_host(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteReadAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ExecuteWriteAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteWriteAtHostSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::ExecuteWriteRequest>
                    for ExecuteWriteAtHostSvc<T> {
                        type Response = super::ExecuteWriteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteWriteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).execute_write_at_host(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteWriteAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ExecuteCooperativeWriteAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteCooperativeWriteAtHostSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::ExecuteCooperativeWriteRequest>
                    for ExecuteCooperativeWriteAtHostSvc<T> {
                        type Response = super::ExecuteCooperativeWriteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ExecuteCooperativeWriteRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).execute_cooperative_write_at_host(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteCooperativeWriteAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ExecuteReadAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteReadAtParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::ExecuteReadRequest>
                    for ExecuteReadAtParticipantSvc<T> {
                        type Response = super::ExecuteReadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteReadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).execute_read_at_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteReadAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ExecuteWriteAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteWriteAtParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::ExecuteWriteRequest>
                    for ExecuteWriteAtParticipantSvc<T> {
                        type Response = super::ExecuteWriteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteWriteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).execute_write_at_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteWriteAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/HasTable" => {
                    #[allow(non_camel_case_types)]
                    struct HasTableSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::HasTableRequest>
                    for HasTableSvc<T> {
                        type Response = super::HasTableReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HasTableRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).has_table(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/SetLogicalStoragePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetLogicalStoragePolicySvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::SetLogicalStoragePolicyRequest>
                    for SetLogicalStoragePolicySvc<T> {
                        type Response = super::SetLogicalStoragePolicyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetLogicalStoragePolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_logical_storage_policy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetLogicalStoragePolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetLogicalStoragePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetLogicalStoragePolicySvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetLogicalStoragePolicyRequest>
                    for GetLogicalStoragePolicySvc<T> {
                        type Response = super::GetLogicalStoragePolicyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetLogicalStoragePolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_logical_storage_policy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLogicalStoragePolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GenerateContract" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateContractSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GenerateContractRequest>
                    for GenerateContractSvc<T> {
                        type Response = super::GenerateContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateContractRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).generate_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenerateContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/AddParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct AddParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::AddParticipantRequest>
                    for AddParticipantSvc<T> {
                        type Response = super::AddParticipantReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddParticipantRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/SendParticipantContract" => {
                    #[allow(non_camel_case_types)]
                    struct SendParticipantContractSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::SendParticipantContractRequest>
                    for SendParticipantContractSvc<T> {
                        type Response = super::SendParticipantContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SendParticipantContractRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_participant_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendParticipantContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ReviewPendingContracts" => {
                    #[allow(non_camel_case_types)]
                    struct ReviewPendingContractsSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::ViewPendingContractsRequest>
                    for ReviewPendingContractsSvc<T> {
                        type Response = super::ViewPendingContractsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ViewPendingContractsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).review_pending_contracts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReviewPendingContractsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/AcceptPendingContract" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptPendingContractSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::AcceptPendingContractRequest>
                    for AcceptPendingContractSvc<T> {
                        type Response = super::AcceptPendingContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcceptPendingContractRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).accept_pending_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcceptPendingContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/RejectPendingContract" => {
                    #[allow(non_camel_case_types)]
                    struct RejectPendingContractSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::RejectPendingContractRequest>
                    for RejectPendingContractSvc<T> {
                        type Response = super::RejectPendingContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RejectPendingContractRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).reject_pending_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RejectPendingContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GenerateHostInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateHostInfoSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GenerateHostInfoRequest>
                    for GenerateHostInfoSvc<T> {
                        type Response = super::GenerateHostInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateHostInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).generate_host_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenerateHostInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ChangeHostStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeHostStatusSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::ChangeHostStatusRequest>
                    for ChangeHostStatusSvc<T> {
                        type Response = super::ChangeHostStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeHostStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_host_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeHostStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/TryAuthAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct TryAuthAtParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::TryAuthAtParticipantRequest>
                    for TryAuthAtParticipantSvc<T> {
                        type Response = super::TryAuthAtPartipantReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TryAuthAtParticipantRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).try_auth_at_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TryAuthAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ChangeUpdatesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeUpdatesFromHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<
                        super::ChangeUpdatesFromHostBehaviorRequest,
                    > for ChangeUpdatesFromHostBehaviorSvc<T> {
                        type Response = super::ChangesUpdatesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeUpdatesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_updates_from_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeUpdatesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ChangeDeletesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeDeletesFromHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<
                        super::ChangeDeletesFromHostBehaviorRequest,
                    > for ChangeDeletesFromHostBehaviorSvc<T> {
                        type Response = super::ChangeDeletesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeDeletesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_deletes_from_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeDeletesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ChangeUpdatesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeUpdatesToHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<
                        super::ChangeUpdatesToHostBehaviorRequest,
                    > for ChangeUpdatesToHostBehaviorSvc<T> {
                        type Response = super::ChangeUpdatesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeUpdatesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_updates_to_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeUpdatesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ChangeDeletesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeDeletesToHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<
                        super::ChangeDeletesToHostBehaviorRequest,
                    > for ChangeDeletesToHostBehaviorSvc<T> {
                        type Response = super::ChangeDeletesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeDeletesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_deletes_to_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeDeletesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetDataHashAtHost" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataHashAtHostSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetDataHashRequest>
                    for GetDataHashAtHostSvc<T> {
                        type Response = super::GetDataHashReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDataHashRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_data_hash_at_host(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataHashAtHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetDataHashAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataHashAtParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetDataHashRequest>
                    for GetDataHashAtParticipantSvc<T> {
                        type Response = super::GetDataHashReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDataHashRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_data_hash_at_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataHashAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/ReadRowIdAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct ReadRowIdAtParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetReadRowIdsRequest>
                    for ReadRowIdAtParticipantSvc<T> {
                        type Response = super::GetReadRowIdsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReadRowIdsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).read_row_id_at_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadRowIdAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetDataLogTableStatusAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataLogTableStatusAtParticipantSvc<T: SqlClient>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetDataLogTableStatusRequest>
                    for GetDataLogTableStatusAtParticipantSvc<T> {
                        type Response = super::GetDataLogTableStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDataLogTableStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .get_data_log_table_status_at_participant(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataLogTableStatusAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/SetDataLogTableStatusAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct SetDataLogTableStatusAtParticipantSvc<T: SqlClient>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::SetDataLogTableStatusRequest>
                    for SetDataLogTableStatusAtParticipantSvc<T> {
                        type Response = super::SetDataLogTableStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetDataLogTableStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .set_data_log_table_status_at_participant(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetDataLogTableStatusAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetPendingActionsAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct GetPendingActionsAtParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetPendingActionsRequest>
                    for GetPendingActionsAtParticipantSvc<T> {
                        type Response = super::GetPendingActionsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPendingActionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_pending_actions_at_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPendingActionsAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/AcceptPendingActionAtParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptPendingActionAtParticipantSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::AcceptPendingActionRequest>
                    for AcceptPendingActionAtParticipantSvc<T> {
                        type Response = super::AcceptPendingActionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcceptPendingActionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).accept_pending_action_at_participant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcceptPendingActionAtParticipantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetDatabases" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatabasesSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetDatabasesRequest>
                    for GetDatabasesSvc<T> {
                        type Response = super::GetDatabasesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatabasesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_databases(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatabasesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetParticipants" => {
                    #[allow(non_camel_case_types)]
                    struct GetParticipantsSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetParticipantsRequest>
                    for GetParticipantsSvc<T> {
                        type Response = super::GetParticipantsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParticipantsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_participants(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetParticipantsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetActiveContract" => {
                    #[allow(non_camel_case_types)]
                    struct GetActiveContractSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetActiveContractRequest>
                    for GetActiveContractSvc<T> {
                        type Response = super::GetActiveContractReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetActiveContractRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_active_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetActiveContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/AuthForToken" => {
                    #[allow(non_camel_case_types)]
                    struct AuthForTokenSvc<T: SqlClient>(pub Arc<T>);
                    impl<T: SqlClient> tonic::server::UnaryService<super::AuthRequest>
                    for AuthForTokenSvc<T> {
                        type Response = super::TokenReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).auth_for_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthForTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/RevokeToken" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeTokenSvc<T: SqlClient>(pub Arc<T>);
                    impl<T: SqlClient> tonic::server::UnaryService<super::AuthRequest>
                    for RevokeTokenSvc<T> {
                        type Response = super::RevokeReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).revoke_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RevokeTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetHostInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetHostInfoSvc<T: SqlClient>(pub Arc<T>);
                    impl<T: SqlClient> tonic::server::UnaryService<super::AuthRequest>
                    for GetHostInfoSvc<T> {
                        type Response = super::HostInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_host_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetHostInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetVersions" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionsSvc<T: SqlClient>(pub Arc<T>);
                    impl<T: SqlClient> tonic::server::UnaryService<super::AuthRequest>
                    for GetVersionsSvc<T> {
                        type Response = super::VersionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_versions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetUpdatesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetUpdatesFromHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<
                        super::GetUpdatesFromHostBehaviorRequest,
                    > for GetUpdatesFromHostBehaviorSvc<T> {
                        type Response = super::GetUpdatesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUpdatesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_updates_from_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUpdatesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetUpdatesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetUpdatesToHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetUpdatesToHostBehaviorRequest>
                    for GetUpdatesToHostBehaviorSvc<T> {
                        type Response = super::GetUpdatesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUpdatesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_updates_to_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUpdatesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetDeletesFromHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetDeletesFromHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<
                        super::GetDeletesFromHostBehaviorRequest,
                    > for GetDeletesFromHostBehaviorSvc<T> {
                        type Response = super::GetDeletesFromHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDeletesFromHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_deletes_from_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDeletesFromHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetDeletesToHostBehavior" => {
                    #[allow(non_camel_case_types)]
                    struct GetDeletesToHostBehaviorSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetDeletesToHostBehaviorRequest>
                    for GetDeletesToHostBehaviorSvc<T> {
                        type Response = super::GetDeletesToHostBehaviorReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDeletesToHostBehaviorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_deletes_to_host_behavior(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDeletesToHostBehaviorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetCooperativeHosts" => {
                    #[allow(non_camel_case_types)]
                    struct GetCooperativeHostsSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetCooperativeHostsRequest>
                    for GetCooperativeHostsSvc<T> {
                        type Response = super::GetCooperativeHostsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCooperativeHostsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_cooperative_hosts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCooperativeHostsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetSettings" => {
                    #[allow(non_camel_case_types)]
                    struct GetSettingsSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetSettingsRequest>
                    for GetSettingsSvc<T> {
                        type Response = super::GetSettingsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSettingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_settings(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSettingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.SQLClient/GetLogsByLastNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetLogsByLastNumberSvc<T: SqlClient>(pub Arc<T>);
                    impl<
                        T: SqlClient,
                    > tonic::server::UnaryService<super::GetLogsByLastNumberRequest>
                    for GetLogsByLastNumberSvc<T> {
                        type Response = super::GetLogsByLastNumberReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLogsByLastNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_logs_by_last_number(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLogsByLastNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SqlClient> Clone for SqlClientServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SqlClient> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SqlClient> tonic::server::NamedService for SqlClientServer<T> {
        const NAME: &'static str = "rcdp.SQLClient";
    }
}
/// Generated server implementations.
pub mod data_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DataServiceServer.
    #[async_trait]
    pub trait DataService: Send + Sync + 'static {
        async fn is_online(
            &self,
            request: tonic::Request<super::TestRequest>,
        ) -> Result<tonic::Response<super::TestReply>, tonic::Status>;
        async fn create_partial_database(
            &self,
            request: tonic::Request<super::CreateDatabaseRequest>,
        ) -> Result<tonic::Response<super::CreateDatabaseResult>, tonic::Status>;
        async fn create_table_in_database(
            &self,
            request: tonic::Request<super::CreateTableRequest>,
        ) -> Result<tonic::Response<super::CreateTableResult>, tonic::Status>;
        async fn insert_command_into_table(
            &self,
            request: tonic::Request<super::InsertDataRequest>,
        ) -> Result<tonic::Response<super::InsertDataResult>, tonic::Status>;
        async fn update_command_into_table(
            &self,
            request: tonic::Request<super::UpdateDataRequest>,
        ) -> Result<tonic::Response<super::UpdateDataResult>, tonic::Status>;
        async fn delete_command_into_table(
            &self,
            request: tonic::Request<super::DeleteDataRequest>,
        ) -> Result<tonic::Response<super::DeleteDataResult>, tonic::Status>;
        async fn get_row_from_partial_database(
            &self,
            request: tonic::Request<super::GetRowFromPartialDatabaseRequest>,
        ) -> Result<
            tonic::Response<super::GetRowFromPartialDatabaseResult>,
            tonic::Status,
        >;
        async fn save_contract(
            &self,
            request: tonic::Request<super::SaveContractRequest>,
        ) -> Result<tonic::Response<super::SaveContractResult>, tonic::Status>;
        async fn accept_contract(
            &self,
            request: tonic::Request<super::ParticipantAcceptsContractRequest>,
        ) -> Result<
            tonic::Response<super::ParticipantAcceptsContractResult>,
            tonic::Status,
        >;
        async fn update_row_data_hash_for_host(
            &self,
            request: tonic::Request<super::UpdateRowDataHashForHostRequest>,
        ) -> Result<
            tonic::Response<super::UpdateRowDataHashForHostResponse>,
            tonic::Status,
        >;
        async fn notify_host_of_removed_row(
            &self,
            request: tonic::Request<super::NotifyHostOfRemovedRowRequest>,
        ) -> Result<
            tonic::Response<super::NotifyHostOfRemovedRowResponse>,
            tonic::Status,
        >;
        async fn try_auth(
            &self,
            request: tonic::Request<super::TryAuthRequest>,
        ) -> Result<tonic::Response<super::TryAuthResult>, tonic::Status>;
    }
    /// a service for communication between different rcd stores
    #[derive(Debug)]
    pub struct DataServiceServer<T: DataService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataService> DataServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DataServiceServer<T>
    where
        T: DataService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rcdp.DataService/IsOnline" => {
                    #[allow(non_camel_case_types)]
                    struct IsOnlineSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::TestRequest>
                    for IsOnlineSvc<T> {
                        type Response = super::TestReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_online(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsOnlineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/CreatePartialDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePartialDatabaseSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::CreateDatabaseRequest>
                    for CreatePartialDatabaseSvc<T> {
                        type Response = super::CreateDatabaseResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_partial_database(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePartialDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/CreateTableInDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTableInDatabaseSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::CreateTableRequest>
                    for CreateTableInDatabaseSvc<T> {
                        type Response = super::CreateTableResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTableRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_table_in_database(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTableInDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/InsertCommandIntoTable" => {
                    #[allow(non_camel_case_types)]
                    struct InsertCommandIntoTableSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::InsertDataRequest>
                    for InsertCommandIntoTableSvc<T> {
                        type Response = super::InsertDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InsertDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).insert_command_into_table(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InsertCommandIntoTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/UpdateCommandIntoTable" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCommandIntoTableSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::UpdateDataRequest>
                    for UpdateCommandIntoTableSvc<T> {
                        type Response = super::UpdateDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_command_into_table(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateCommandIntoTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/DeleteCommandIntoTable" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCommandIntoTableSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DeleteDataRequest>
                    for DeleteCommandIntoTableSvc<T> {
                        type Response = super::DeleteDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_command_into_table(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCommandIntoTableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/GetRowFromPartialDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct GetRowFromPartialDatabaseSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::GetRowFromPartialDatabaseRequest,
                    > for GetRowFromPartialDatabaseSvc<T> {
                        type Response = super::GetRowFromPartialDatabaseResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetRowFromPartialDatabaseRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_row_from_partial_database(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRowFromPartialDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/SaveContract" => {
                    #[allow(non_camel_case_types)]
                    struct SaveContractSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::SaveContractRequest>
                    for SaveContractSvc<T> {
                        type Response = super::SaveContractResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SaveContractRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).save_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/AcceptContract" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptContractSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::ParticipantAcceptsContractRequest,
                    > for AcceptContractSvc<T> {
                        type Response = super::ParticipantAcceptsContractResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ParticipantAcceptsContractRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).accept_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcceptContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/UpdateRowDataHashForHost" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRowDataHashForHostSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::UpdateRowDataHashForHostRequest>
                    for UpdateRowDataHashForHostSvc<T> {
                        type Response = super::UpdateRowDataHashForHostResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateRowDataHashForHostRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_row_data_hash_for_host(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateRowDataHashForHostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/NotifyHostOfRemovedRow" => {
                    #[allow(non_camel_case_types)]
                    struct NotifyHostOfRemovedRowSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::NotifyHostOfRemovedRowRequest>
                    for NotifyHostOfRemovedRowSvc<T> {
                        type Response = super::NotifyHostOfRemovedRowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NotifyHostOfRemovedRowRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).notify_host_of_removed_row(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NotifyHostOfRemovedRowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rcdp.DataService/TryAuth" => {
                    #[allow(non_camel_case_types)]
                    struct TryAuthSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::TryAuthRequest>
                    for TryAuthSvc<T> {
                        type Response = super::TryAuthResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TryAuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).try_auth(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TryAuthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: DataService> Clone for DataServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: DataService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataService> tonic::server::NamedService for DataServiceServer<T> {
        const NAME: &'static str = "rcdp.DataService";
    }
}
