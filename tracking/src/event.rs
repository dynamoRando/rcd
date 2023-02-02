use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SharkEventType {
    Unknown,
    Spotting,
    StartPeriod,
    EndPeriod,
}

impl SharkEventType {
    pub fn as_string(&self) -> &str {
        match self {
            SharkEventType::EndPeriod => "EndPeriod",
            SharkEventType::StartPeriod => "StartPeriod",
            SharkEventType::Spotting => "Spotting",
            SharkEventType::Unknown => "Unknown",
        }
    }

    pub fn try_parse(value: &str) -> SharkEventType {
        if value == "EndPeriod" {
            return SharkEventType::EndPeriod;
        }

        if value == "StartPeriod" {
            return SharkEventType::StartPeriod;
        }

        if value == "Spotting" {
            return SharkEventType::Spotting;
        }

        SharkEventType::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SharkEvent {
    pub event_date: String,
    pub event_type: SharkEventType,
    pub notes: String,
}
