/// Determines how a host will respond to a particpant's delete action.
/// # Types
/// * 0 - Unknown
/// * 1 - Ignore - If the host discovers that the particpant has deleted the row, it will take no action.
/// * 2 - AutoDelete - If the host discovers that the participant has deleted the row, it will update the reference row with
/// the delete data then logically delete the row.
/// * 3 - UpdateStatusOnly - If the host discovers that the particpant has deleted the row then update the reference row
/// with the delete data but do not perform a logical delete on the row (Note: The row can still be manually deelted
/// on the host side at a later time.)
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RemoteDeleteBehavior {
    Unknown = 0,
    Ignore = 1,
    AutoDelete = 2,
    UpdateStatusOnly = 3,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl RemoteDeleteBehavior {
    #[allow(dead_code)]
    fn from_i64(value: i64) -> RemoteDeleteBehavior {
        match value {
            0 => RemoteDeleteBehavior::Unknown,
            1 => RemoteDeleteBehavior::Ignore,
            2 => RemoteDeleteBehavior::AutoDelete,
            3 => RemoteDeleteBehavior::UpdateStatusOnly,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

/// Determines where data in table will be stroed.
/// # Types
/// * 0 - None - This is the default and when a database has no participants.
/// * 1 - HostOnly - Data is only kept at the host.
/// * 2 - ParticpantOwned - Data is kept at the participant. Hashes of the data are kept at the host. If the participant
/// changes the data, the hash will no longer match unless the host has configured the table to accept changes.
/// * 3 - UpdateStatusOnly - If the host discovers that the particpant has deleted the row then update the reference row
/// * 4 - Shared - Data is at the host, and changes are automatically pushed to the participant. If data is deleted at the host,
/// it is not automatically deleted at the participant but rather a record marker showing it's been deleted is sent to the
/// participant, which the participant can act on or ignore (note: the marker will still exist.) This is a 'soft' delete
/// at the participant.
/// * 5 - Mirror - This is basically SQL replication - whatever changes happen at the host will automatically be replicated
/// at the participant. Deletes are permanent.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogicalStoragePolicy {
    None = 0,
    HostOnly = 1,
    ParticpantOwned = 2,
    Shared = 3,
    Mirror = 4,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl LogicalStoragePolicy {
    #[allow(dead_code)]
    pub fn from_i64(value: i64) -> LogicalStoragePolicy {
        match value {
            0 => LogicalStoragePolicy::None,
            1 => LogicalStoragePolicy::HostOnly,
            2 => LogicalStoragePolicy::ParticpantOwned,
            3 => LogicalStoragePolicy::Shared,
            4 => LogicalStoragePolicy::Mirror,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

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
    #[allow(dead_code)]
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
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ColumnType {
    Unknown = 0,
    Int = 1,
    Bit = 2,
    Char = 3,
    DateTime = 4,
    Decimal = 5,
    Varchar = 6,
    Binary = 7,
    Varbinary = 8,
    Text = 9
}

impl ColumnType {
    #[allow(dead_code)]
    pub fn try_parse(desc: &str) -> Option<ColumnType> {
        let string_data_type = desc.to_lowercase();

        if string_data_type.len() == 0 {
            return Some(ColumnType::Unknown);
        }

        if string_data_type.contains("int") {
            return Some(ColumnType::Int);
        }

        if string_data_type.contains("bit") {
            return Some(ColumnType::Bit);
        }

        if string_data_type.contains("varchar") {
            return Some(ColumnType::Varchar);
        }

        if string_data_type.contains("char"){
            return Some(ColumnType::Char);
        }

        if string_data_type.contains("datetime") {
            return Some(ColumnType::DateTime);
        }

        if string_data_type.contains("decimal") {
            return Some(ColumnType::Decimal);
        }

        if string_data_type.contains("varbinary") {
            return Some(ColumnType::Varbinary);
        }
 
        if string_data_type.contains("binary") {
            return Some(ColumnType::Binary);
        }

        if string_data_type.contains("text") {
            return Some(ColumnType::Text);
        }
        
        return None;
    }

    #[allow(dead_code)]
    pub fn from_u32(value: u32) -> ColumnType {
        match value {
            0 => ColumnType::Unknown,
            1 => ColumnType::Int,
            2 => ColumnType::Bit,
            3 => ColumnType::Char,
            4 => ColumnType::DateTime,
            5 => ColumnType::Decimal,
            6 => ColumnType::Varchar,
            7 => ColumnType::Binary,
            8 => ColumnType::Varbinary,
            9 => ColumnType::Text,
            _ => panic!("Unknown value: {}", value),
        }
    }

    #[allow(dead_code)]
    pub fn to_u32(col_type: ColumnType) -> u32 {
        match col_type {
            ColumnType::Unknown => 0,
            ColumnType::Int => 1,
            ColumnType::Bit => 2,
            ColumnType::Char => 3,
            ColumnType::DateTime => 4,
            ColumnType::Decimal => 5,
            ColumnType::Varchar => 6,
            ColumnType::Binary => 7,
            ColumnType::Varbinary => 8,
            ColumnType::Text => 9
        }
    }
}