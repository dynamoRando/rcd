use gloo::{
    net::http::{Method, Request},
    storage::{SessionStorage, Storage},
};
use rcd_messages::client::{DatabaseSchema, ParticipantStatus};
use yew::{platform::spawn_local, AttrValue, Callback};

use crate::{token::Token, log::log_to_console};

const KEY: &str = "rcdadmin.key.instance";
const DATABASES: &str = "rcdadmin.key.databases";
const PARTICIPANTS: &str = "rcdadmin.key.participants";

/// sends an HTTP POST to the specified URL with the rcd-message as JSON, returning JSON
pub fn get_data(url: String, body: String, callback: Callback<AttrValue>) {
    let message = format!("{}{}", "outgoing message: ", body);
    log_to_console(message);

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

/// Saves the JWT to Session Storage
pub fn set_token(token: Token) {
    let token = serde_json::to_string(&token).unwrap();
    SessionStorage::set(KEY, token).expect("failed to set");
}

/// Gets the JWT from Session Storage
pub fn get_token() -> Token {
    let token = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    let token: Token = serde_json::from_str(&token.to_string()).unwrap();
    return token;
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
