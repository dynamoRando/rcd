use crate::cdata::sql_client_server::{SqlClient, SqlClientServer};
use crate::cdata::AuthResult;
use crate::cdata::CreateUserDatabaseReply;
#[allow(unused_imports)]
use crate::cdata::{RejectPendingContractReply, RejectPendingContractRequest};
use crate::dbi::Dbi;
use crate::host_info::HostInfo;
use crate::query_parser;
use crate::rcd_enum::DmlType;
#[allow(unused_imports)]
use crate::rcd_enum::{LogicalStoragePolicy, RcdGenerateContractError, RemoteDeleteBehavior};
#[allow(unused_imports)]
use crate::{cdata::*, remote_db_srv};
use chrono::Utc;
use conv::{UnwrapOk, ValueFrom};
use rusqlite::{Connection, Result};
use std::path::Path;
use tonic::{transport::Server, Request, Response, Status};

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
    fn get_rcd_db(self: &Self) -> Connection {
        let db_path = Path::new(&self.root_folder).join(&self.database_name);
        return Connection::open(&db_path).unwrap();
    }

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

    #[allow(dead_code, unused_mut, unused_variables)]
    async fn generate_host_info(
        &self,
        request: Request<GenerateHostInfoRequest>,
    ) -> Result<Response<GenerateHostInfoReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let mut is_generate_successful = false;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let host_name = message.host_name.clone();

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);

        if is_authenticated {
            self.dbi().rcd_generate_host_info(&host_name);
            is_generate_successful = true;
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let generate_host_info_result = GenerateHostInfoReply {
            authentication_result: Some(auth_response),
            is_successful: is_generate_successful,
        };

        Ok(Response::new(generate_host_info_result))
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

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;

        if is_authenticated {
            let result = self.dbi().create_database(&db_name);
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

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;

        if is_authenticated {
            self.dbi().enable_coooperative_features(&db_name);
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

    #[allow(unused_variables, unused_assignments, unreachable_code)]
    async fn execute_read(
        &self,
        request: Request<ExecuteReadRequest>,
    ) -> Result<Response<ExecuteReadReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let conn = self.get_rcd_db();
        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let sql = message.sql_statement;
        let rcd_db_conn = self.get_rcd_db();
        let result_table = Vec::new();

        let mut statement_result_set = StatementResultset {
            is_error: true,
            result_message: String::from(""),
            number_of_rows_affected: 0,
            rows: result_table,
            execution_error_message: String::from(""),
        };

        if is_authenticated {
            if self.dbi().has_cooperative_tables_mock(&db_name, &sql) {
                unimplemented!();
                // we would need to get a list of participants for each of the cooperative tables
                let cooperative_tables = self.dbi().get_cooperative_tables(&db_name, &sql);

                for ct in &cooperative_tables {
                    let participants_for_table =
                        self.dbi().get_participants_for_table(&db_name, ct.as_str());
                    for participant in &participants_for_table {
                        // we would need to get rows for that table from the participant
                        let host_info = HostInfo::get(&self.dbi());
                        let remote_data_result = remote_db_srv::get_row_from_participant(
                            participant.clone(),
                            host_info,
                            &db_name,
                            &ct,
                        );
                        unimplemented!();
                    }
                }

                // and then send a request to each participant for row data that fit the query
                // and finally we would need to assemble those results into a table to be returned
            } else {
                let query_result = self.dbi().execute_read(&db_name, &sql);

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
        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let statement = message.sql_statement;

        if is_authenticated {
            let rows_affected = self.dbi().execute_write(&db_name, &statement);
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

    #[allow(unused_variables, unused_mut)]
    async fn execute_cooperative_write(
        &self,
        request: Request<ExecuteCooperativeWriteRequest>,
    ) -> Result<Response<ExecuteCooperativeWriteReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let mut is_remote_action_successful = false;

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let statement = message.sql_statement;

        if is_authenticated {
            if self.dbi().has_participant(&db_name, &message.alias) {
                let dml_type = query_parser::determine_dml_type(&statement, self.dbi().db_type());
                let db_participant = self
                    .dbi()
                    .get_participant_by_alias(&db_name, &message.alias);
                let host_info = self.dbi().rcd_get_host_info();
                let cmd_table_name = query_parser::get_table_name(&statement, self.dbi().db_type());

                match dml_type {
                    DmlType::Unknown => todo!(),
                    DmlType::Insert => {
                        let remote_insert_result = remote_db_srv::insert_row_at_participant(
                            &db_participant,
                            &host_info,
                            &db_name,
                            &cmd_table_name,
                            &statement,
                        )
                        .await;

                        if remote_insert_result.is_successful {
                            // we need to add the data hash and row id here
                            let data_hash = remote_insert_result.data_hash.clone();
                            let row_id = remote_insert_result.row_id;
                            
                            unimplemented!()
                        }
                    }
                    DmlType::Update => todo!(),
                    DmlType::Delete => todo!(),
                }
                /*
                    we need to determine the type of statement: INSERT/UPDATE/DELETE
                    because this we need to figure out what metadata, if any, we need
                    to save on the host side: either a data hash or row id, etc.

                    once we have determined the type of action, we will call the appropriate
                    method on the data service in remote_db_srv and pass
                    the raw SQL statement onto the participant to take action
                */

                unimplemented!()
            }
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let execute_write_reply = ExecuteWriteReply {
            authentication_result: Some(auth_response),
            is_successful: is_remote_action_successful,
            total_rows_affected: 0,
        };

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

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let table_name = message.table_name;

        if is_authenticated {
            has_table = self.dbi().has_table(&db_name, table_name.as_str())
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

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let policy_num = message.policy_mode;
        let policy = LogicalStoragePolicy::from_i64(policy_num as i64);
        let table_name = message.table_name;

        if is_authenticated {
            policy_is_set = self
                .dbi()
                .set_logical_storage_policy(&db_name, table_name.as_str(), policy)
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

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let table_name = message.table_name;

        if is_authenticated {
            let i_policy = self
                .dbi()
                .get_logical_storage_policy(&db_name, &table_name)
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
        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let desc = message.description;
        let i_remote_delete_behavior = message.remote_delete_behavior;
        let host_name = message.host_name;

        let mut reply_message = String::from("");

        if is_authenticated {
            let result = self.dbi().generate_contract(
                &db_name,
                &host_name,
                &desc,
                RemoteDeleteBehavior::from_u32(i_remote_delete_behavior),
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

        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let alias = message.alias;
        let ip4addr = message.ip4_address;
        let db_port: u32 = message.port;

        let reply_message = String::from("");
        let mut is_successful = false;

        if is_authenticated {
            is_successful = self
                .dbi()
                .add_participant(&db_name, &alias, &ip4addr, db_port);
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

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let db_name = message.database_name;
        let participant_alias = message.participant_alias;

        let reply_message = String::from("");
        let mut is_successful = false;

        if is_authenticated {
            if self.dbi().has_participant(&db_name, &participant_alias) {
                let participant = self
                    .dbi()
                    .get_participant_by_alias(&db_name, &participant_alias);
                let active_contract = self.dbi().get_active_contract(&db_name);
                let db_schema = self.dbi().get_database_schema(&db_name);
                let host_info = HostInfo::get(&self.dbi());
                is_successful = remote_db_srv::send_participant_contract(
                    participant,
                    host_info,
                    active_contract,
                    self.own_db_addr_port.clone(),
                    db_schema,
                )
                .await;
            }
        };

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let send_participant_contract_reply = SendParticipantContractReply {
            authentication_result: Some(auth_response),
            is_sent: is_successful,
            message: reply_message,
        };

        Ok(Response::new(send_participant_contract_reply))
    }

    #[allow(unused_variables, unused_mut)]
    async fn review_pending_contracts(
        &self,
        request: Request<ViewPendingContractsRequest>,
    ) -> Result<Response<ViewPendingContractsReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let is_authenticated = self.verify_login(&a.user_name, &a.pw);

        let mut pending_contracts: Vec<Contract> = Vec::new();

        if is_authenticated {
            pending_contracts = self.dbi().get_pending_contracts();
        };

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let review_pending_contracts_reply = ViewPendingContractsReply {
            authentication_result: Some(auth_response),
            contracts: pending_contracts,
        };

        Ok(Response::new(review_pending_contracts_reply))
    }

    #[allow(dead_code, unused_assignments, unused_variables, unused_mut)]
    async fn accept_pending_contract(
        &self,
        request: Request<AcceptPendingContractRequest>,
    ) -> Result<Response<AcceptPendingContractReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        // check if the user is authenticated
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let is_authenticated = self.verify_login(&a.user_name, &a.pw);
        let mut is_accepted = false;
        let mut return_message = String::from("");

        if is_authenticated {
            // 1 - we need to update the rcd_db record that we are accepting this contract
            // 2 - then we actually need to create the database with the properties of the
            // contract
            // 3 - we need to notify the host that we have accepted the contract

            let contracts = self.dbi().get_pending_contracts();
            let pending_contract = contracts
                .iter()
                .enumerate()
                .filter(|&(i, c)| {
                    c.host_info.as_ref().unwrap().host_name.to_string() == message.host_alias
                })
                .map(|(_, c)| c);

            let param_contract = pending_contract.last().unwrap().clone();

            // 1 - accept the contract
            let is_contract_updated = self.dbi().accept_pending_contract(&message.host_alias);

            // 2 - create the database with the properties of the contract
            // make the database
            let db_is_created = self
                .dbi()
                .create_partial_database_from_contract(&param_contract);

            let self_host_info = self.dbi().rcd_get_host_info();
            // 3 - notify the host that we've accepted the contract
            let is_host_notified = remote_db_srv::notify_host_of_acceptance_of_contract(
                &param_contract,
                &self_host_info,
                self.own_db_addr_port.clone(),
            )
            .await;

            if is_contract_updated && db_is_created && is_host_notified {
                is_accepted = true;
                return_message = String::from("accepted contract successfuly");
            } else if !is_contract_updated {
                return_message = String::from("failed to update contract in rcd db");
            } else if !db_is_created {
                return_message = String::from("failed to to create partial db from contract");
            } else if !is_host_notified {
                return_message = String::from("failed to notify host of acceptance of contract");
            }
        };

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let accepted_reply = AcceptPendingContractReply {
            authentication_result: Some(auth_response),
            is_successful: is_accepted,
            message: return_message,
        };

        Ok(Response::new(accepted_reply))
    }

    #[allow(unused_variables)]
    async fn reject_pending_contract(
        &self,
        request: tonic::Request<RejectPendingContractRequest>,
    ) -> Result<tonic::Response<RejectPendingContractReply>, tonic::Status> {
        unimplemented!();
    }
}

#[allow(dead_code)]
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
