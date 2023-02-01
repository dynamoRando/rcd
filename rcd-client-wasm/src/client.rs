use crate::token::Token;

use rcd_http_common::url::client::{AUTH_FOR_TOKEN, GET_PARTICIPANTS};
use rcd_messages::client::{AuthRequest, GetParticipantsReply, GetParticipantsRequest, TokenReply};

use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcdClient {
    addr: String,
}

impl RcdClient {
    pub fn new(ip: String, port: u32) -> Self {
        let addr = format!("{}{}{}{}", "http://", ip, ":", port.to_string());
        RcdClient { addr }
    }

    pub async fn get_participants(
        &self,
        authentication: AuthRequest,
        database_name: &str,
    ) -> Option<GetParticipantsReply> {
        let request = GetParticipantsRequest {
            authentication: Some(authentication),
            database_name: database_name.to_string(),
        };

        let body = serde_json::to_string(&request).unwrap();
        let address = &self.addr;
        let url = format!("{address}{GET_PARTICIPANTS}");
        let json_data = post(&url, &body).await;

        if !json_data.is_empty() {
            let reply: GetParticipantsReply = serde_json::from_str(&json_data).unwrap();
            return Some(reply);
        }

        None
    }

    pub async fn auth_for_token(&self, un: &str, pw: &str) -> Option<Token> {
        let auth_request = AuthRequest {
            user_name: un.to_string(),
            pw: pw.to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: String::from(""),
        };

        let body = serde_json::to_string(&auth_request).unwrap();
        let address = &self.addr;
        let url = format!("{address}{AUTH_FOR_TOKEN}");
        let json_data = post(&url, &body).await;

        if !json_data.is_empty() {
            let token_reply: TokenReply = serde_json::from_str(&json_data).unwrap();

            let token: Token = Token {
                jwt: token_reply.jwt,
                jwt_exp: token_reply.expiration_utc,
                addr: address.clone(),
                is_logged_in: true,
            };

            return Some(token);
        }

        None
    }
}

#[wasm_bindgen]
pub async fn post(url: &str, body: &str) -> String {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(body)));

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.text().unwrap()).await.unwrap();
    let json_text = JsValue::as_string(&json).unwrap();

    json_text
}
