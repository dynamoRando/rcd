use gloo::{
    net::http::{Method, Request},
    storage::{SessionStorage, Storage},
};
use yew::{platform::spawn_local, AttrValue, Callback};

use crate::token::Token;

const KEY: &str = "rcdadmin.key.instance";

/// sends an HTTP POST to the specified URL with the rcd-message as JSON, returning JSON
pub fn get_data(url: String, body: String, callback: Callback<AttrValue>) {
    spawn_local(async move {
        let http_response = Request::new(&url)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        callback.emit(AttrValue::from(http_response));
    });
}

pub fn set_token(token: Token) {
    let token = serde_json::to_string(&token).unwrap();
    SessionStorage::set(KEY, token).expect("failed to set");
}

pub fn get_token() -> Token {
    let token = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    let token: Token = serde_json::from_str(&token.to_string()).unwrap();
    return token;
}
