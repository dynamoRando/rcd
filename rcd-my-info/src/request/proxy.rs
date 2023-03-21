use gloo::{
    storage::{SessionStorage, Storage}, console::debug,
};
use rcd_client_wasm::token::Token;
use rcd_messages::proxy::server_messages::{http::{TOKEN_URL, REGISTER_URL}, AuthForTokenRequest, AuthForTokenReply, RegisterLoginRequest, RegisterLoginReply};
use serde::{de, Deserialize, Serialize};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request};
use web_sys::Response;


const RCDPROXY: &str = "rcdmyinfo.key.proxy";
const KEY: &str = "rcdmyinfo.key.rcdproxy.instance";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcdProxy {
    addr: String,
}

impl RcdProxy {
    pub fn new(addr: &str) -> Self {
        let addr = format!("{}{}", "http://", addr);
        Self { addr }
    }

    pub async fn register_account(&mut self, un: &str, pw: &str) -> Result<RegisterLoginReply, String> {
        let request = RegisterLoginRequest {
            login: un.to_string(),
            pw: pw.to_string(),
        };

        let url = self.get_http_url(REGISTER_URL);
        let result: Result<RegisterLoginReply, String> = self.get_http_result_error(url, request).await;

        match result {
            Ok(registration) => Ok(registration),
            Err(e) => Err(e),
        }
    }

    pub async fn auth_for_token(&mut self, un: &str, pw: &str) -> Result<Token, String> {
        let request = AuthForTokenRequest {
            login: un.to_string(),
            pw: pw.to_string(),
        };

        let url = self.get_http_url(TOKEN_URL);
        let result: Result<AuthForTokenReply, String> = self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(Token {
                jwt: r.jwt.unwrap(),
                jwt_exp: r.expiration_utc.unwrap(),
                addr: self.addr.clone(),
                is_logged_in: true,
            }),
            Err(e) => Err(e),
        }
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
        let url = format!("{address}/{action_url}");
        debug!(url.clone());
        url
    }
}

pub fn set_proxy(client: &RcdProxy) {
    let client_json = serde_json::to_string(&client).unwrap();
    SessionStorage::set(RCDPROXY, client_json).expect("failed to set");
}

pub fn get_proxy() -> RcdProxy {
    let client = SessionStorage::get(RCDPROXY).unwrap_or_else(|_| String::from(""));
    if client.is_empty() {
        RcdProxy::new("localhost:0")
    } else {
        let client: RcdProxy = serde_json::from_str(&client).unwrap();
        client
    }
}

/// Saves the JWT to Session Storage
pub fn set_token(token: Token) {
    let token = serde_json::to_string(&token).unwrap();
    SessionStorage::set(KEY, token).expect("failed to set");
}

/// Gets the JWT from Session Storage
pub fn get_token() -> Token {
    let token = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    if token.is_empty() {
        Token::new()
    } else {
        let token: Token = serde_json::from_str(&token).unwrap();
        token
    }
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
                    // let m = format!("{:?}", e);
                    // log_to_console(m);

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
