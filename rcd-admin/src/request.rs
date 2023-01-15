use gloo::{
    net::http::{Method, Request},
    storage::{SessionStorage, Storage},
};
use rcd_messages::client::{DatabaseSchema, ParticipantStatus};
use yew::{platform::spawn_local, AttrValue, Callback};

use crate::{log::log_to_console, token::Token};

const KEY: &str = "rcdadmin.key.instance";
const DATABASES: &str = "rcdadmin.key.databases";
const PARTICIPANTS: &str = "rcdadmin.key.participants";
const STATUS: &str = "rcdadmin.key.status";

/// sends an HTTP POST to the specified URL with the rcd-message as JSON, returning JSON if successful,
/// otherwise a string describing the error that occurred
pub fn post(url: String, body: String, callback: Callback<Result<AttrValue, String>>) {
    let message = format!("{}{}", "outgoing message: ", body);
    log_to_console(message);
    if body != "" {
        spawn_local(async move {
            let result = Request::new(&url)
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await;

            if result.is_ok() {
                let response = result.unwrap().text().await;
                if response.is_ok() {
                    let data = response.unwrap();
                    callback.emit(Ok(AttrValue::from(data)));
                }
            } else {
                let err = result.err().unwrap().to_string();
                callback.emit(Err(err))
            }
        });
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
    if token == "" {
        let token = Token::new();
        return token;
    } else {
        let token: Token = serde_json::from_str(&token.to_string()).unwrap();
        return token;
    }
}

/// Saves the RCD instance's Database Schemas to Session Storage
pub fn set_databases(dbs: Vec<DatabaseSchema>) {
    let dbs_json = serde_json::to_string(&dbs).unwrap();
    SessionStorage::set(DATABASES, dbs_json).expect("failed to set");
}

/// Gets the RCD instance's Database Schemas from Session Storage
pub fn get_databases() -> Vec<DatabaseSchema> {
    let databases = SessionStorage::get(DATABASES).unwrap_or_else(|_| String::from(""));
    let databases: Vec<DatabaseSchema> = serde_json::from_str(&databases).unwrap();
    return databases;
}

/// Saves the RCD databases Participants to Session Storage
pub fn set_participants(participants: Vec<ParticipantStatus>) {
    let participants_json = serde_json::to_string(&participants).unwrap();
    SessionStorage::set(PARTICIPANTS, participants_json).expect("failed to set");
}

/// Gets the RCD databases Participants to Session Storage
pub fn get_participants() -> Vec<ParticipantStatus> {
    let participants = SessionStorage::get(PARTICIPANTS).unwrap_or_else(|_| String::from(""));
    let participants: Vec<ParticipantStatus> = serde_json::from_str(&participants).unwrap();
    return participants;
}

/// updates our status on if we're logged in or not
pub fn update_token_login_status(is_logged_in: bool) {
    let mut token = get_token();
    token.is_logged_in = is_logged_in;
    set_token(token);
}

pub fn set_status(status: String) {
    let date = js_sys::Date::new_0();
    let now = String::from(date.to_locale_time_string("en-US"));
    let message = format!("{}: {}", now, &status.to_string());
    SessionStorage::set(STATUS, message).expect("failed to set");
}

/// Gets the JWT from Session Storage
pub fn get_status() -> String {
    return SessionStorage::get(STATUS).unwrap_or_else(|_| String::from(""));
}

pub fn clear_status() {
    SessionStorage::set(STATUS, "".to_string()).expect("failed to set");
}
