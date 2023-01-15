
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DmlType {
    Unknown = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
    Select = 4,
}

impl DmlType {
    pub fn from_u32(value: u32) -> DmlType {
        match value {
            0 => DmlType::Unknown,
            1 => DmlType::Insert,
            2 => DmlType::Update,
            3 => DmlType::Delete,
            4 => DmlType::Select,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(dml_type: DmlType) -> u32 {
        match dml_type {
            DmlType::Unknown => 0,
            DmlType::Insert => 1,
            DmlType::Update => 2,
            DmlType::Delete => 3,
            DmlType::Select => 4,
        }
    }
}