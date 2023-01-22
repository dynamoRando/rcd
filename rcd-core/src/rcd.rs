/*

Also known as "core".

This is the Rcd "core" business layer. What was previously defined in the rcd-grpc::sqlclient_srv is intended to be
slowly moved over to a communication ambivalent layer, which is this module.

This 'core' will handle most client actions by way of the defined proto types.

*/

use chrono::Utc;
use rcd_common::defaults;
use rcd_common::rcd_settings::RcdSettings;
use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, AcceptPendingContractReply,
    AcceptPendingContractRequest, AddParticipantReply, AddParticipantRequest, AuthRequest,
    AuthResult, ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest, ChangeHostStatusReply,
    ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
    ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
    ChangesUpdatesFromHostBehaviorReply, CreateUserDatabaseReply, CreateUserDatabaseRequest,
    EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest,
    ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
    ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest, GenerateContractReply,
    GenerateContractRequest, GenerateHostInfoReply, GenerateHostInfoRequest,
    GetActiveContractReply, GetActiveContractRequest, GetCooperativeHostsReply,
    GetCooperativeHostsRequest, GetDataHashReply, GetDataHashRequest, GetDatabasesReply,
    GetDatabasesRequest, GetDeletesFromHostBehaviorReply, GetDeletesFromHostBehaviorRequest,
    GetDeletesToHostBehaviorReply, GetDeletesToHostBehaviorRequest, GetLogicalStoragePolicyReply,
    GetLogicalStoragePolicyRequest, GetLogsByLastNumberReply, GetLogsByLastNumberRequest,
    GetParticipantsReply, GetParticipantsRequest, GetPendingActionsReply, GetPendingActionsRequest,
    GetReadRowIdsReply, GetReadRowIdsRequest, GetSettingsReply, GetSettingsRequest,
    GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
    GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest, HasTableReply, HasTableRequest,
    HostInfoReply, RcdLogEntry, RevokeReply, SendParticipantContractReply,
    SendParticipantContractRequest, SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest,
    TestReply, TestRequest, TokenReply, TryAuthAtParticipantRequest, TryAuthAtPartipantReply,
    ViewPendingContractsReply, ViewPendingContractsRequest,
};

use crate::comm::RcdRemoteDbClient;
use crate::dbi::Dbi;

mod auth;
mod contract;
mod db;
mod io;
mod logical_storage_policy;
mod participant;

#[derive(Debug, Clone)]
pub struct Rcd {
    pub db_interface: Option<Dbi>,
    pub remote_client: Option<RcdRemoteDbClient>,
    pub settings: Option<RcdSettings>,
}

impl Rcd {
    pub async fn revoke_token(&self, request: AuthRequest) -> RevokeReply {
        return auth::revoke_token(self, request).await;
    }

    pub async fn auth_for_token(&self, request: AuthRequest) -> TokenReply {
        return auth::auth_for_token(self, request).await;
    }

    pub async fn try_auth_at_participant(
        &self,
        request: TryAuthAtParticipantRequest,
    ) -> TryAuthAtPartipantReply {
        return participant::try_auth_at_participant(self, request).await;
    }

    pub async fn get_logical_storage_policy(
        &self,
        request: GetLogicalStoragePolicyRequest,
    ) -> GetLogicalStoragePolicyReply {
        return logical_storage_policy::get_logical_storage_policy(self, request).await;
    }

    pub async fn set_logical_storage_policy(
        &self,
        request: SetLogicalStoragePolicyRequest,
    ) -> SetLogicalStoragePolicyReply {
        return logical_storage_policy::set_logical_storage_policy(self, request).await;
    }

    pub async fn send_participant_contract(
        &self,
        request: SendParticipantContractRequest,
    ) -> SendParticipantContractReply {
        return participant::send_participant_contract(self, request).await;
    }

    pub async fn add_participant(&self, request: AddParticipantRequest) -> AddParticipantReply {
        return participant::add_participant(self, request).await;
    }

    pub async fn execute_write_at_participant(
        &self,
        request: ExecuteWriteRequest,
    ) -> ExecuteWriteReply {
        return io::execute_write_at_participant(self, request).await;
    }

    pub async fn execute_cooperative_write_at_host(
        &self,
        request: ExecuteCooperativeWriteRequest,
    ) -> ExecuteCooperativeWriteReply {
        return io::execute_cooperative_write_at_host(self, request).await;
    }
    pub async fn execute_write_at_host(&self, request: ExecuteWriteRequest) -> ExecuteWriteReply {
        return io::execute_write_at_host(self, request).await;
    }

    /// Attempts to execute a `SELECT` statement
    /// at the specified location against a partial database
    pub async fn execute_read_at_participant(
        &self,
        request: ExecuteReadRequest,
    ) -> ExecuteReadReply {
        return io::execute_read_at_participant(self, request).await;
    }

    /// Attempts to execute a `SELECT` statement
    /// at the specified location against a host database
    pub async fn execute_read_at_host(&self, request: ExecuteReadRequest) -> ExecuteReadReply {
        return io::execute_read_at_host(self, request).await;
    }

    pub async fn review_pending_contracts(
        &self,
        request: ViewPendingContractsRequest,
    ) -> ViewPendingContractsReply {
        return contract::review_pending_contracts(self, request).await;
    }

    pub async fn accept_pending_contract(
        &self,
        request: AcceptPendingContractRequest,
    ) -> AcceptPendingContractReply {
        return contract::accept_pending_contract(self, request).await;
    }

    pub async fn get_data_hash_at_participant(
        &self,
        request: GetDataHashRequest,
    ) -> GetDataHashReply {
        return db::get_data_hash_at_participant(self, request).await;
    }

    pub async fn change_updates_from_host_behavior(
        &self,
        request: ChangeUpdatesFromHostBehaviorRequest,
    ) -> ChangesUpdatesFromHostBehaviorReply {
        return db::change_updates_from_host_behavior(self, request).await;
    }

    pub async fn get_settings(&self, request: GetSettingsRequest) -> GetSettingsReply {
        let auth_result = self.verify_login(request.authentication.unwrap());
        let mut settings: String = String::from("");

        if auth_result.0 {
            settings = serde_json::to_string(&self.settings.as_ref().unwrap()).unwrap();
        }

        GetSettingsReply {
            authentication_result: Some(auth_result.1),
            settings_json: settings,
        }
    }

    pub async fn get_cooperative_hosts(
        &self,
        request: GetCooperativeHostsRequest,
    ) -> GetCooperativeHostsReply {
        return db::get_cooperative_hosts(self, request).await;
    }

    pub async fn get_updates_from_host_behavior(
        &self,
        request: GetUpdatesFromHostBehaviorRequest,
    ) -> GetUpdatesFromHostBehaviorReply {
        return db::get_updates_from_host_behavior(self, request).await;
    }

    pub async fn get_updates_to_host_behavior(
        &self,
        request: GetUpdatesToHostBehaviorRequest,
    ) -> GetUpdatesToHostBehaviorReply {
        return db::get_updates_to_host_behavior(self, request).await;
    }

    pub async fn create_user_database(
        &self,
        request: CreateUserDatabaseRequest,
    ) -> CreateUserDatabaseReply {
        return db::create_user_database(self, request).await;
    }

    pub async fn generate_host_info(
        &self,
        request: GenerateHostInfoRequest,
    ) -> GenerateHostInfoReply {
        return db::generate_host_info(self, request).await;
    }

    pub async fn get_host_info(&self, request: AuthRequest) -> HostInfoReply {
        return db::get_host_info(self, request).await;
    }

    pub async fn change_host_status(
        &self,
        request: ChangeHostStatusRequest,
    ) -> ChangeHostStatusReply {
        return db::change_host_status(self, request).await;
    }

    pub async fn get_pending_actions_at_participant(
        &self,
        request: GetPendingActionsRequest,
    ) -> GetPendingActionsReply {
        return db::get_pending_updates_at_participant(self, request).await;
    }

    pub async fn accept_pending_action_at_participant(
        &self,
        request: AcceptPendingActionRequest,
    ) -> AcceptPendingActionReply {
        return db::accept_pending_action_at_participant(self, request).await;
    }

    pub async fn has_table(&self, request: HasTableRequest) -> HasTableReply {
        return db::has_table(self, request).await;
    }

    pub fn is_online(&self, request: TestRequest) -> TestReply {
        let item = request.request_echo_message;

        println!("is_online, requested echo: {}", item);

        TestReply {
            reply_time_utc: Utc::now().to_rfc2822(),
            reply_echo_message: item,
            rcdx_version: defaults::VERSION.to_string(),
        }
    }

    pub async fn get_data_hash_at_host(&self, request: GetDataHashRequest) -> GetDataHashReply {
        return db::get_data_hash_at_host(self, request).await;
    }

    pub async fn change_deletes_from_host_behavior(
        &self,
        request: ChangeDeletesFromHostBehaviorRequest,
    ) -> ChangeDeletesFromHostBehaviorReply {
        return db::change_deletes_from_host_behavior(self, request).await;
    }

    pub async fn generate_contract(
        &self,
        request: GenerateContractRequest,
    ) -> GenerateContractReply {
        return db::generate_contract(self, request).await;
    }

    pub async fn get_databases(&self, request: GetDatabasesRequest) -> GetDatabasesReply {
        return db::get_databases(self, request).await;
    }

    pub async fn get_participants(&self, request: GetParticipantsRequest) -> GetParticipantsReply {
        return db::get_participants(self, request).await;
    }

    pub async fn get_active_contact(
        &self,
        request: GetActiveContractRequest,
    ) -> GetActiveContractReply {
        return db::get_active_contract(self, request).await;
    }

    pub async fn read_row_id_at_participant(
        &self,
        request: GetReadRowIdsRequest,
    ) -> GetReadRowIdsReply {
        return db::read_row_id_at_participant(self, request).await;
    }

    pub async fn enable_coooperative_features(
        &self,
        request: EnableCoooperativeFeaturesRequest,
    ) -> EnableCoooperativeFeaturesReply {
        return db::enable_coooperative_features(self, request).await;
    }

    pub async fn change_updates_to_host_behavior(
        &self,
        request: ChangeUpdatesToHostBehaviorRequest,
    ) -> ChangeUpdatesToHostBehaviorReply {
        return db::change_updates_to_host_behavior(self, request).await;
    }

    pub async fn change_deletes_to_host_behavior(
        &self,
        request: ChangeDeletesToHostBehaviorRequest,
    ) -> ChangeDeletesToHostBehaviorReply {
        return db::change_deletes_to_host_behavior(self, request).await;
    }

    pub async fn get_deletes_to_host_behavior(
        &self,
        request: GetDeletesToHostBehaviorRequest,
    ) -> GetDeletesToHostBehaviorReply {
        return db::get_deletes_to_host_behavior(self, request).await;
    }

    pub async fn get_deletes_from_host_behavior(
        &self,
        request: GetDeletesFromHostBehaviorRequest,
    ) -> GetDeletesFromHostBehaviorReply {
        return db::get_deletes_from_host_behavior(self, request).await;
    }

    pub async fn get_last_log_entries(
        &self,
        request: GetLogsByLastNumberRequest,
    ) -> GetLogsByLastNumberReply {
        let auth_result = self.verify_login(request.authentication.unwrap());
        let mut entries: Vec<RcdLogEntry> = Vec::new();

        if auth_result.0 {
            let log_entries = self.dbi().get_last_log_entries(request.number_of_logs);

            for entry in &log_entries {
                let x = RcdLogEntry {
                    dt: entry.dt.clone(),
                    dt_utc: entry.dt_utc.clone(),
                    level: entry.level.clone(),
                    message: entry.message.clone(),
                };

                entries.push(x);
            }
        }

        GetLogsByLastNumberReply {
            authentication_result: Some(auth_result.1),
            logs: entries,
        }
    }

    fn verify_login(&self, request: AuthRequest) -> (bool, AuthResult) {
        let is_authenticated: bool;

        if !request.jwt.is_empty() {
            is_authenticated = self.dbi().verify_token(request.jwt);
        } else {
            is_authenticated = self.dbi().verify_login(&request.user_name, &request.pw);
        }

        let auth_response = AuthResult {
            is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        (is_authenticated, auth_response)
    }

    fn dbi(&self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }

    fn remote(&self) -> RcdRemoteDbClient {
        return self.remote_client.as_ref().unwrap().clone();
    }
}
