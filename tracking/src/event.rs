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

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    id: u32,
    date: NaiveDateTime,
    notes: Option<String>,
    associated_events: Option<Vec<AssociatedEvent>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedEvent {
    event_id: u32,
    event_type: EventType,
    date: NaiveDateTime,
    notes: Option<String>,
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
