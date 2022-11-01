use rcd_core::rcd_data::RcdData;
use rcdproto::rcdp::*;
use rcdproto::rcdp::{data_service_server::DataService, data_service_server::DataServiceServer};
use rusqlite::Result;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
/// Implements the `DataService` definition from the protobuff file
pub struct DataServiceImpl {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
    pub core: Option<RcdData>,
}

impl DataServiceImpl {
    fn core(self: &Self) -> RcdData {
        return self.core.as_ref().unwrap().clone();
    }
}

#[tonic::async_trait]
impl DataService for DataServiceImpl {
    async fn is_online(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestReply>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = self.core().is_online(request.into_inner()).await;
        Ok(Response::new(response))
    }

    async fn create_partial_database(
        &self,
        request: Request<CreateDatabaseRequest>,
    ) -> Result<Response<CreateDatabaseResult>, Status> {
        let create_db_result = self
            .core()
            .create_partial_database(request.into_inner())
            .await;

        Ok(Response::new(create_db_result))
    }

    async fn create_table_in_database(
        &self,
        request: Request<CreateTableRequest>,
    ) -> Result<Response<CreateTableResult>, Status> {
        let create_table_result = self
            .core()
            .create_table_in_database(request.into_inner())
            .await;
        Ok(Response::new(create_table_result))
    }

    async fn insert_command_into_table(
        &self,
        request: Request<InsertDataRequest>,
    ) -> Result<Response<InsertDataResult>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let result = self
            .core()
            .insert_command_into_table(request.into_inner())
            .await;
        Ok(Response::new(result))
    }

    async fn update_command_into_table(
        &self,
        request: Request<UpdateDataRequest>,
    ) -> Result<Response<UpdateDataResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let result = self
            .core()
            .update_command_into_table(request.into_inner())
            .await;

        Ok(Response::new(result))
    }

    async fn delete_command_into_table(
        &self,
        request: Request<DeleteDataRequest>,
    ) -> Result<Response<DeleteDataResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let result = self
            .core()
            .delete_command_into_table(request.into_inner())
            .await;

        Ok(Response::new(result))
    }

    async fn get_row_from_partial_database(
        &self,
        request: Request<GetRowFromPartialDatabaseRequest>,
    ) -> Result<Response<GetRowFromPartialDatabaseResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let result = self
            .core()
            .get_row_from_partial_database(request.into_inner())
            .await;

        Ok(Response::new(result))
    }

    async fn save_contract(
        &self,
        request: Request<SaveContractRequest>,
    ) -> Result<Response<SaveContractResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let result = self.core().save_contract(request.into_inner()).await;
        Ok(Response::new(result))
    }

    async fn accept_contract(
        &self,
        request: Request<ParticipantAcceptsContractRequest>,
    ) -> Result<Response<ParticipantAcceptsContractResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let result = self.core().accept_contract(request.into_inner()).await;

        Ok(Response::new(result))
    }

    async fn update_row_data_hash_for_host(
        &self,
        request: Request<UpdateRowDataHashForHostRequest>,
    ) -> Result<Response<UpdateRowDataHashForHostResponse>, Status> {
        println!(
            "update_row_data_hash_for_host: Request from {:?}",
            request.remote_addr()
        );
        println! {"{:?}", request};

        let result = self
            .core()
            .update_row_data_hash_for_host(request.into_inner())
            .await;

        Ok(Response::new(result))
    }

    async fn notify_host_of_removed_row(
        &self,
        request: Request<NotifyHostOfRemovedRowRequest>,
    ) -> Result<Response<NotifyHostOfRemovedRowResponse>, Status> {
        println!(
            "notify_host_of_removed_row: Request from {:?}",
            request.remote_addr()
        );

        let result = self
            .core()
            .notify_host_of_removed_row(request.into_inner())
            .await;

        Ok(Response::new(result))
    }

    async fn try_auth(
        &self,
        request: Request<TryAuthRequest>,
    ) -> Result<Response<TryAuthResult>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let result = self.core().try_auth(request.into_inner()).await;

        Ok(Response::new(result))
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
        core: None,
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
