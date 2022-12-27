use gloo::{
    net::http::{Method, Request},
    storage::{SessionStorage, Storage},
};
use rcd_messages::client::AuthRequest;
use yew::{platform::spawn_local, AttrValue, Callback};

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

pub fn get_auth() -> Option<AuthRequest> {
    let jwt = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));

    if jwt == "" {
        return None;
    } else {
        return Some(AuthRequest {
            user_name: "".to_string(),
            pw: "".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt,
        });
    }
}

pub fn set_auth(jwt: String) {
    SessionStorage::set(KEY, jwt).expect("failed to set");
}
