use log::debug;
use rcdproto::rcdp::{sql_client_client::SqlClientClient, GetLogsByLastNumberRequest};
use serde::de;
use std::time::Duration;
use thiserror::Error;
use tonic::{transport::Channel, Response, Status};

#[derive(Debug, Clone)]
pub struct GrpcClient {
    addr_port: String,
    timeout_in_sec: u32,
    client: Option<SqlClientClient<Channel>>,
}

#[derive(Error, Debug, PartialEq)]
pub enum GrpcClientError {
    #[error("Could not not parse request: `{0}`")]
    ParseRequestError(String),
    #[error("Error issuing grpc request: `{0}`")]
    GrpcError(String),
}

impl GrpcClient {
    pub async fn new(addr_port: &str, timeout_in_sec: u32) -> Self {
        debug!("GrpcClient addr: {addr_port:?}");

        let client = get_grpc_client(addr_port, timeout_in_sec).await;

        Self {
            addr_port: addr_port.to_string(),
            timeout_in_sec: timeout_in_sec,
            client: Some(client),
        }
    }

    pub async fn get_logs_by_last_number(&self, json: &str) -> Result<String, GrpcClientError> {
        let request = self.input::<GetLogsByLastNumberRequest>(json)?;
        let response = self.get_client().get_logs_by_last_number(request).await;
        self.response(response)
    }

    fn response<T: serde::Serialize + std::clone::Clone>(
        &self,
        response: Result<Response<T>, Status>,
    ) -> Result<String, GrpcClientError> {
        match response {
            Ok(response) => {
                let response = response.into_inner();
                let json = serde_json::to_string(&response).unwrap();
                return Ok(json);
            }
            Err(e) => return Err(GrpcClientError::GrpcError(e.to_string())),
        }
    }

    fn input<T: de::DeserializeOwned + std::clone::Clone>(
        &self,
        json: &str,
    ) -> Result<T, GrpcClientError> {
        let parse_result = serde_json::from_str::<T>(json);

        match parse_result {
            Ok(request) => return Ok(request),
            Err(e) => return Err(GrpcClientError::ParseRequestError(e.to_string())),
        };
    }

    fn get_client(&self) -> SqlClientClient<Channel> {
        self.client.as_ref().unwrap().clone()
    }
}

async fn get_grpc_client(
    grpc_client_addr_port: &str,
    timeout_in_seconds: u32,
) -> SqlClientClient<Channel> {
    debug!("{grpc_client_addr_port:?}");

    let endpoint = tonic::transport::Channel::builder(grpc_client_addr_port.parse().unwrap())
        .timeout(Duration::from_secs(timeout_in_seconds.into()));
    let channel = endpoint.connect().await.unwrap();
    SqlClientClient::new(channel)
}
