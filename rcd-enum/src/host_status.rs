#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HostStatus {
    Unknown = 0,
    Allow = 1,
    Deny = 2,
}

impl HostStatus {
    pub fn from_u32(value: u32) -> HostStatus {
        match value {
            0 => HostStatus::Unknown,
            1 => HostStatus::Allow,
            2 => HostStatus::Deny,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(value: HostStatus) -> u32 {
        match value {
            HostStatus::Unknown => 0,
            HostStatus::Allow => 1,
            HostStatus::Deny => 2,
        }
    }

    pub fn to_string(value: HostStatus) -> String {
        match value {
            HostStatus::Unknown => "Unknown".to_string(),
            HostStatus::Allow => "Allow".to_string(),
            HostStatus::Deny => "Deny".to_string(),
        }
    }

    pub fn from_str(value: &str) -> HostStatus {
        match value {
            "Unknown" => HostStatus::Unknown,
            "Allow" => HostStatus::Allow,
            "Deny" => HostStatus::Deny,
            _ => HostStatus::Unknown,
        }
    }

    pub fn as_string(self) -> String {
        return HostStatus::to_string(self);
    }
}
