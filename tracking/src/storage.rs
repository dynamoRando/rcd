use gloo::storage::{SessionStorage, Storage};
use tracking_model::{event::SharkEvent, user::Token};

use crate::logging::log_to_console;

const EVENTS: &str = "shark.key.storage.events";
const AUTH: &str = "shark.key.storage.auth";
const UN: &str = "shark.key.storage.auth.un";
const UID: &str = "shark.key.storage.auth.un.id";
const API: &str = "shark.key.storage.auth.api.id";

pub fn get_events() -> Vec<SharkEvent> {
    let events_json = SessionStorage::get(EVENTS).unwrap_or_else(|_| String::from(""));
    if events_json.is_empty() {
        let x: Vec<SharkEvent> = Vec::new();
        return x;
    }
    let events: Vec<SharkEvent> = serde_json::from_str(&events_json).unwrap();
    events
}

pub fn save_events(events: &Vec<SharkEvent>) {
    let events_json = serde_json::to_string(events).unwrap();
    SessionStorage::set(EVENTS, events_json).expect("failed to set");
}

pub fn add_event(event: SharkEvent) {
    let mut events = get_events();
    events.push(event);
    save_events(&events);
}

pub fn get_last_x_events(x: usize) -> Vec<SharkEvent> {
    let mut events = get_events();

    let message = format!("get_last_x_events: {}", x.to_string());
    log_to_console(&message);

    events.sort_by_key(|e| e.date());
    events.reverse();

    let final_length = if x > events.len() {
        events.len()
    } else {
        x
    };

    let message = format!("get_last_x_events: {}", final_length.to_string());
    log_to_console(&message);

    events.truncate(final_length);

    let message = format!("get_last_x_events: {events:?}");
    log_to_console(&message);

    events
}

pub fn save_token(token: Token) {
    let json = serde_json::to_string(&token).unwrap();
    SessionStorage::set(AUTH, json).expect("failed to set");
}

pub fn save_api_addr(addr: &str) {
    SessionStorage::set(API, addr).expect("failed to set");
}

pub fn get_api_addr() -> String {
    SessionStorage::get(API).unwrap_or_else(|_| String::from("Not Set"))
}

pub fn save_un(un: &str) {
    SessionStorage::set(UN, un).expect("failed to set");
}

pub fn get_un() -> String {
    SessionStorage::get(UN).unwrap_or_else(|_| String::from(""))
}

pub fn save_uid(uid: u32) {
    SessionStorage::set(UID, uid).expect("failed to set");
}

pub fn get_uid() -> u32 {
    SessionStorage::get(UID).unwrap_or_else(|_| 0)
}

pub fn get_token() -> Token {
    let json = SessionStorage::get(AUTH).unwrap_or_else(|_| String::from(""));

    if json.is_empty() {
        Token::default()
    } else {
        let t: Token = serde_json::from_str(&json).unwrap();
        return t;
    }
}
