use cdata::sql_client_server::{SqlClient, SqlClientServer};
use cdata::AuthResult;
use cdata::CreateUserDatabaseReply;
use chrono::Utc;
use rusqlite::{Connection, Result};
use std::path::Path;
use tonic::{transport::Server, Request, Response, Status};

#[path = "rcd_db.rs"]
pub mod rcd_db;
#[path = "sqlitedb.rs"]
pub mod sqlitedb;

#[allow(dead_code)]
pub mod cdata {
    include!("../cdata.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("greeter_descriptor");
}

#[derive(Default)]
pub struct SqlClientImpl {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
}

impl SqlClientImpl {
    fn get_rcd_db(self: &Self) -> Connection {
        let db_path = Path::new(&self.root_folder).join(&self.database_name);
        return Connection::open(&db_path).unwrap();
    }
}

#[tonic::async_trait]
impl SqlClient for SqlClientImpl {
    async fn is_online(
        &self,
        request: Request<cdata::TestRequest>,
    ) -> Result<Response<cdata::TestReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let item = request.into_inner().request_echo_message;

        let response = cdata::TestReply {
            reply_time_utc: String::from(Utc::now().to_rfc2822()),
            reply_echo_message: String::from(item),
        };
        Ok(Response::new(response))
    }

    async fn create_user_database(
        &self,
        request: Request<cdata::CreateUserDatabaseRequest>,
    ) -> Result<Response<cdata::CreateUserDatabaseReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let mut is_database_created = false;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;

        if is_authenticated {
            let result = sqlitedb::create_database(&db_name, &self.root_folder);
            if !result.is_err() {
                is_database_created = true;
            }
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let create_db_result = CreateUserDatabaseReply {
            authentication_result: Some(auth_response),
            is_created: is_database_created,
            message: String::from(""),
        };

        Ok(Response::new(create_db_result))
    }

    async fn enable_coooperative_features(
        &self,
        request: Request<cdata::EnableCoooperativeFeaturesRequest>,
    ) -> Result<Response<cdata::EnableCoooperativeFeaturesReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn execute_read(
        &self,
        request: Request<cdata::ExecuteReadRequest>,
    ) -> Result<Response<cdata::ExecuteReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn execute_cooperative_read(
        &self,
        request: Request<cdata::ExecuteCooperativeReadRequest>,
    ) -> Result<Response<cdata::ExecuteCooperativeReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn execute_write(
        &self,
        request: Request<cdata::ExecuteWriteRequest>,
    ) -> Result<Response<cdata::ExecuteWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn execute_cooperative_write(
        &self,
        request: Request<cdata::ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<cdata::ExecuteCooperativeWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn has_table(
        &self,
        request: Request<cdata::HasTableRequest>,
    ) -> Result<Response<cdata::HasTableReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn set_logical_storage_policy(
        &self,
        request: Request<cdata::SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<cdata::SetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn get_logical_storage_policy(
        &self,
        request: Request<cdata::GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<cdata::GetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn generate_contract(
        &self,
        request: Request<cdata::GenerateContractRequest>,
    ) -> Result<Response<cdata::GenerateContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn add_participant(
        &self,
        request: Request<cdata::AddParticipantRequest>,
    ) -> Result<Response<cdata::AddParticipantReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn send_participant_contract(
        &self,
        request: Request<cdata::SendParticipantContractRequest>,
    ) -> Result<Response<cdata::SendParticipantContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn review_pending_contracts(
        &self,
        request: Request<cdata::ViewPendingContractsRequest>,
    ) -> Result<Response<cdata::ViewPendingContractsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn accept_pending_contract(
        &self,
        request: Request<cdata::AcceptPendingContractRequest>,
    ) -> Result<Response<cdata::AcceptPendingContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn reject_pending_contract(
        &self,
        request: Request<cdata::RejectPendingContractRequest>,
    ) -> Result<Response<cdata::RejectPendingContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }
}

#[allow(dead_code)]
#[tokio::main]
pub async fn start_service(
    address_port: &str,
    root_folder: &str,
    database_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // https://betterprogramming.pub/building-a-grpc-server-with-rust-be2c52f0860e
    let addr = address_port.parse().unwrap();

    //let sql_client = SqlClientImpl::default();

    let sql_client = SqlClientImpl {
        root_folder: root_folder.to_string(),
        database_name: database_name.to_string(),
        addr_port: address_port.to_string(),
    };

    let sql_client_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(cdata::FILE_DESCRIPTOR_SET)
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