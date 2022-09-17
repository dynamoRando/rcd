use crate::cdata::data_service_server::DataServiceServer;
use crate::cdata::*;
use crate::dbi::{InsertPartialDataResult, UpdatePartialDataResult};
use crate::{cdata::data_service_server::DataService, dbi::Dbi};
use chrono::Utc;
use rusqlite::Result;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
/// Implements the `DataService` definition from the protobuff file
pub struct DataServiceImpl {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
    pub db_interface: Option<Dbi>,
}

impl DataServiceImpl {
    fn dbi(self: &Self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }
}

#[tonic::async_trait]
impl DataService for DataServiceImpl {
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

    async fn create_partial_database(
        &self,
        request: Request<CreateDatabaseRequest>,
    ) -> Result<Response<CreateDatabaseResult>, Status> {
        let message = request.into_inner();
        let is_authenticated = authenticate_host(message.authentication.unwrap(), &self.dbi());
        let db_name = message.database_name;
        let mut db_id = String::from("");

        if is_authenticated {
            let result = self.dbi().create_partial_database(&db_name);
            if !result.is_err() {
                db_id = self.dbi().get_db_id(&db_name.as_str());
            }
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let create_db_result = CreateDatabaseResult {
            authentication_result: Some(auth_response),
            is_successful: is_authenticated,
            database_name: db_name,
            result_message: String::from(""),
            database_id: db_id,
        };

        Ok(Response::new(create_db_result))
    }

    async fn create_table_in_database(
        &self,
        request: Request<CreateTableRequest>,
    ) -> Result<Response<CreateTableResult>, Status> {
        let message = request.into_inner();
        let is_authenticated = authenticate_host(message.authentication.unwrap(), &self.dbi());
        let db_name = message.database_name;
        let table_name = message.table_name;
        let table_schema = message.columns;
        let mut table_is_created = false;
        let mut table_id = String::from("");
        let mut db_id = String::from("");

        if is_authenticated {
            let result =
                self.dbi()
                    .create_table_in_partial_database(&db_name, &table_name, table_schema);
            if !result.is_err() {
                table_is_created = true;
                table_id = self.dbi().get_table_id(&db_name, &table_name);
                db_id = self.dbi().get_db_id(&db_name.as_str());
            }
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let create_table_result = CreateTableResult {
            authentication_result: Some(auth_response),
            is_successful: table_is_created,
            database_name: db_name,
            result_message: String::from(""),
            table_id: table_id,
            table_name: table_name,
            database_id: db_id,
        };

        Ok(Response::new(create_table_result))
    }

    async fn insert_command_into_table(
        &self,
        request: Request<InsertDataRequest>,
    ) -> Result<Response<InsertDataResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let message = request.into_inner();
        let is_authenticated = authenticate_host(message.authentication.unwrap(), &self.dbi());
        let db_name = message.database_name;
        let table_name = message.table_name;

        let mut result = InsertPartialDataResult {
            is_successful: false,
            row_id: 0,
            data_hash: 0,
        };

        if is_authenticated {
            let cmd = &message.cmd;

            result = self
                .dbi()
                .insert_data_into_partial_db(&db_name, &table_name, cmd);
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let result = InsertDataResult {
            authentication_result: Some(auth_response),
            is_successful: result.is_successful,
            data_hash: result.data_hash,
            message: String::from(""),
            row_id: result.row_id,
        };

        Ok(Response::new(result))
    }

    async fn update_command_into_table(
        &self,
        request: Request<UpdateDataRequest>,
    ) -> Result<Response<UpdateDataResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let message = request.into_inner();
        let is_authenticated = authenticate_host(message.authentication.unwrap(), &self.dbi());
        let db_name = message.database_name;
        let table_name = message.table_name;
        let where_clause = message.where_clause.clone();

        let mut result = UpdatePartialDataResult {
            is_successful: false,
            row_id: 0,
            data_hash: 0,
        };

        if is_authenticated {
            let cmd = &message.cmd;

            result = self
                .dbi()
                .update_data_into_partial_db(&db_name, &table_name, cmd, &where_clause);
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let result = UpdateDataResult {
            authentication_result: Some(auth_response),
            is_successful: result.is_successful,
            message: String::from(""),
            rows: Vec::new(),
        };

        Ok(Response::new(result))
    }

    async fn delete_command_into_table(
        &self,
        request: Request<DeleteDataRequest>,
    ) -> Result<Response<DeleteDataResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        unimplemented!("not implemented");
    }

    async fn get_row_from_partial_database(
        &self,
        request: Request<GetRowFromPartialDatabaseRequest>,
    ) -> Result<Response<GetRowFromPartialDatabaseResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let message = request.into_inner();
        let is_authenticated = authenticate_host(message.authentication.unwrap(), &self.dbi());

        let mut result_row = Row {
            row_id: 0,
            database_name: message.row_address.as_ref().unwrap().database_name.clone(),
            table_name: message.row_address.as_ref().unwrap().table_name.clone(),
            values: Vec::new(),
            is_remoteable: true,
            remote_metadata: None,
            hash: Vec::new(),
        };

        if is_authenticated {
            let db_name = message.row_address.as_ref().unwrap().database_name.clone();
            let table_name = message.row_address.as_ref().unwrap().table_name.clone();
            let row_id = message.row_address.as_ref().unwrap().row_id;

            result_row = self
                .dbi()
                .get_row_from_partial_database(&db_name, &table_name, row_id);
        }

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        let result = GetRowFromPartialDatabaseResult {
            authentication_result: Some(auth_response),
            is_successful: false,
            result_message: String::from(""),
            row: Some(result_row),
        };

        Ok(Response::new(result))
    }

    async fn save_contract(
        &self,
        request: Request<SaveContractRequest>,
    ) -> Result<Response<SaveContractResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let message = request.into_inner();
        println!("{:?}", &message.message_info.unwrap());

        let contract = message.contract.unwrap().clone();

        let save_is_successful = self.dbi().save_contract(contract);

        let result = SaveContractResult {
            is_saved: save_is_successful,
            error_message: String::from(""),
        };

        Ok(Response::new(result))
    }

    async fn accept_contract(
        &self,
        request: Request<ParticipantAcceptsContractRequest>,
    ) -> Result<Response<ParticipantAcceptsContractResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let message = request.into_inner();
        let debug_message_info = &message.message_info.as_ref().unwrap().clone();

        println!("{:?}", debug_message_info);
        println!("{:?}", &message);

        let participant_message = message.participant.as_ref().unwrap().clone();

        let accepted_participant = self.dbi().get_participant_by_alias(
            &message.database_name,
            &message.participant.as_ref().unwrap().alias,
        );

        let is_successful = self.dbi().update_participant_accepts_contract(
            &message.database_name,
            accepted_participant,
            participant_message,
            &message.contract_version_guid,
        );

        let result = ParticipantAcceptsContractResult {
            contract_acceptance_is_acknowledged: is_successful,
            error_message: String::from(""),
        };

        Ok(Response::new(result))
    }

    async fn update_row_data_hash_for_host(
        &self,
        _request: Request<UpdateRowDataHashForHostRequest>,
    ) -> Result<Response<UpdateRowDataHashForHostResponse>, Status> {
        unimplemented!("not implemented");
    }

    async fn notify_host_of_removed_row(
        &self,
        _request: Request<NotifyHostOfRemovedRowRequest>,
    ) -> Result<Response<NotifyHostOfRemovedRowResponse>, Status> {
        unimplemented!("not implemented");
    }
}

#[tokio::main]
pub async fn start_db_service(
    address_port: &str,
    root_folder: &str,
    database_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = address_port.parse().unwrap();
    // let data_client = DataServiceImpl::default();

    let data_client = DataServiceImpl {
        root_folder: root_folder.to_string(),
        database_name: database_name.to_string(),
        addr_port: address_port.to_string(),
        db_interface: None,
    };

    let data_client_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();

    println!("data client server listening on {}", addr);

    Server::builder()
        .add_service(DataServiceServer::new(data_client))
        .add_service(data_client_service) // Add this
        .serve(addr)
        .await?;

    Ok(())
}

fn authenticate_host(authentication: AuthRequest, dbi: &Dbi) -> bool {
    let mut is_authenticated = false;

    let host_id = authentication.user_name;
    let host_token = authentication.token;

    if crate::rcd_db::verify_host_by_id(&host_id, host_token.to_vec(), dbi) {
        is_authenticated = true;
    }

    if crate::rcd_db::verify_host_by_name(&host_id, host_token.to_vec(), dbi) {
        is_authenticated = true;
    }

    return is_authenticated;
}
