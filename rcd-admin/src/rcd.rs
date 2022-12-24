
use rcd_messages::client::{AuthRequest};
use serde::Deserialize;
use yew::Properties;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RcdConnection {
    pub http_addr: String,
    pub http_port: u32,
    pub un: String,
    pub pw: String,
}

#[derive(Clone, PartialEq, Deserialize, Properties)]
struct RcdResponse {
    result: String,
}

impl RcdConnection {
    

    
    /// sends an HTTP POST to the specified URL with the rcd-message as JSON, returning JSON
    async fn get_data(&self, endpoint: String, body: String) -> Result<String, String> {
        // let endpoint = format!("{}:{}", self.get_base_address(), endpoint);
        // let client = reqwest::Client::new();

        // let data = client
        //     .post(endpoint)
        //     .body(body)
        //     .send()
        //     .await
        //     .unwrap()
        //     .text()
        //     .await
        //     .unwrap();

        // return Ok(data);

        todo!()
    }

    fn get_base_address(&self) -> String {
        return format!("{}{}{}{}", "http://", self.http_addr, ":", self.http_port);
    }

    fn get_auth(&self) -> AuthRequest {
        return AuthRequest {
            user_name: self.un.clone(),
            pw: self.pw.clone(),
            pw_hash: Vec::new(),
            token: Vec::new(),
        };
    }
}
