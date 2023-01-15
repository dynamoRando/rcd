/// Specifies the UpdateStatus in a UpdateDataResult message
/// in rcdp.proto
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PartialDataStatus {
    Unknown = 0,
    SucessOverwriteOrLog = 1,
    Pending = 2,
    Ignored = 3,
}

impl PartialDataStatus {
    pub fn from_u32(value: u32) -> PartialDataStatus {
        match value {
            0 => PartialDataStatus::Unknown,
            1 => PartialDataStatus::SucessOverwriteOrLog,
            2 => PartialDataStatus::Pending,
            3 => PartialDataStatus::Ignored,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(value: PartialDataStatus) -> u32 {
        match value {
            PartialDataStatus::Unknown => 0,
            PartialDataStatus::SucessOverwriteOrLog => 1,
            PartialDataStatus::Pending => 2,
            PartialDataStatus::Ignored => 3,
        }
    }

    pub fn to_string(value: PartialDataStatus) -> String {
        match value {
            PartialDataStatus::Unknown => "Unknown".to_string(),
            PartialDataStatus::SucessOverwriteOrLog => "SucessOverwriteOrLog".to_string(),
            PartialDataStatus::Pending => "Pending".to_string(),
            PartialDataStatus::Ignored => "Ignored".to_string(),
        }
    }

    pub fn from_str(value: &str) -> PartialDataStatus {
        match value {
            "Unknown" => PartialDataStatus::Unknown,
            "SucessOverwriteOrLog" => PartialDataStatus::SucessOverwriteOrLog,
            "Pending" => PartialDataStatus::Pending,
            "Ignored" => PartialDataStatus::Ignored,
            _ => PartialDataStatus::Unknown,
        }
    }
}
