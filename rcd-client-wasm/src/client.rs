use crate::token::Token;

use rcd_http_common::url::client::{AUTH_FOR_TOKEN, GET_PARTICIPANTS};
use rcd_messages::client::{AuthRequest, GetParticipantsReply, GetParticipantsRequest, TokenReply};

use serde::{de, Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcdClient {
    addr: String,
}

impl RcdClient {
    pub fn new(ip: String, port: u32) -> Self {
        let addr = format!("{}{}{}{}", "http://", ip, ":", port);
        RcdClient { addr }
    }

    pub async fn get_participants(
        &mut self,
        authentication: AuthRequest,
        database_name: &str,
    ) -> Option<GetParticipantsReply> {
        let request = GetParticipantsRequest {
            authentication: Some(authentication),
            database_name: database_name.to_string(),
        };

        let url = self.get_http_url(GET_PARTICIPANTS);
        let result: GetParticipantsReply = self.get_http_result(url, request).await;

        Some(result)
    }

    pub async fn auth_for_token(&mut self, un: &str, pw: &str) -> Option<Token> {
        let request = AuthRequest {
            user_name: un.to_string(),
            pw: pw.to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: String::from(""),
        };

        let url = self.get_http_url(AUTH_FOR_TOKEN);
        let result: TokenReply = self.get_http_result(url, request).await;

        Some(Token {
            jwt: result.jwt,
            jwt_exp: result.expiration_utc,
            addr: self.addr.clone(),
            is_logged_in: true,
        })
    }

    async fn get_http_result<
        'a,
        'b,
        T: de::DeserializeOwned + std::clone::Clone,
        U: de::DeserializeOwned + serde::Serialize + std::clone::Clone,
    >(
        &mut self,
        url: String,
        request: U,
    ) -> T {
        let body = serde_json::to_string(&request).unwrap();
        let result_json: String = post(&url, &body).await;
        let value: T = serde_json::from_str(&result_json).unwrap();
        value
    }

    async fn get_http_result_error<
        'a,
        'b,
        T: de::DeserializeOwned + std::clone::Clone,
        U: de::DeserializeOwned + serde::Serialize + std::clone::Clone,
    >(
        &mut self,
        url: String,
        request: U,
    ) -> Result<T, String> {
        let body = serde_json::to_string(&request).unwrap();
        let result = post_result(&url, &body).await;

        match result {
            Ok(r) => {
                let value: T = serde_json::from_str(&r).unwrap();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn get_http_url(&self, action_url: &str) -> String {
        let address = &self.addr;
        let url = format!("{address}{action_url}");
        url
    }
}

#[wasm_bindgen]
pub async fn post(url: &str, body: &str) -> String {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(body)));

    let request = Request::new_with_str_and_init(url, &opts).unwrap();

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

    JsValue::as_string(&json).unwrap()
}

pub async fn post_result(url: &str, body: &str) -> Result<String, String> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(body)));

    let request = Request::new_with_str_and_init(url, &opts);

    match request {
        Ok(r) => {
            r.headers().set("Content-Type", "application/json").unwrap();

            let window = web_sys::window().unwrap();
            let resp_value_result = JsFuture::from(window.fetch_with_request(&r)).await;
            match resp_value_result {
                Ok(result) => {
                    assert!(result.is_instance_of::<Response>());
                    let resp: Response = result.dyn_into().unwrap();

                    let json = JsFuture::from(resp.text().unwrap()).await.unwrap();

                    Ok(JsValue::as_string(&json).unwrap())
                }
                Err(e) => {
                    if JsValue::is_string(&e) {
                        Err(JsValue::as_string(&e).unwrap())
                    } else {
                        Err("Unable to connect".to_string())
                    }
                }
            }
        }
        Err(e) => {
            if JsValue::is_string(&e) {
                Err(JsValue::as_string(&e).unwrap())
            } else {
                Err("Unable to connect".to_string())
            }
        }
    }
}
