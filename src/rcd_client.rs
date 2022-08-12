use crate::cdata::CreateUserDatabaseRequest;
use crate::cdata::{sql_client_client::SqlClientClient, AuthRequest};
use log::info;
use std::error::Error;
use tonic::transport::Channel;

#[allow(dead_code)]
pub struct RcdClient {
    addr_port: String,
    user_name: String,
    pw: String,
}

impl RcdClient {
    #[allow(dead_code)]
    pub fn new(addr_port: String, user_name: String, pw: String) -> RcdClient {
        return RcdClient {
            addr_port: addr_port,
            user_name: user_name,
            pw: pw,
        };
    }

    #[allow(dead_code)]
    pub async fn create_user_database(self: &Self, db_name: &str) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(CreateUserDatabaseRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        });

        info!("sending request");

        let mut client = self.get_client().await;

        let response = client
            .create_user_database(request)
            .await
            .unwrap()
            .into_inner();
        println!("RESPONSE={:?}", response);
        info!("response back");

        Ok(response.is_created)
    }

    async fn get_client(self: &Self) -> SqlClientClient<Channel> {
        let endpoint = tonic::transport::Channel::builder(self.addr_port.parse().unwrap());
        let channel = endpoint.connect().await.unwrap();
        return SqlClientClient::new(channel);
    }

    #[allow(dead_code)]
    fn gen_auth_request(&self) -> AuthRequest {
        let auth = AuthRequest {
            user_name: self.user_name.clone(),
            pw: self.pw.clone(),
            pw_hash: Vec::new(),
            token: Vec::new(),
        };

        return auth;
    }
}
