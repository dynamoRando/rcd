use crate::cdata::data_service_server::DataService;
#[allow(unused_imports)]
use crate::cdata::data_service_server::DataServiceServer;
#[allow(unused_imports)]
use crate::cdata::*;
use crate::rcd_db;
#[allow(unused_imports)]
use crate::sqlitedbpart::*;
use chrono::Utc;
use rusqlite::{Connection, Result};
use std::path::Path;
#[allow(unused_imports)]
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
/// Implements the `DataService` definition from the protobuff file
pub struct DataServiceImpl {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
}

impl DataServiceImpl {
    #[allow(dead_code)]
    fn get_rcd_db(self: &Self) -> Connection {
        let db_path = Path::new(&self.root_folder).join(&self.database_name);
        return Connection::open(&db_path).unwrap();
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

    #[allow(dead_code, unused_variables, unused_assignments)]
    async fn create_partial_database(
        &self,
        request: Request<CreateDatabaseRequest>,
    ) -> Result<Response<CreateDatabaseResult>, Status> {
        let message = request.into_inner();
        let a = message.authentication.unwrap();
        let host_id = a.user_name;
        let host_token = a.token.as_bytes();
        let mut is_authenticated = false;
        let mut is_part_db_created = false;
        let db_name = message.database_name;
        let mut db_id = String::from("");

        if crate::rcd_db::verify_host_by_id(&host_id, host_token.to_vec()) {
            is_authenticated = true;
        }

        if crate::rcd_db::verify_host_by_name(&host_id, host_token.to_vec()) {
            is_authenticated = true;
        }

        if is_authenticated {
            let result = crate::sqlitedbpart::create_partial_database(&db_name, &self.root_folder);
            if !result.is_err() {
                is_part_db_created = true;
                db_id = crate::sqlitedbpart::get_db_id(&db_name.as_str());
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
        let a = message.authentication.unwrap();
        let host_id = a.user_name;
        let host_token = a.token;
        let mut is_authenticated = false;
        let db_name = message.database_name;
        let table_name = message.table_name;
        let table_schema = message.columns;
        let mut table_is_created = false;
        let mut table_id = String::from("");
        let mut db_id = String::from("");

        if crate::rcd_db::verify_host_by_id(&host_id, host_token.to_vec()) {
            is_authenticated = true;
        }

        if crate::rcd_db::verify_host_by_name(&host_id, host_token.to_vec()) {
            is_authenticated = true;
        }

        if is_authenticated {
            let result = crate::sqlitedbpart::create_table_in_partial_database(
                &db_name,
                &self.root_folder,
                &table_name,
                table_schema,
            );
            if !result.is_err() {
                table_is_created = true;
                table_id = crate::sqlitedbpart::get_table_id(&db_name, &table_name);
                db_id = crate::sqlitedbpart::get_db_id(&db_name.as_str());
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

    async fn insert_row_into_table(
        &self,
        _request: Request<InsertRowRequest>,
    ) -> Result<Response<InsertRowResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn update_row_in_table(
        &self,
        _request: Request<UpdateRowInTableRequest>,
    ) -> Result<Response<UpdateRowInTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn get_rows_from_table(
        &self,
        _request: Request<GetRowsFromTableRequest>,
    ) -> Result<Response<GetRowsFromTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn get_row_from_partial_database(
        &self,
        _request: Request<GetRowFromPartialDatabaseRequest>,
    ) -> Result<Response<GetRowFromPartialDatabaseResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn save_contract(
        &self,
        request: Request<SaveContractRequest>,
    ) -> Result<Response<SaveContractResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let message = request.into_inner();
        println!("{:?}", &message.message_info.unwrap());

        let contract = message.contract.unwrap().clone();

        let rcd_db_conn = self.get_rcd_db();
        rcd_db::save_contract(contract, &rcd_db_conn);

        unimplemented!("save contract not implemented");
    }

    async fn accept_contract(
        &self,
        _request: Request<ParticipantAcceptsContractRequest>,
    ) -> Result<Response<ParticipantAcceptsContractResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn remove_row_from_partial_database(
        &self,
        _request: Request<RemoveRowFromPartialDatabaseRequest>,
    ) -> Result<Response<RemoveRowFromPartialDatabaseResult>, Status> {
        unimplemented!("not implemented");
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

#[allow(dead_code)]
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
