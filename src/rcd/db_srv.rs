use chrono::Utc;
use tonic::{transport::Server, Request, Response, Status};
use cdata::data_service_server::DataService;
use cdata::data_service_server::DataServiceServer;

mod cdata {
    include!("../cdata.rs");

    // Add this
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("greeter_descriptor");
}


#[derive(Default)]
pub struct DataServiceImpl {}

#[tonic::async_trait]
impl DataService for DataServiceImpl {
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

    async fn create_partial_database(&self,
        _request: Request<cdata::CreateDatabaseRequest>,
    ) -> Result<Response<cdata::CreateDatabaseResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn create_table_in_database(&self,
        _request: Request<cdata::CreateTableRequest>,
    ) -> Result<Response<cdata::CreateTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn insert_row_into_table(&self,
        _request: Request<cdata::InsertRowRequest>,
    ) -> Result<Response<cdata::InsertRowResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn update_row_in_table(&self,
        _request: Request<cdata::UpdateRowInTableRequest>,
    ) -> Result<Response<cdata::UpdateRowInTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn get_rows_from_table(&self,
        _request: Request<cdata::GetRowsFromTableRequest>,
    ) -> Result<Response<cdata::GetRowsFromTableResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn get_row_from_partial_database(&self,
        _request: Request<cdata::GetRowFromPartialDatabaseRequest>,
    ) -> Result<Response<cdata::GetRowFromPartialDatabaseResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn save_contract(&self,
        _request: Request<cdata::SaveContractRequest>,
    ) -> Result<Response<cdata::SaveContractResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn accept_contract(&self,
        _request: Request<cdata::ParticipantAcceptsContractRequest>,
    ) -> Result<Response<cdata::ParticipantAcceptsContractResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn remove_row_from_partial_database(&self,
        _request: Request<cdata::RemoveRowFromPartialDatabaseRequest>,
    ) -> Result<Response<cdata::RemoveRowFromPartialDatabaseResult>, Status> {
        unimplemented!("not implemented");
    }

    async fn update_row_data_hash_for_host(&self,
        _request: Request<cdata::UpdateRowDataHashForHostRequest>,
    ) -> Result<Response<cdata::UpdateRowDataHashForHostResponse>, Status> {
        unimplemented!("not implemented");
    }

    async fn notify_host_of_removed_row(&self,
        _request: Request<cdata::NotifyHostOfRemovedRowRequest>,
    ) -> Result<Response<cdata::NotifyHostOfRemovedRowResponse>, Status> {
        unimplemented!("not implemented");
    }

}

#[cfg(test)]
#[tokio::main]
pub async fn start_service(address_port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = address_port.parse().unwrap();
    let data_client = DataServiceImpl::default();

    let data_client_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(cdata::FILE_DESCRIPTOR_SET)
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
