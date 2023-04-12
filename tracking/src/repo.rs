use crate::logging::log_to_console;
use serde::{Deserialize, Serialize};
use tracking_model::{
    event::SharkEvent,
    user::{Token, User},
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

const REPO_LOCATION: &str = "http://localhost:8020/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {}

impl Repo {
    pub async fn logout(un: &str) -> Result<(), String> {
        log_to_console("login");
        let addr = format!("{}{}", REPO_LOCATION, r#"user/logout"#);

        let u = User {
            un: un.to_string(),
            alias: Some(un.to_string()),
            id: None,
        };

        let ju = serde_json::to_string(&u).unwrap();

        let result_post = Self::post(&addr, &ju).await;

        match result_post {
            Ok(_) => return Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn login(un: &str) -> Result<Token, String> {
        log_to_console("login");
        let addr = format!("{}{}", REPO_LOCATION, r#"user/auth"#);

        let u = User {
            un: un.to_string(),
            alias: Some(un.to_string()),
            id: None,
        };

        let ju = serde_json::to_string(&u).unwrap();

        let result_post = Self::post(&addr, &ju).await;

        match result_post {
            Ok(token) => {
                let t: Token = serde_json::from_str(&token).unwrap();
                return Ok(t);
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn register_user(un: &str, host_id: &str) -> Result<(), String> {
        log_to_console("register user");
        let addr = format!("{}{}", REPO_LOCATION, r#"user/create"#);

        let u = User {
            un: un.to_string(),
            alias: Some(un.to_string()),
            id: Some(host_id.to_string()),
        };

        let ju = serde_json::to_string(&u).unwrap();

        let result_post = Self::post(&addr, &ju).await;
        if let Err(e) = result_post {
            return Err(e);
        }

        Ok(())
    }

    pub async fn get_uid_for_un(un: &str) -> Result<u32, String> {
        log_to_console("getting uid");
        let addr = format!("{}{}", REPO_LOCATION, r#"user/get/id"#);

        let u = User {
            un: un.to_string(),
            alias: None,
            id: None,
        };

        let ju = serde_json::to_string(&u).unwrap();

        let result_get = Self::post(&addr, &ju).await;
        match result_get {
            Ok(result) => {
                log_to_console(&result);
                let u: User = serde_json::from_str(&result).unwrap();
                let uid: u32 = u.id.unwrap().parse().unwrap();
                return Ok(uid);
            }
            Err(e) => {
                log_to_console(&e);
                Err(e)
            }
        }
    }

    pub async fn get_api_version() -> Result<String, String> {
        log_to_console("getting events");
        let addr = format!("{}{}", REPO_LOCATION, r#"version"#);
        let result_get = Self::get(&addr).await;
        match result_get {
            Ok(result) => {
                log_to_console(&result);
                return Ok(result);
            }
            Err(e) => {
                log_to_console(&e);
                Err(e)
            }
        }
    }

    pub async fn get_events() -> Result<Vec<SharkEvent>, String> {
        log_to_console("getting events");
        let addr = format!("{}{}", REPO_LOCATION, r#"events/get"#);
        let result_get = Self::get(&addr).await;
        match result_get {
            Ok(result) => {
                log_to_console(&result);
                let result: Vec<SharkEvent> = serde_json::from_str(&result).unwrap();
                return Ok(result);
            }
            Err(e) => {
                log_to_console(&e);
                Err(e)
            }
        }
    }

    async fn post(url: &str, json_body: &str) -> Result<String, String> {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        opts.body(Some(&JsValue::from_str(json_body)));

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

    async fn get(url: &str) -> Result<String, String> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

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
}
