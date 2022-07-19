use crate::cdata::sql_client_server::{SqlClient, SqlClientServer};
use crate::cdata::AuthResult;
use crate::cdata::CreateUserDatabaseReply;
use crate::cdata::FILE_DESCRIPTOR_SET;
use crate::cdata::*;
#[allow(unused_imports)]
use crate::sqlitedb::*;
use chrono::Utc;
use rusqlite::{Connection, Result};
use std::path::Path;
use tonic::{transport::Server, Request, Response, Status};

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

    async fn create_user_database(
        &self,
        request: Request<CreateUserDatabaseRequest>,
    ) -> Result<Response<CreateUserDatabaseReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let mut is_database_created = false;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;

        if is_authenticated {
            let result = crate::sqlitedb::create_database(&db_name, &self.root_folder);
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
        request: Request<EnableCoooperativeFeaturesRequest>,
    ) -> Result<Response<EnableCoooperativeFeaturesReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;

        if is_authenticated {
            crate::sqlitedb::enable_coooperative_features(&db_name, &self.root_folder);
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let enable_cooperative_features_reply = EnableCoooperativeFeaturesReply {
            authentication_result: Some(auth_response),
            is_successful: true,
            message: String::from(""),
        };

        Ok(Response::new(enable_cooperative_features_reply))
    }

    #[allow(unused_variables, unused_assignments)]
    async fn execute_read(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;
        let sql = message.sql_statement;

        let mut result_table = Vec::new();

        if is_authenticated {
            let table = crate::sqlitedb::execute_read(&db_name, &self.root_folder, &sql);
            result_table = table.to_cdata_rows();
        }

        let statement_result_set = StatementResultset {
            is_error: false,
            result_message: String::from(""),
            number_of_rows_affected: 0,
            rows: result_table,
            execution_error_message: String::from("")
        };

        let mut statement_results = Vec::new();
        statement_results.push(statement_result_set);

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let execute_read_reply = ExecuteReadReply {
            authentication_result: Some(auth_response),
            total_resultsets: 0,
            results: statement_results
        };

        Ok(Response::new(execute_read_reply))
    }

    async fn execute_cooperative_read(
        &self,
        request: Request<ExecuteCooperativeReadRequest>,
    ) -> Result<Response<ExecuteCooperativeReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    #[allow(unused_variables)]
    async fn execute_write(
        &self,
        request: Request<ExecuteWriteRequest>,
    ) -> Result<Response<ExecuteWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let rows_affected: u32 = 0;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;
        let statement = message.sql_statement;

        if is_authenticated {
            // ideally in the future we would inspect the sql statement and determine
            // if the table we were going to affect was a cooperative one and then act accordingly
            // right now, we will just execute the write statement
            let rows_affected =  crate::sqlitedb::execute_write(&db_name, &self.root_folder, &statement);
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let execute_write_reply = ExecuteWriteReply {
            authentication_result: Some(auth_response),
            is_successful: true,
            total_rows_affected: rows_affected,
        };

        Ok(Response::new(execute_write_reply))
    }

    async fn execute_cooperative_write(
        &self,
        request: Request<ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<ExecuteCooperativeWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn has_table(
        &self,
        request: Request<HasTableRequest>,
    ) -> Result<Response<HasTableReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn set_logical_storage_policy(
        &self,
        request: Request<SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<SetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn get_logical_storage_policy(
        &self,
        request: Request<GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<GetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn generate_contract(
        &self,
        request: Request<GenerateContractRequest>,
    ) -> Result<Response<GenerateContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn add_participant(
        &self,
        request: Request<AddParticipantRequest>,
    ) -> Result<Response<AddParticipantReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn send_participant_contract(
        &self,
        request: Request<SendParticipantContractRequest>,
    ) -> Result<Response<SendParticipantContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn review_pending_contracts(
        &self,
        request: Request<ViewPendingContractsRequest>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn accept_pending_contract(
        &self,
        request: Request<AcceptPendingContractRequest>,
    ) -> Result<Response<AcceptPendingContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        unimplemented!("");
    }

    async fn reject_pending_contract(
        &self,
        request: Request<RejectPendingContractRequest>,
    ) -> Result<Response<RejectPendingContractReply>, Status> {
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
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
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
