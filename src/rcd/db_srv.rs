use chrono::Utc;
#[allow(unused_imports)]
use tonic::{transport::Server, Request, Response, Status};
use crate::cdata::data_service_server::DataService;
#[allow(unused_imports)]
use crate::cdata::data_service_server::DataServiceServer;
#[allow(unused_imports)]
use crate::cdata::*;


#[derive(Default)]
pub struct DataServiceImpl {}

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

    async fn create_partial_database(&self,
        _request: Request<CreateDatabaseRequest>,
    ) -> Result<Response<CreateDatabaseResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn create_table_in_database(&self,
        _request: Request<CreateTableRequest>,
    ) -> Result<Response<CreateTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn insert_row_into_table(&self,
        _request: Request<InsertRowRequest>,
    ) -> Result<Response<InsertRowResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn update_row_in_table(&self,
        _request: Request<UpdateRowInTableRequest>,
    ) -> Result<Response<UpdateRowInTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn get_rows_from_table(&self,
        _request: Request<GetRowsFromTableRequest>,
    ) -> Result<Response<GetRowsFromTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn get_row_from_partial_database(&self,
        _request: Request<GetRowFromPartialDatabaseRequest>,
    ) -> Result<Response<GetRowFromPartialDatabaseResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn save_contract(&self,
        _request: Request<SaveContractRequest>,
    ) -> Result<Response<SaveContractResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn accept_contract(&self,
        _request: Request<ParticipantAcceptsContractRequest>,
    ) -> Result<Response<ParticipantAcceptsContractResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn remove_row_from_partial_database(&self,
        _request: Request<RemoveRowFromPartialDatabaseRequest>,
    ) -> Result<Response<RemoveRowFromPartialDatabaseResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn update_row_data_hash_for_host(&self,
        _request: Request<UpdateRowDataHashForHostRequest>,
    ) -> Result<Response<UpdateRowDataHashForHostResponse>, Status> {
        unimplemented!("not implemented");
    }

    async fn notify_host_of_removed_row(&self,
        _request: Request<NotifyHostOfRemovedRowRequest>,
    ) -> Result<Response<NotifyHostOfRemovedRowResponse>, Status> {
        unimplemented!("not implemented");
    }

}

#[allow(dead_code)]
#[cfg(test)]
#[tokio::main]
pub async fn start_service(address_port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = address_port.parse().unwrap();
    let data_client = DataServiceImpl::default();

    let data_client_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
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
