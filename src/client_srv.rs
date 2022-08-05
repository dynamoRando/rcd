use crate::cdata::sql_client_server::{SqlClient, SqlClientServer};
use crate::cdata::AuthResult;
use crate::cdata::CreateUserDatabaseReply;
use crate::{cdata::*, sqlitedb};
use crate::rcd_enum::{LogicalStoragePolicy, RcdGenerateContractError, RemoteDeleteBehavior};
#[allow(unused_imports)]
use crate::sqlitedb::*;
use chrono::Utc;
use conv::{UnwrapOk, ValueFrom};
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

        let result_table = Vec::new();

        let mut statement_result_set = StatementResultset {
            is_error: true,
            result_message: String::from(""),
            number_of_rows_affected: 0,
            rows: result_table,
            execution_error_message: String::from(""),
        };

        if is_authenticated {
            let query_result = crate::sqlitedb::execute_read(&db_name, &self.root_folder, &sql);

            if query_result.is_ok() {
                let result_rows = query_result.unwrap().to_cdata_rows();
                statement_result_set.number_of_rows_affected =
                    u64::value_from(result_rows.len()).unwrap_ok();
                statement_result_set.rows = result_rows;
                statement_result_set.is_error = false;
            } else {
                statement_result_set.execution_error_message =
                    query_result.unwrap_err().to_string();
            }
        }

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
            total_resultsets: 1,
            results: statement_results,
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
            let rows_affected =
                crate::sqlitedb::execute_write(&db_name, &self.root_folder, &statement);
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

        let mut has_table = false;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;
        let table_name = message.table_name;

        if is_authenticated {
            has_table = crate::sqlitedb::has_table_client_service(
                &db_name,
                &self.root_folder,
                table_name.as_str(),
            )
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let has_table_reply = HasTableReply {
            authentication_result: Some(auth_response),
            has_table: has_table,
        };

        Ok(Response::new(has_table_reply))
    }

    async fn set_logical_storage_policy(
        &self,
        request: Request<SetLogicalStoragePolicyRequest>,
    ) -> Result<Response<SetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let mut policy_is_set = false;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;
        let policy_num = message.policy_mode;
        let policy = LogicalStoragePolicy::from_i64(policy_num as i64);
        let table_name = message.table_name;

        if is_authenticated {
            policy_is_set = crate::sqlitedb::set_logical_storage_policy(
                &db_name,
                &self.root_folder,
                table_name,
                policy,
            )
            .unwrap();
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let set_policy_reply = SetLogicalStoragePolicyReply {
            authentication_result: Some(auth_response),
            is_successful: policy_is_set,
            message: String::from(""),
        };

        Ok(Response::new(set_policy_reply))
    }

    async fn get_logical_storage_policy(
        &self,
        request: Request<GetLogicalStoragePolicyRequest>,
    ) -> Result<Response<GetLogicalStoragePolicyReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let mut policy = LogicalStoragePolicy::None;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;
        let table_name = message.table_name;

        if is_authenticated {
            let i_policy = crate::sqlitedb::get_logical_storage_policy(
                &db_name,
                &self.root_folder,
                table_name,
            )
            .unwrap();

            policy = LogicalStoragePolicy::from_i64(i_policy as i64);
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let get_policy_reply = GetLogicalStoragePolicyReply {
            authentication_result: Some(auth_response),
            policy_mode: LogicalStoragePolicy::to_u32(policy),
        };

        Ok(Response::new(get_policy_reply))
    }

    #[allow(unused_variables)]
    async fn generate_contract(
        &self,
        request: Request<GenerateContractRequest>,
    ) -> Result<Response<GenerateContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let mut is_successful = false;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;
        let desc = message.description;
        let i_remote_delete_behavior = message.remote_delete_behavior;
        let host_name = message.host_name;

        let mut reply_message = String::from("");

        if is_authenticated {
            let result = crate::sqlitedb::generate_contract(
                &db_name,
                &self.root_folder,
                &host_name,
                &desc,
                RemoteDeleteBehavior::from_i64(i_remote_delete_behavior as i64),
            );

            match result {
                Ok(r) => is_successful = r,
                Err(e) => {
                    is_successful = false;
                    if let RcdGenerateContractError::NotAllTablesSet(msg) = e {
                        reply_message = msg;
                    }
                }
            }
        };

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let generate_contract_reply = GenerateContractReply {
            authentication_result: Some(auth_response),
            is_successful: is_successful,
            message: reply_message,
        };

        Ok(Response::new(generate_contract_reply))
    }

    async fn add_participant(
        &self,
        request: Request<AddParticipantRequest>,
    ) -> Result<Response<AddParticipantReply>, Status> {
        println!("Request from {:?}", request.remote_addr());
        
        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = crate::rcd_db::verify_login(&a.user_name, &a.pw, &conn);
        let db_name = message.database_name;
        let alias = message.alias;
        let ip4addr = message.ip4_address;
        let db_port: u32 = message.port;

        let reply_message = String::from("");
        let mut is_successful = false;

        if is_authenticated {
            is_successful = sqlitedb::add_participant(&db_name, &self.root_folder, &alias, &ip4addr, db_port);
        };

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let add_participant_reply = AddParticipantReply {
            authentication_result: Some(auth_response),
            is_successful: is_successful,
            message: reply_message,
        };

        Ok(Response::new(add_participant_reply))
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
pub async fn start_client_service(
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
