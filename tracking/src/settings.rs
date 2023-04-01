use crate::{logging::log_to_console, token::Token, SETTINGS_TOML};
use config::Config;
use rcd_messages::proxy::{
    request_type::RequestType,
    server_messages::{
        http::{EXECUTE, REVOKE_TOKEN_URL, TOKEN_URL},
        AuthForTokenReply, AuthForTokenRequest, ExecuteReply, ExecuteRequest,
    },
};
use serde::{de, Deserialize, Serialize};

use gloo::{
    console::__macro::JsValue,
    storage::{SessionStorage, Storage},
};

pub const PROXY: &str = "shark.proxy.settings";
pub const TOKEN: &str = "shark.proxy.token.key";
pub const DB_NAME: &str = "shark.db";

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    address: String,
    account: String,
}

impl Proxy {
    pub fn new(address: &str, account: &str) -> Self {
        Self {
            address: address.to_string(),
            account: account.to_string(),
        }
    }

    pub fn read_from_config(path_to_file: &str) -> Proxy {
        let error_message = format!("{}{}", "Could not find ", SETTINGS_TOML);

        let settings = Config::builder()
            .add_source(config::File::with_name(path_to_file))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .expect(&error_message);

        let addr = settings.get_string(&String::from("address")).unwrap();
        let acc = settings.get_string(&String::from("account")).unwrap();

        Proxy {
            address: addr,
            account: acc,
        }
    }

    pub fn http(&self) -> String {
        format!("{}{}", "http://", self.address)
    }

    pub fn addr(&self) -> String {
        self.address.clone()
    }

    pub fn account(&self) -> String {
        self.account.clone()
    }

    pub fn save_to_session_storage(&self) {
        let json = serde_json::to_string(&self).unwrap();
        log_to_console(&json);
        SessionStorage::set(PROXY, json).expect("failed to set");
    }

    pub fn get_token_from_session_storage() -> Token {
        let token = SessionStorage::get(TOKEN).unwrap_or_else(|_| String::from(""));
        if token.is_empty() {
            Token::new()
        } else {
            let token: Token = serde_json::from_str(&token).unwrap();
            token
        }
    }

    pub fn get_from_session_storage() -> Proxy {
        let json = SessionStorage::get(PROXY).unwrap_or_else(|_| String::from(""));

        if !json.is_empty() {
            let settings: Proxy = serde_json::from_str(&json).unwrap();
            return settings;
        };

        Proxy {
            address: "proxy.home:50040".to_string(),
            account: "shark".to_string(),
        }
    }

    pub async fn logout(&mut self, un: &str) {
        let request = AuthForTokenRequest {
            login: un.to_string(),
            pw: "".to_string(),
        };

        let url = self.get_http_url(REVOKE_TOKEN_URL);
        let result: Result<AuthForTokenReply, String> =
            self.get_http_result_error(url, request).await;
        let debug = format!("{result:?}");
        log_to_console(&debug);
    }

    pub async fn execute_request(
        &mut self,
        request_json: &str,
        request_type: RequestType,
    ) -> Result<String, String> {
        let token = Proxy::get_token_from_session_storage();

        if let Some(_) = token.id {
            let request = ExecuteRequest {
                login: None,
                pw: None,
                jwt: Some(token.jwt.clone()),
                request_type: request_type.into(),
                request_json: request_json.to_string(),
            };

            let url = self.get_http_url(EXECUTE);

            let result: Result<ExecuteReply, String> =
                self.get_http_result_error(url, request).await;

            match result {
                Ok(result) => {
                    if result.execute_success {
                        return Ok(result.reply.unwrap());
                    } else {
                        let message = format!("could not execute: {result:?}");
                        return Err(message);
                    }
                }
                Err(e) => return Err(e),
            }
        } else {
            Err("Host Id not in token".to_string())
        }
    }

    pub async fn execute_request_as<T: de::DeserializeOwned + std::clone::Clone>(
        &mut self,
        request_json: &str,
        request_type: RequestType,
    ) -> Result<T, String> {
        let token = Proxy::get_token_from_session_storage();

        if token.id.is_some() {
            let request = ExecuteRequest {
                login: None,
                pw: None,
                jwt: Some(token.jwt.clone()),
                request_type: request_type.into(),
                request_json: request_json.to_string(),
            };

            let url = self.get_http_url(EXECUTE);

            let result: Result<ExecuteReply, String> =
                self.get_http_result_error(url, request).await;

            match result {
                Ok(result) => {
                    if result.execute_success {
                        return Ok(serde_json::from_str::<T>(&result.reply.unwrap()).unwrap());
                    } else {
                        let message = format!("could not execute: {result:?}");
                        return Err(message);
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            Err("Host Id not in token".to_string())
        }
    }

    pub async fn auth_for_token(&mut self, un: &str, pw: &str) -> Result<Token, String> {
        let request = AuthForTokenRequest {
            login: un.to_string(),
            pw: pw.to_string(),
        };

        let url = self.get_http_url(TOKEN_URL);
        let result: Result<AuthForTokenReply, String> =
            self.get_http_result_error(url, request).await;
        let debug = format!("{result:?}");
        log_to_console(&debug);
        match result {
            Ok(r) => Ok(Token {
                jwt: r.jwt.unwrap(),
                jwt_exp: r.expiration_utc.unwrap(),
                addr: self.address.clone(),
                is_logged_in: true,
                id: r.id,
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
        let address = &self.address;
        let url = format!("{address}/{action_url}");
        url
    }
}

pub mod request {
    use super::Proxy;
    use crate::logging::log_to_console;
    use rcd_messages::proxy::request_type::RequestType;
    use yew::{platform::spawn_local, AttrValue, Callback};

    pub fn post(
        request_type: RequestType,
        request_json: &str,
        callback: Callback<Result<AttrValue, String>>,
    ) {
        let message = format!("{}{}", "outgoing message: ", request_json);
        log_to_console(&message);

        let mut proxy = Proxy::get_from_session_storage();

        let request_json = request_json.clone().to_string();
        if !request_json.is_empty() {
            spawn_local(async move {
                let result = proxy.execute_request(&request_json, request_type).await;

                match result {
                    Ok(result) => callback.emit(Ok(AttrValue::from(result))),
                    Err(e) => callback.emit(Err(e)),
                };
            });
        }
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
