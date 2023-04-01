use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/*

# event

| column_name | type     |
| ----------  | -------- |
| id          | int      |
| event_date  | datetime |
| notes       | text     |

# associated_event

| column_name | type     |
| ----------  | -------- |
| event_id    | int      |
| event_type  | int      |
| event_date  | datetime |
| notes       | text     |

*/

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SharkEvent {
    id: u32,
    date: String,
    notes: Option<String>,
    associated_events: Option<Vec<SharkAssociatedEvent>>,
}

impl SharkEvent {
    pub fn date(&self) -> NaiveDateTime {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SharkAssociatedEvent {
    event_id: u32,
    event_type: EventType,
    date: String,
    notes: Option<String>,
}

impl SharkAssociatedEvent {
    pub fn date(&self) -> NaiveDateTime {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EventType {
    Unknown,
    Spotting,
    End,
    Other,
}

impl EventType {
    pub fn as_string(&self) -> &str {
        match self {
            EventType::Spotting => "Spotting",
            EventType::End => "End",
            EventType::Other => "Other",
            EventType::Unknown => "Unknown",
        }
    }

    pub fn try_parse(value: &str) -> EventType {
        if value == "Spotting" {
            return EventType::Spotting;
        }

        if value == "End" {
            return EventType::End;
        }

        if value == "Other" {
            return EventType::Other;
        }

        EventType::Unknown
    }
}
