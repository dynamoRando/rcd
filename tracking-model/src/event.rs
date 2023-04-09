use chrono::NaiveDateTime;
use num_derive::{FromPrimitive, ToPrimitive};
use serde_derive::{Serialize, Deserialize};

extern crate num;

/*

CREATE TABLE IF NOT EXISTS event 
(
    id INT,
    event_date DATETIME,
    notes TEXT,
    user_id INT
);

CREATE TABLE IF NOT EXISTS associated_event
(
    event_id INT,
    event_type INT,
    event_date DATETIME,
    notes TEXT,
    user_id iNT
);

# event

| column_name | type     |
| ----------  | -------- |
| id          | int      |
| event_date  | datetime |
| notes       | text     |
| user_id     | int      |

# associated_event

| column_name | type     |
| ----------  | -------- |
| event_id    | int      |
| event_type  | int      |
| event_date  | datetime |
| user_id     | int      |

*/

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SharkEventReply {
    pub is_logged_in: bool,
    pub events: Vec<SharkEvent>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SharkEvent {
    pub id: u32,
    pub date: String,
    pub notes: Option<String>,
    pub associated_events: Option<Vec<SharkAssociatedEvent>>,
    pub user_id: Option<u32>,
}

impl SharkEvent {
    pub fn date(&self) -> Result<NaiveDateTime, String> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SharkAssociatedEvent {
    pub event_id: u32,
    pub event_type: EventType,
    pub date: String,
    pub notes: Option<String>,
    pub user_id: Option<u32>,
}

impl SharkAssociatedEvent {
    pub fn date(&self) -> Result<NaiveDateTime, String> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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

    pub fn try_parse_from_string(value: &str) -> EventType {
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
