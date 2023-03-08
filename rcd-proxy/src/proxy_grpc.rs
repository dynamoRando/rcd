use crate::RcdProxy;
use log::debug;

use rcdproto::rcdp::sql_client_server::{SqlClient};
use rcdproto::rcdp::*;
use tonic::{Request, Response, Status};

#[derive(Clone, Debug)]
#[allow(dead_code)]
/// Implements the `SQLClient` definition from the protobuff file
pub struct ProxyClientGrpc {
    root_folder: String,
    database_name: String,
    addr_port: String,
    own_db_addr_port: String,
    proxy: RcdProxy,
}

impl ProxyClientGrpc {
    #[allow(dead_code, unused_variables)]
    pub fn new(
        root_folder: String,
        database_name: String,
        addr_port: String,
        own_db_addr_port: String,
        proxy: RcdProxy,
    ) -> Self {
        Self {
            root_folder,
            database_name,
            addr_port,
            own_db_addr_port,
            proxy,
        }
    }
}

#[tonic::async_trait]
impl SqlClient for ProxyClientGrpc {
    #[allow(dead_code, unused_variables)]
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        // look up in the auth request the value for id
        // then look up in own database the directory for that instance
        // then instatiate the rcd instance and handle the request
        todo!();
    }

    #[allow(dead_code, unused_variables)]
    async fn get_logs_by_last_number(
        &self,
        request: Request<GetLogsByLastNumberRequest>,
    ) -> Result<Response<GetLogsByLastNumberReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_settings(
        &self,
        request: Request<GetSettingsRequest>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_cooperative_hosts(
        &self,
        request: Request<GetCooperativeHostsRequest>,
    ) -> Result<Response<GetCooperativeHostsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_updates_from_host_behavior(
        &self,
        request: Request<GetUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_updates_to_host_behavior(
        &self,
        request: Request<GetUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesToHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_deletes_from_host_behavior(
        &self,
        request: Request<GetDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_deletes_to_host_behavior(
        &self,
        request: Request<GetDeletesToHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesToHostBehaviorReply>, Status> {
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_versions(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<VersionReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        // need to write an HTTP version as well
        todo!()
    }
    #[allow(dead_code, unused_variables)]
    async fn get_host_info(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<HostInfoReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn revoke_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<RevokeReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn auth_for_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<TokenReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_active_contract(
        &self,
        request: Request<GetActiveContractRequest>,
    ) -> Result<Response<GetActiveContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_participants(
        &self,
        request: Request<GetParticipantsRequest>,
    ) -> Result<Response<GetParticipantsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_databases(
        &self,
        request: Request<GetDatabasesRequest>,
    ) -> Result<Response<GetDatabasesReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn accept_pending_action_at_participant(
        &self,
        request: Request<AcceptPendingActionRequest>,
    ) -> Result<Response<AcceptPendingActionReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_pending_actions_at_participant(
        &self,
        request: Request<GetPendingActionsRequest>,
    ) -> Result<Response<GetPendingActionsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]

    async fn get_data_log_table_status_at_participant(
        &self,
        request: Request<GetDataLogTableStatusRequest>,
    ) -> Result<Response<GetDataLogTableStatusReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }
    #[allow(dead_code, unused_variables)]
    async fn set_data_log_table_status_at_participant(
        &self,
        request: Request<SetDataLogTableStatusRequest>,
    ) -> Result<Response<SetDataLogTableStatusReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }
    #[allow(dead_code, unused_variables)]
    async fn generate_host_info(
        &self,
        request: Request<GenerateHostInfoRequest>,
    ) -> Result<Response<GenerateHostInfoReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn create_user_database(
        &self,
        request: Request<CreateUserDatabaseRequest>,
    ) -> Result<Response<CreateUserDatabaseReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn enable_coooperative_features(
        &self,
        request: Request<EnableCoooperativeFeaturesRequest>,
    ) -> Result<Response<EnableCoooperativeFeaturesReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn execute_read_at_host(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn execute_read_at_participant(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn execute_write_at_host(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn execute_write_at_participant(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    #[allow(unused_assignments)]
    async fn execute_cooperative_write_at_host(
        &self,
        request: Request<ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<ExecuteCooperativeWriteReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn has_table(
        &self,
        request: Request<HasTableRequest>,
    ) -> Result<Response<HasTableReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn set_logical_storage_policy(
        &self,
        request: Request<SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<SetLogicalStoragePolicyReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_logical_storage_policy(
        &self,
        request: Request<GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<GetLogicalStoragePolicyReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn generate_contract(
        &self,
        request: Request<GenerateContractRequest>,
    ) -> Result<Response<GenerateContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn add_participant(
        &self,
        request: Request<AddParticipantRequest>,
    ) -> Result<Response<AddParticipantReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn send_participant_contract(
        &self,
        request: Request<SendParticipantContractRequest>,
    ) -> Result<Response<SendParticipantContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn review_pending_contracts(
        &self,
        request: Request<ViewPendingContractsRequest>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn accept_pending_contract(
        &self,
        request: Request<AcceptPendingContractRequest>,
    ) -> Result<Response<AcceptPendingContractReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn reject_pending_contract(
        &self,
        _request: tonic::Request<RejectPendingContractRequest>,
    ) -> Result<tonic::Response<RejectPendingContractReply>, tonic::Status> {
        unimplemented!();
    }
    #[allow(dead_code, unused_variables)]
    async fn change_host_status(
        &self,
        request: tonic::Request<ChangeHostStatusRequest>,
    ) -> Result<tonic::Response<ChangeHostStatusReply>, tonic::Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn try_auth_at_participant(
        &self,
        request: tonic::Request<TryAuthAtParticipantRequest>,
    ) -> Result<tonic::Response<TryAuthAtPartipantReply>, tonic::Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn change_updates_from_host_behavior(
        &self,
        request: Request<ChangeUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangesUpdatesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn change_deletes_from_host_behavior(
        &self,
        request: Request<ChangeDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesFromHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn change_updates_to_host_behavior(
        &self,
        request: Request<ChangeUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeUpdatesToHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn change_deletes_to_host_behavior(
        &self,
        request: Request<ChangeDeletesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesToHostBehaviorReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn read_row_id_at_participant(
        &self,
        request: Request<GetReadRowIdsRequest>,
    ) -> Result<Response<GetReadRowIdsReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_data_hash_at_host(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
    #[allow(dead_code, unused_variables)]
    async fn get_data_hash_at_participant(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        debug!("Request from {:?}", request.remote_addr());
        todo!();
    }
}
