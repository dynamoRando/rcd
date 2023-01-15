use serde::{Deserialize, Serialize};

/// Represents the kinds of databases in rcd
/// # Kinds
/// - 0 - Unknown
/// - 1 - Rcd database itself
/// - 2 - Host database
/// - 3 - Partial database
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RcdDatabaseType {
    Unknown = 0,
    Rcd = 1,
    Host = 2,
    Partial = 3,
}

impl RcdDatabaseType {
    pub fn from_u32(value: u32) -> RcdDatabaseType {
        match value {
            0 => RcdDatabaseType::Unknown,
            1 => RcdDatabaseType::Rcd,
            2 => RcdDatabaseType::Host,
            3 => RcdDatabaseType::Partial,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(value: RcdDatabaseType) -> u32 {
        match value {
            RcdDatabaseType::Unknown => 0,
            RcdDatabaseType::Rcd => 1,
            RcdDatabaseType::Host => 2,
            RcdDatabaseType::Partial => 3,
        }
    }

    pub fn to_string(value: RcdDatabaseType) -> String {
        match value {
            RcdDatabaseType::Unknown => "Unknown".to_string(),
            RcdDatabaseType::Rcd => "Rcd".to_string(),
            RcdDatabaseType::Host => "Host".to_string(),
            RcdDatabaseType::Partial => "Partial".to_string()
        }
    }

    pub fn from_str(value: &str) -> RcdDatabaseType {
        match value {
            "Unknown" => RcdDatabaseType::Unknown,
            "Rcd" => RcdDatabaseType::Rcd,
            "Host" => RcdDatabaseType::Host,
            "Partial" => RcdDatabaseType::Partial,
            _ => RcdDatabaseType::Unknown,
        }
    }
}
