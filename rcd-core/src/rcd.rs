/*

Also known as "core".

This is the Rcd "core" business layer. What was previously defined in the rcd-grpc::sqlclient_srv is intended to be
slowly moved over to a communication ambivalent layer, which is this module.

This 'core' will handle most client actions by way of the defined proto types.

*/

use chrono::Utc;
use rcd_common::defaults;

use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, AuthRequest, AuthResult,
    ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest, ChangeHostStatusReply,
    ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
    ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
    ChangesUpdatesFromHostBehaviorReply, CreateUserDatabaseReply, CreateUserDatabaseRequest,
    EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest, GenerateContractReply,
    GenerateContractRequest, GenerateHostInfoReply, GenerateHostInfoRequest, GetDataHashReply,
    GetDataHashRequest, GetDatabasesReply, GetDatabasesRequest, GetPendingActionsReply,
    GetPendingActionsRequest, GetReadRowIdsReply, GetReadRowIdsRequest, HasTableReply,
    HasTableRequest, TestReply, TestRequest,
};

use crate::comm::RcdRemoteDbClient;
use crate::dbi::Dbi;

mod contract;
mod db;

#[derive(Debug, Clone)]
pub struct Rcd {
    pub db_interface: Option<Dbi>,
    pub remote_client: Option<RcdRemoteDbClient>,
}

impl Rcd {
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

    pub async fn change_host_status(
        &self,
        request: ChangeHostStatusRequest,
    ) -> ChangeHostStatusReply {
        return db::change_host_status(&self, request).await;
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
        return db::accept_pending_action_at_participant(&self, request).await;
    }

    pub async fn has_table(&self, request: HasTableRequest) -> HasTableReply {
        return db::has_table(&self, request).await;
    }

    pub fn is_online(&self, request: TestRequest) -> TestReply {
        let item = request.request_echo_message;

        let response = TestReply {
            reply_time_utc: String::from(Utc::now().to_rfc2822()),
            reply_echo_message: String::from(item),
            rcdx_version: defaults::VERSION.to_string(),
        };
        return response;
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

    fn verify_login(&self, request: AuthRequest) -> (bool, AuthResult) {
        let is_authenticated = self.dbi().verify_login(&request.user_name, &request.pw);

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        return (is_authenticated, auth_response);
    }

    fn dbi(&self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }

    fn remote(&self) -> RcdRemoteDbClient {
        return self.remote_client.as_ref().unwrap().clone();
    }
}
