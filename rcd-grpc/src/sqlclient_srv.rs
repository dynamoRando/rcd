use rcd_core::rcd::Rcd;
use rcdproto::rcdp::sql_client_server::{SqlClient, SqlClientServer};
use rcdproto::rcdp::*;
use rcdproto::rcdp::{
    CreateUserDatabaseReply, RejectPendingContractReply, RejectPendingContractRequest,
};
use rusqlite::Result;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default, Debug)]
/// Implements the `SQLClient` definition from the protobuff file
pub struct SqlClientImpl {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
    pub own_db_addr_port: String,
    pub core: Option<Rcd>,
}

impl SqlClientImpl {
    fn core(self: &Self) -> &Rcd {
        return self.core.as_ref().unwrap();
    }
}

#[tonic::async_trait]
impl SqlClient for SqlClientImpl {
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = self.core().is_online(request.into_inner());
        Ok(Response::new(response))
    }

    async fn get_cooperative_hosts(
        &self,
        request: Request<GetCooperativeHostsRequest>,
    ) -> Result<Response<GetCooperativeHostsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = self
            .core()
            .get_cooperative_hosts(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_updates_from_host_behavior(
        &self,
        request: Request<GetUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesFromHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = self
            .core()
            .get_updates_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_updates_to_host_behavior(
        &self,
        request: Request<GetUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<GetUpdatesToHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = self
            .core()
            .get_updates_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_deletes_from_host_behavior(
        &self,
        request: Request<GetDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesFromHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = self
            .core()
            .get_deletes_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_deletes_to_host_behavior(
        &self,
        request: Request<GetDeletesToHostBehaviorRequest>,
    ) -> Result<Response<GetDeletesToHostBehaviorReply>, Status> {
        let response = self
            .core()
            .get_deletes_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(response))
    }

    async fn get_versions(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<VersionReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        // need to write an HTTP version as well
        todo!()
    }

    async fn get_host_info(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<HostInfoReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = self.core().get_host_info(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn revoke_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<RevokeReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = self.core().revoke_token(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn auth_for_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<TokenReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self.core().auth_for_token(request.into_inner()).await;
        Ok(Response::new(result))
    }

    async fn get_active_contract(
        &self,
        request: Request<GetActiveContractRequest>,
    ) -> Result<Response<GetActiveContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self.core().get_active_contact(request.into_inner()).await;
        Ok(Response::new(result))
    }

    async fn get_participants(
        &self,
        request: Request<GetParticipantsRequest>,
    ) -> Result<Response<GetParticipantsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self.core().get_participants(request.into_inner()).await;
        Ok(Response::new(result))
    }

    async fn get_databases(
        &self,
        request: Request<GetDatabasesRequest>,
    ) -> Result<Response<GetDatabasesReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self.core().get_databases(request.into_inner()).await;
        Ok(Response::new(result))
    }

    async fn accept_pending_action_at_participant(
        &self,
        request: Request<AcceptPendingActionRequest>,
    ) -> Result<Response<AcceptPendingActionReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .accept_pending_action_at_participant(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    async fn get_pending_actions_at_participant(
        &self,
        request: Request<GetPendingActionsRequest>,
    ) -> Result<Response<GetPendingActionsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let pending_updates = self
            .core()
            .get_pending_actions_at_participant(request.into_inner())
            .await;
        Ok(Response::new(pending_updates))
    }

    async fn get_data_log_table_status_at_participant(
        &self,
        request: Request<GetDataLogTableStatusRequest>,
    ) -> Result<Response<GetDataLogTableStatusReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }

    async fn set_data_log_table_status_at_participant(
        &self,
        request: Request<SetDataLogTableStatusRequest>,
    ) -> Result<Response<SetDataLogTableStatusReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!();
    }

    async fn generate_host_info(
        &self,
        request: Request<GenerateHostInfoRequest>,
    ) -> Result<Response<GenerateHostInfoReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let generate_host_info_result = self.core().generate_host_info(request.into_inner()).await;
        Ok(Response::new(generate_host_info_result))
    }

    async fn create_user_database(
        &self,
        request: Request<CreateUserDatabaseRequest>,
    ) -> Result<Response<CreateUserDatabaseReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let create_db_result = self.core().create_user_database(request.into_inner()).await;
        Ok(Response::new(create_db_result))
    }

    async fn enable_coooperative_features(
        &self,
        request: Request<EnableCoooperativeFeaturesRequest>,
    ) -> Result<Response<EnableCoooperativeFeaturesReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let enable_cooperative_features_reply = self
            .core()
            .enable_coooperative_features(request.into_inner())
            .await;
        Ok(Response::new(enable_cooperative_features_reply))
    }

    async fn execute_read_at_host(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_read_reply = self.core().execute_read_at_host(request.into_inner()).await;
        Ok(Response::new(execute_read_reply))
    }

    async fn execute_read_at_participant(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_read_reply = self
            .core()
            .execute_read_at_participant(request.into_inner())
            .await;
        Ok(Response::new(execute_read_reply))
    }

    async fn execute_write_at_host(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_write_reply = self
            .core()
            .execute_write_at_host(request.into_inner())
            .await;
        Ok(Response::new(execute_write_reply))
    }

    async fn execute_write_at_participant(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_write_reply = self
            .core()
            .execute_write_at_participant(request.into_inner())
            .await;
        Ok(Response::new(execute_write_reply))
    }

    #[allow(unused_assignments)]
    async fn execute_cooperative_write_at_host(
        &self,
        request: Request<ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<ExecuteCooperativeWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_write_reply = self
            .core()
            .execute_cooperative_write_at_host(request.into_inner())
            .await;
        Ok(Response::new(execute_write_reply))
    }

    async fn has_table(
        &self,
        request: Request<HasTableRequest>,
    ) -> Result<Response<HasTableReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let has_table_reply = self.core().has_table(request.into_inner()).await;
        Ok(Response::new(has_table_reply))
    }

    async fn set_logical_storage_policy(
        &self,
        request: Request<SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<SetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let set_policy_reply = self
            .core()
            .set_logical_storage_policy(request.into_inner())
            .await;
        Ok(Response::new(set_policy_reply))
    }

    async fn get_logical_storage_policy(
        &self,
        request: Request<GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<GetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let get_policy_reply = self
            .core()
            .get_logical_storage_policy(request.into_inner())
            .await;
        Ok(Response::new(get_policy_reply))
    }

    async fn generate_contract(
        &self,
        request: Request<GenerateContractRequest>,
    ) -> Result<Response<GenerateContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let generate_contract_reply = self.core().generate_contract(request.into_inner()).await;
        Ok(Response::new(generate_contract_reply))
    }

    async fn add_participant(
        &self,
        request: Request<AddParticipantRequest>,
    ) -> Result<Response<AddParticipantReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let add_participant_reply = self.core().add_participant(request.into_inner()).await;
        Ok(Response::new(add_participant_reply))
    }

    async fn send_participant_contract(
        &self,
        request: Request<SendParticipantContractRequest>,
    ) -> Result<Response<SendParticipantContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let send_participant_contract_reply = self
            .core()
            .send_participant_contract(request.into_inner())
            .await;
        Ok(Response::new(send_participant_contract_reply))
    }

    async fn review_pending_contracts(
        &self,
        request: Request<ViewPendingContractsRequest>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let review_pending_contracts_reply = self
            .core()
            .review_pending_contracts(request.into_inner())
            .await;
        Ok(Response::new(review_pending_contracts_reply))
    }

    async fn accept_pending_contract(
        &self,
        request: Request<AcceptPendingContractRequest>,
    ) -> Result<Response<AcceptPendingContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let accepted_reply = self
            .core()
            .accept_pending_contract(request.into_inner())
            .await;
        Ok(Response::new(accepted_reply))
    }

    async fn reject_pending_contract(
        &self,
        request: tonic::Request<RejectPendingContractRequest>,
    ) -> Result<tonic::Response<RejectPendingContractReply>, tonic::Status> {
        unimplemented!();
    }

    async fn change_host_status(
        &self,
        request: tonic::Request<ChangeHostStatusRequest>,
    ) -> Result<tonic::Response<ChangeHostStatusReply>, tonic::Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self.core().change_host_status(request.into_inner()).await;
        Ok(Response::new(result))
    }

    async fn try_auth_at_participant(
        &self,
        request: tonic::Request<TryAuthAtParticipantRequest>,
    ) -> Result<tonic::Response<TryAuthAtPartipantReply>, tonic::Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = self
            .core()
            .try_auth_at_participant(request.into_inner())
            .await;

        Ok(Response::new(response))
    }

    
    async fn change_updates_from_host_behavior(
        &self,
        request: Request<ChangeUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangesUpdatesFromHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .change_updates_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    
    async fn change_deletes_from_host_behavior(
        &self,
        request: Request<ChangeDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesFromHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .change_deletes_from_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    
    async fn change_updates_to_host_behavior(
        &self,
        request: Request<ChangeUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeUpdatesToHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .change_updates_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    
    async fn change_deletes_to_host_behavior(
        &self,
        request: Request<ChangeDeletesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesToHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .change_deletes_to_host_behavior(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    
    async fn read_row_id_at_participant(
        &self,
        request: Request<GetReadRowIdsRequest>,
    ) -> Result<Response<GetReadRowIdsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .read_row_id_at_participant(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    
    async fn get_data_hash_at_host(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .get_data_hash_at_host(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    
    async fn get_data_hash_at_participant(
        &self,
        request: Request<GetDataHashRequest>,
    ) -> Result<Response<GetDataHashReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .get_data_hash_at_participant(request.into_inner())
            .await;
        Ok(Response::new(result))
    }
}

#[tokio::main]
pub async fn start_client_service(
    address_port: &str,
    root_folder: &str,
    database_name: &str,
    own_db_addr_port: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // https://betterprogramming.pub/building-a-grpc-server-with-rust-be2c52f0860e
    let addr = address_port.parse().unwrap();

    //let sql_client = SqlClientImpl::default();

    let sql_client = SqlClientImpl {
        root_folder: root_folder.to_string(),
        database_name: database_name.to_string(),
        addr_port: address_port.to_string(),
        own_db_addr_port: own_db_addr_port.to_string(),
        core: None,
    };

    let sql_client_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();

    println!("sql client server listening on {}", addr);

    Server::builder()
        .add_service(SqlClientServer::new(sql_client))
        .add_service(sql_client_service) // Add this
        .serve(addr)
        .await?;

    Ok(())
}
