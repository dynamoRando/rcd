#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RcdError {
    pub number: u32,
    pub message: String,
    pub help: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RcdLogEntry {
    pub dt: String,
    pub dt_utc: String,
    pub level: String,
    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetLogsByLastNumberRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub number_of_logs: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetLogsByLastNumberReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub logs: Vec<RcdLogEntry>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetSettingsRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetSettingsReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub settings_json: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCooperativeHostsRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCooperativeHostsReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub hosts: Vec<HostInfoStatus>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDeletesToHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
    pub table_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDeletesToHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDeletesFromHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
    pub table_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDeletesFromHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetUpdatesToHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
    pub table_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetUpdatesToHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetUpdatesFromHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
    pub table_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetUpdatesFromHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct VersionReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub rcdx: String,
    /// ... so on
    pub rcd_core: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct HostInfoStatus {
    pub host: ::core::option::Option<Host>,
    pub last_communcation_utc: String,
    pub status: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct HostInfoReply {
    pub authentication_result: Option<AuthResult>,
    pub host_info: Option<Host>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RevokeReply {
    pub is_successful: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TokenReply {
    pub is_successful: bool,
    pub expiration_utc: String,
    pub jwt: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetActiveContractRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetActiveContractReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub contract: ::core::option::Option<Contract>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetParticipantsRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetParticipantsReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub participants: Vec<ParticipantStatus>,
    pub is_error: bool,
    pub error: Option<RcdError>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ParticipantStatus {
    pub participant: ::core::option::Option<Participant>,
    pub contract_status: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDatabasesRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct GetDatabasesReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub databases: Vec<DatabaseSchema>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AcceptPendingActionRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AcceptPendingActionReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetPendingActionsRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub action: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetPendingActionsReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub pending_statements: Vec<PendingStatement>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct PendingStatement {
    pub row_id: u32,

    pub statement: String,

    pub requested_ts_utc: String,

    pub host_id: String,

    pub action: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SetDataLogTableStatusRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub use_data_log: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SetDataLogTableStatusReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDataLogTableStatusRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDataLogTableStatusReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub use_data_log: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetReadRowIdsRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub where_clause: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetReadRowIdsReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub row_ids: Vec<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDataHashRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetDataHashReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub data_hash: u64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeDeletesToHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeDeletesToHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeUpdatesToHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeUpdatesToHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeDeletesFromHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeDeletesFromHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeUpdatesFromHostBehaviorRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangesUpdatesFromHostBehaviorReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TryAuthAtParticipantRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub participant_id: String,

    pub participant_alias: String,

    pub db_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TryAuthAtPartipantReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeHostStatusRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub host_alias: String,

    pub host_id: String,

    pub status: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeHostStatusReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub status: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GenerateHostInfoRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub host_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GenerateHostInfoReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SendParticipantContractRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub participant_alias: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SendParticipantContractReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_sent: bool,

    pub message: String,
}
/// a message representing the results of a SQL query
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct StatementResultset {
    pub is_error: bool,

    pub result_message: String,

    pub number_of_rows_affected: u64,

    pub rows: Vec<Row>,

    pub execution_error_message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateUserDatabaseRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateUserDatabaseReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_created: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ExecuteReadRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
    pub sql_statement: String,
    pub database_type: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ExecuteReadReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub total_resultsets: u64,
    pub results: Vec<StatementResultset>,
    pub is_error: bool,
    pub error: Option<RcdError>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ExecuteWriteRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
    pub database_name: String,
    pub sql_statement: String,
    pub database_type: u32,
    pub where_clause: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ExecuteWriteReply {
    pub authentication_result: ::core::option::Option<AuthResult>,
    pub is_successful: bool,
    pub total_rows_affected: u32,
    pub is_error: bool,
    pub error: Option<RcdError>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct HasTableRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct HasTableReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub has_table: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GenerateContractRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub host_name: String,

    pub description: String,

    pub database_name: String,

    pub remote_delete_behavior: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GenerateContractReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SetLogicalStoragePolicyRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub policy_mode: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SetLogicalStoragePolicyReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetLogicalStoragePolicyRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetLogicalStoragePolicyReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub policy_mode: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ExecuteCooperativeWriteRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub sql_statement: String,

    pub database_type: u32,

    pub alias: String,

    pub participant_id: String,

    pub where_clause: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ExecuteCooperativeWriteReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub total_rows_affected: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AddParticipantRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub alias: String,

    pub ip4_address: String,

    pub port: u32,

    pub http_addr: String,

    pub http_port: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AddParticipantReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ViewPendingContractsRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ViewPendingContractsReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub contracts: Vec<Contract>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AcceptPendingContractRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub host_alias: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AcceptPendingContractReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RejectPendingContractRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub host_alias: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RejectPendingContractReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EnableCoooperativeFeaturesRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EnableCoooperativeFeaturesReply {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TryAuthRequest {
    pub authentication: ::core::option::Option<AuthRequest>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TryAuthResult {
    pub authentication_result: ::core::option::Option<AuthResult>,
}
/// a message for creating a table in a database
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateTableRequest {
    /// The user requesting the table creation
    pub authentication: ::core::option::Option<AuthRequest>,
    /// The database in which to create the table
    pub database_name: String,
    /// The database GUID in which to create the table
    pub database_guid: String,
    /// The name of the table to create
    pub table_name: String,
    /// a list of columns for the table
    pub columns: Vec<ColumnSchema>,
}
/// a message for describing the result of a CreateTableRequest
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateTableResult {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub database_name: String,

    pub result_message: String,

    pub database_id: String,

    pub table_name: String,

    pub table_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RowInfo {
    pub database_name: String,

    pub table_name: String,

    pub rowid: u32,

    pub data_hash: u64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct InsertDataRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub cmd: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct InsertDataResult {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub data_hash: u64,

    pub message: String,

    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct UpdateDataRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub cmd: String,

    pub where_clause: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct UpdateDataResult {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,

    pub rows: Vec<RowInfo>,
    /// 0 - unknown
    /// 1 - success (overwrite or overwrite with log)
    /// 2 - pending (queue for review)
    /// 3 - ignored (ignore)
    pub update_status: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DeleteDataRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub database_name: String,

    pub table_name: String,

    pub cmd: String,

    pub where_clause: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DeleteDataResult {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub message: String,

    pub rows: Vec<RowInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetRowFromPartialDatabaseRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub row_address: ::core::option::Option<RowParticipantAddress>,

    pub message_info: ::core::option::Option<MessageInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetRowFromPartialDatabaseResult {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub result_message: String,

    pub row: ::core::option::Option<Row>,
}
/// a message from a host to a participant to save a contract
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SaveContractRequest {
    pub contract: ::core::option::Option<Contract>,

    pub message_info: ::core::option::Option<MessageInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SaveContractResult {
    pub is_saved: bool,

    pub error_message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ParticipantAcceptsContractRequest {
    pub participant: ::core::option::Option<Participant>,

    pub contract_guid: String,

    pub contract_version_guid: String,

    pub database_name: String,

    pub message_info: ::core::option::Option<MessageInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ParticipantAcceptsContractResult {
    pub contract_acceptance_is_acknowledged: bool,

    pub error_message: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct UpdateRowDataHashForHostRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub message_info: ::core::option::Option<MessageInfo>,

    pub host_info: ::core::option::Option<Host>,

    pub database_name: String,

    pub database_id: String,

    pub table_name: String,

    pub table_id: u32,

    pub row_id: u32,

    pub updated_hash_value: u64,

    pub is_deleted_at_participant: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct UpdateRowDataHashForHostResponse {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NotifyHostOfRemovedRowRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub message_info: ::core::option::Option<MessageInfo>,

    pub host_info: ::core::option::Option<Host>,

    pub database_name: String,

    pub database_id: String,

    pub table_name: String,

    pub table_id: u32,

    pub row_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NotifyHostOfRemovedRowResponse {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,
}
/// A message for basic online testing
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TestRequest {
    pub request_time_utc: String,

    pub request_origin_url: String,

    pub request_origin_ip4: String,

    pub request_origin_ip6: String,

    pub request_port_number: u32,

    pub request_echo_message: String,
}
/// A message for basic online testing
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TestReply {
    pub reply_time_utc: String,

    pub reply_echo_message: String,

    pub rcdx_version: String,
}
/// a message for general information
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MessageInfo {
    pub is_little_endian: bool,

    pub message_addresses: Vec<String>,

    pub message_generated_time_utc: String,

    pub message_guid: String,
}
/// A message for authentication purposes (note: this is proof of concept, and obviously not secure)
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AuthRequest {
    pub user_name: String,
    pub pw: String,
    pub pw_hash: Vec<u8>,
    pub token: Vec<u8>,
    pub jwt: String,
}
/// A message describing the results of an authentication attempt
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct AuthResult {
    pub user_name: String,
    pub token: String,
    pub is_authenticated: bool,
    pub authentication_message: String,
}
/// A message for creating a user database
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateDatabaseRequest {
    pub authentication: ::core::option::Option<AuthRequest>,

    pub message_info: ::core::option::Option<MessageInfo>,

    pub database_name: String,
}
/// A message describing the results of a CreateDatabaseRequest
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateDatabaseResult {
    pub authentication_result: ::core::option::Option<AuthResult>,

    pub is_successful: bool,

    pub database_name: String,

    pub result_message: String,

    pub database_id: String,
}
/// an object for representing a row in a table. used for returning data
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Row {
    pub database_name: String,

    pub table_name: String,

    pub row_id: u32,

    pub values: Vec<RowValue>,

    pub is_remoteable: bool,

    pub remote_metadata: ::core::option::Option<RowRemoteMetadata>,

    pub hash: Vec<u8>,
}
/// an object for storing values for a row in a table. used for returning data
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct RowValue {
    pub column: ::core::option::Option<ColumnSchema>,

    pub is_null_value: bool,
    /// we send the raw bytes and expect the client to convert the value based on the column type.
    /// note: this value does not include the 4 byte INT length prefix for variable length fields
    /// to ease conversion refer to the Drummersoft.DrummerDB.Common library, in particular the `DbBinaryConvert` class
    pub value: Vec<u8>,

    pub string_value: String,
}
/// describes the data status of the host in relation to the participant
/// if for example the data hash between the host and the participant do not match
/// or if the row was deleted at the participant, but the reference at the host is not
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct RowRemoteMetadata {
    pub is_remote_out_of_sync_with_host: bool,

    pub is_hash_out_of_sync_with_host: bool,

    pub is_remote_deleted: bool,

    pub is_local_deleted: bool,
}
/// a message for describing schema information of a column in a database table
/// see Drummersoft.DrummerDB.Core.Structures.Version.SystemSchemaConstants100 for more information
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct ColumnSchema {
    /// the name of the column. Max length of 50 characters
    pub column_name: String,
    /// The ENUM data type of the column. See DotCommon.SQLColumnType
    pub column_type: u32,
    /// the max or fixed length of the column, if applicable
    pub column_length: u32,
    /// if the column is nullable or not
    pub is_nullable: bool,
    /// the ordinal value of the column, i.e. the order in which the column appears in the table
    pub ordinal: u32,
    /// empty string in a request, populated in a response with the table GUID the column is attached to
    pub table_id: String,
    /// empty string in a request, populated in a response with the column GUID value
    pub column_id: String,
    /// if the column is the primary key of the table. If this is part of a list of columns, it is implied to be a composite primary key
    pub is_primary_key: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Contract {
    /// the unique contract id
    pub contract_guid: String,
    /// a description of the rights in the contract
    pub description: String,
    /// the schema of the entire database
    pub schema: ::core::option::Option<DatabaseSchema>,
    /// a GUID representing the version of the contract
    pub contract_version: String,

    pub host_info: ::core::option::Option<Host>,
    /// the status of the contract, if applicable
    pub status: u32,
}
/// a message representing information about a participant in the system
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Participant {
    pub participant_guid: String,

    pub alias: String,

    pub ip4_address: String,

    pub ip6_address: String,

    pub database_port_number: u32,

    pub token: Vec<u8>,

    pub internal_participant_guid: String,

    pub http_addr: String,

    pub http_port: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Host {
    pub host_guid: String,
    pub host_name: String,
    pub ip4_address: String,
    pub ip6_address: String,
    pub database_port_number: u32,
    pub token: Vec<u8>,
    pub http_addr: String,
    pub http_port: u32,
}
/// a message for describing the schema of a database
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct DatabaseSchema {
    pub database_name: String,
    pub database_id: String,
    pub tables: Vec<TableSchema>,
    pub database_type: u32,
    pub rcd_database_type: u32,
    pub cooperation_enabled: bool,
    pub has_participants: bool,
}
/// a message for describing the schema information of a table in a database
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct TableSchema {
    pub table_name: String,

    pub table_id: String,

    pub database_name: String,

    pub database_id: String,

    pub columns: Vec<ColumnSchema>,
    /// Drummersoft.DrummerDB.Core.Structures.Enum.LogicalStoragePolicy
    pub logical_storage_policy: u32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TransactionInfo {
    pub transaction_batch_id: String,

    pub transaction_mode: u32,
}
/// a message for identifying the location of a row in a partial database
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RowParticipantAddress {
    pub database_name: String,

    pub table_name: String,

    pub row_id: u32,
}
