use std::sync::{Arc, Mutex};

use chrono::NaiveDateTime;
use rcd_messages::{
    client::{ExecuteReadReply, ExecuteReadRequest},
    proxy::request_type::RequestType,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{AttrValue, Callback};

use crate::{
    event::{EventType, SharkAssociatedEvent, SharkEvent},
    logging::log_to_console,
    settings::{request, Proxy, DB_NAME},
};

pub const SQL_GET_EVENTS: &str = "
SELECT 
    id, 
    event_date, 
    notes 
FROM 
    event
;";

pub const SQL_GET_ASSOCIATED_EVENTS: &str = "
SELECT 
    event_id,
    event_type,
    event_date,
    notes
FROM 
    associated_event
;
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {}

impl Repo {
    pub async fn get_events() -> Result<Vec<SharkEvent>, String> {
        log_to_console("getting events");
        let addr = "http://localhost:8020/events/get";
        let result_get = Self::get(addr).await;
        match result_get {
            Ok(result) => {
                log_to_console(&result);
                let result: Vec<SharkEvent> = serde_json::from_str(&result).unwrap();
                return Ok(result);
            }
            Err(e) => {
                log_to_console(&e);
                Err(e)
            },
        }
    }

    pub async fn get(url: &str) -> Result<String, String> {
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
