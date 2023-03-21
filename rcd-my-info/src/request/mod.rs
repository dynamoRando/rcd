use gloo::{
    net::http::{Method, Request},
    storage::{SessionStorage, Storage},
};
use rcd_client_wasm::{client::RcdClient, token::Token};
use rcd_messages::client::{DatabaseSchema, ParticipantStatus};
use yew::{platform::spawn_local, AttrValue, Callback};

use crate::log::log_to_console;


pub mod proxy; 
pub mod rcd;


/// sends an HTTP POST to the specified URL with the rcd-message as JSON, returning JSON if successful,
/// otherwise a string describing the error that occurred
pub fn post(url: String, body: String, callback: Callback<Result<AttrValue, String>>) {
    let message = format!("{}{}", "outgoing message: ", body);
    log_to_console(message);
    if !body.is_empty() {
        spawn_local(async move {
            let result = Request::new(&url)
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await;

            if result.is_ok() {
                let response = result.as_ref().unwrap().text().await;

                if let Ok(data) = response {
                    callback.emit(Ok(AttrValue::from(data)));
                } else {
                    let err = result.err().unwrap().to_string();
                    callback.emit(Err(err))
                }
            }
        });
    }
}