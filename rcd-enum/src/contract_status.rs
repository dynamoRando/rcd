/// Determines where status of a contract between a host and a participant.
/// # Types
/// * 0 - Unknown
/// * 1 - NotSent
/// * 2 - Pending
/// * 3 - Accepted
/// * 4 - Rejected
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ContractStatus {
    Unknown = 0,
    NotSent = 1,
    Pending = 2,
    Accepted = 3,
    Rejected = 4,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl ContractStatus {
    pub fn from_i64(value: i64) -> ContractStatus {
        match value {
            0 => ContractStatus::Unknown,
            1 => ContractStatus::NotSent,
            2 => ContractStatus::Pending,
            3 => ContractStatus::Accepted,
            4 => ContractStatus::Rejected,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn from_u32(value: u32) -> ContractStatus {
        match value {
            0 => ContractStatus::Unknown,
            1 => ContractStatus::NotSent,
            2 => ContractStatus::Pending,
            3 => ContractStatus::Accepted,
            4 => ContractStatus::Rejected,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(status: ContractStatus) -> u32 {
        match status {
            ContractStatus::Unknown => 0,
            ContractStatus::NotSent => 1,
            ContractStatus::Pending => 2,
            ContractStatus::Accepted => 3,
            ContractStatus::Rejected => 4,
        }
    }
}
