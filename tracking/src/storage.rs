use crate::event::SharkEvent;
use gloo::storage::{SessionStorage, Storage};
const KEY: &str = "shark.key.storage";

pub fn get_events() -> Vec<SharkEvent> {
    let events_json = SessionStorage::get(KEY).unwrap_or_else(|_| String::from(""));
    if events_json.is_empty() {
        let x: Vec<SharkEvent> = Vec::new();
        return x;
    }
    let events: Vec<SharkEvent> = serde_json::from_str(&events_json).unwrap();
    events
}

pub fn save_events(events: &Vec<SharkEvent>) {
    let events_json = serde_json::to_string(events).unwrap();
    SessionStorage::set(KEY, events_json).expect("failed to set");
}

pub fn add_event(event: SharkEvent) {
    let mut events = get_events();
    events.push(event);
    save_events(&events);
}
