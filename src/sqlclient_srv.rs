use crate::cdata::sql_client_server::{SqlClient, SqlClientServer};
use crate::cdata::CreateUserDatabaseReply;
use crate::cdata::{RejectPendingContractReply, RejectPendingContractRequest};
use crate::dbi::Dbi;
use crate::{cdata::*, remote_db_srv};
use chrono::Utc;
use rusqlite::Result;
use tonic::{transport::Server, Request, Response, Status};

mod contract;
mod db;
mod io;
mod logical_storage_policy;
mod participant;

#[derive(Default)]
/// Implements the `SQLClient` definition from the protobuff file
pub struct SqlClientImpl {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
    pub own_db_addr_port: String,
    pub db_interface: Option<Dbi>,
}

impl SqlClientImpl {
    fn verify_login(self: &Self, login: &str, pw: &str) -> bool {
        let dbi = self.db_interface.as_ref().unwrap().clone();
        return crate::rcd_db::verify_login(&login, &pw, &dbi);
    }

    fn dbi(self: &Self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }
}

#[tonic::async_trait]
impl SqlClient for SqlClientImpl {
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let item = request.into_inner().request_echo_message;

        let response = TestReply {
            reply_time_utc: String::from(Utc::now().to_rfc2822()),
            reply_echo_message: String::from(item),
        };
        Ok(Response::new(response))
    }

    async fn generate_host_info(
        &self,
        request: Request<GenerateHostInfoRequest>,
    ) -> Result<Response<GenerateHostInfoReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let generate_host_info_result = db::generate_host_info(request.into_inner(), self).await;
        Ok(Response::new(generate_host_info_result))
    }

    async fn create_user_database(
        &self,
        request: Request<CreateUserDatabaseRequest>,
    ) -> Result<Response<CreateUserDatabaseReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let create_db_result = db::create_user_database(request.into_inner(), self).await;
        Ok(Response::new(create_db_result))
    }

    async fn enable_coooperative_features(
        &self,
        request: Request<EnableCoooperativeFeaturesRequest>,
    ) -> Result<Response<EnableCoooperativeFeaturesReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let enable_cooperative_features_reply =
            db::enable_coooperative_features(request.into_inner(), self).await;
        Ok(Response::new(enable_cooperative_features_reply))
    }

    async fn execute_read(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_read_reply = io::execute_read(request.into_inner(), self).await;
        Ok(Response::new(execute_read_reply))
    }

    async fn execute_write(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_write_reply = io::execute_write(request.into_inner(), self).await;
        Ok(Response::new(execute_write_reply))
    }

    #[allow(unused_assignments)]
    async fn execute_cooperative_write(
        &self,
        request: Request<ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<ExecuteCooperativeWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let execute_write_reply = io::execute_cooperative_write(request.into_inner(), self).await;
        Ok(Response::new(execute_write_reply))
    }

    async fn has_table(
        &self,
        request: Request<HasTableRequest>,
    ) -> Result<Response<HasTableReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let has_table_reply = db::has_table(request.into_inner(), self).await;
        Ok(Response::new(has_table_reply))
    }

    async fn set_logical_storage_policy(
        &self,
        request: Request<SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<SetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let set_policy_reply =
            logical_storage_policy::set_logical_storage_policy(request.into_inner(), self).await;
        Ok(Response::new(set_policy_reply))
    }

    async fn get_logical_storage_policy(
        &self,
        request: Request<GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<GetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let get_policy_reply =
            logical_storage_policy::get_logical_storage_policy(request.into_inner(), self).await;
        Ok(Response::new(get_policy_reply))
    }

    async fn generate_contract(
        &self,
        request: Request<GenerateContractRequest>,
    ) -> Result<Response<GenerateContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let generate_contract_reply = db::generate_contract(request.into_inner(), self).await;
        Ok(Response::new(generate_contract_reply))
    }

    async fn add_participant(
        &self,
        request: Request<AddParticipantRequest>,
    ) -> Result<Response<AddParticipantReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let add_participant_reply = participant::add_participant(request.into_inner(), self).await;
        Ok(Response::new(add_participant_reply))
    }

    async fn send_participant_contract(
        &self,
        request: Request<SendParticipantContractRequest>,
    ) -> Result<Response<SendParticipantContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let send_participant_contract_reply =
            participant::send_participant_contract(request.into_inner(), self).await;
        Ok(Response::new(send_participant_contract_reply))
    }

    async fn review_pending_contracts(
        &self,
        request: Request<ViewPendingContractsRequest>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let review_pending_contracts_reply =
            contract::review_pending_contracts(request.into_inner(), self).await;
        Ok(Response::new(review_pending_contracts_reply))
    }

    async fn accept_pending_contract(
        &self,
        request: Request<AcceptPendingContractRequest>,
    ) -> Result<Response<AcceptPendingContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let accepted_reply = contract::accept_pending_contract(request.into_inner(), self).await;
        Ok(Response::new(accepted_reply))
    }

    #[allow(unused_variables)]
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
        let result = db::change_host_status(request.into_inner(), self).await;
        Ok(Response::new(result))
    }

    async fn try_auth_at_participant(
        &self,
        request: tonic::Request<TryAuthAtParticipantRequest>,
    ) -> Result<tonic::Response<TryAuthAtPartipantReply>, tonic::Status> {
        println!("Request from {:?}", request.remote_addr());

        let message = request.into_inner();
        let own_host_info = self.dbi().rcd_get_host_info();
        let a = message.authentication.unwrap();
        let is_authenticated = self.verify_login(&a.user_name, &a.pw);

        let db_participant = self
            .dbi()
            .get_participant_by_alias(&message.db_name, &message.participant_alias);

        let result = remote_db_srv::try_auth_at_participant(db_participant, &own_host_info).await;

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let response = TryAuthAtPartipantReply {
            authentication_result: Some(auth_response),
            is_successful: result,
            message: String::from(""),
        };

        Ok(Response::new(response))
    }

    #[allow(dead_code, unused_variables)]
    async fn change_updates_from_host_behavior(
        &self,
        request: Request<ChangeUpdatesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangesUpdatesFromHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = db::change_updates_from_host_behavior(request.into_inner(), self).await;
        Ok(Response::new(result))
    }

    #[allow(dead_code, unused_variables)]
    async fn change_deletes_from_host_behavior(
        &self,
        request: Request<ChangeDeletesFromHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesFromHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = db::change_deletes_from_host_behavior(request.into_inner(), self).await;
        Ok(Response::new(result))
    }

    #[allow(dead_code, unused_variables)]
    async fn change_updates_to_host_behavior(
        &self,
        request: Request<ChangeUpdatesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeUpdatesToHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = db::change_updates_to_host_behavior(request.into_inner(), self).await;
        Ok(Response::new(result))
    }

    #[allow(dead_code, unused_variables)]
    async fn change_deletes_to_host_behavior(
        &self,
        request: Request<ChangeDeletesToHostBehaviorRequest>,
    ) -> Result<Response<ChangeDeletesToHostBehaviorReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = db::change_deletes_to_host_behavior(request.into_inner(), self).await;
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
        db_interface: None,
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
