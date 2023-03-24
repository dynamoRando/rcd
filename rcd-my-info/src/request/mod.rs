use gloo::{
    net::http::{Method, Request},
    storage::{SessionStorage, Storage},
};
use rcd_client_wasm::{client::RcdClient, token::Token};
use rcd_messages::{
    client::{AuthRequest, DatabaseSchema, ParticipantStatus},
    proxy::request_type::RequestType,
};
use yew::{platform::spawn_local, AttrValue, Callback};

use crate::log::log_to_console;

use self::{proxy::get_proxy, rcd::get_rcd_token};

pub mod proxy;
pub mod rcd;

pub fn post(
    request_type: RequestType,
    request_json: &str,
    callback: Callback<Result<AttrValue, String>>,
) {
    let message = format!("{}{}", "outgoing message: ", request_json);
    log_to_console(message);

    let mut proxy = get_proxy();
    let token = get_rcd_token();

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
