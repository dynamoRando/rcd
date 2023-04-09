use gloo::storage::{SessionStorage, Storage};
use tracking_model::{event::SharkEvent, user::Token};

const EVENTS: &str = "shark.key.storage.events";
const AUTH: &str = "shark.key.storage.auth";
const UN: &str = "shark.key.storage.auth.un";

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

pub fn save_token(token: Token) {
    let json = serde_json::to_string(&token).unwrap();
    SessionStorage::set(AUTH, json).expect("failed to set");
}

pub fn save_un(un: &str) {
    SessionStorage::set(UN, un).expect("failed to set");
}

pub fn get_un() -> String {
    SessionStorage::get(UN).unwrap_or_else(|_| String::from(""))
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
