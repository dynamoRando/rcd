#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PartialDataResultAction {
    Unknown = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
}

impl PartialDataResultAction {
    pub fn from_u32(value: u32) -> PartialDataResultAction {
        match value {
            0 => PartialDataResultAction::Unknown,
            1 => PartialDataResultAction::Insert,
            2 => PartialDataResultAction::Update,
            3 => PartialDataResultAction::Delete,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(action: PartialDataResultAction) -> u32 {
        match action {
            PartialDataResultAction::Unknown => 0,
            PartialDataResultAction::Insert => 1,
            PartialDataResultAction::Update => 2,
            PartialDataResultAction::Delete => 3,
        }
    }

    pub fn to_string(value: PartialDataResultAction) -> String {
        match value {
            PartialDataResultAction::Unknown => "Unknown".to_string(),
            PartialDataResultAction::Insert => "Insert".to_string(),
            PartialDataResultAction::Update => "Update".to_string(),
            PartialDataResultAction::Delete => "Delete".to_string(),
        }
    }

    pub fn from_str(value: &str) -> PartialDataResultAction {
        match value {
            "Unknown" => PartialDataResultAction::Unknown,
            "Insert" => PartialDataResultAction::Insert,
            "Update" => PartialDataResultAction::Update,
            "Delete" => PartialDataResultAction::Delete,
            _ => PartialDataResultAction::Update,
        }
    }
}
