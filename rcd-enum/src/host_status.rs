
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
}