use std::{error::Error, fmt};

/// Determines how a host will respond to a particpant's delete action.
/// # Types
/// * 0 - Unknown
/// * 1 - Ignore - If the host discovers that the particpant has deleted the row, it will take no action.
/// * 2 - AutoDelete - If the host discovers that the participant has deleted the row, it will update the reference row with
/// the delete data then logically delete the row.
/// * 3 - UpdateStatusOnly - If the host discovers that the particpant has deleted the row then update the reference row
/// with the delete data but do not perform a logical delete on the row (Note: The row can still be manually deleted
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
    pub fn from_i64(value: i64) -> RemoteDeleteBehavior {
        match value {
            0 => RemoteDeleteBehavior::Unknown,
            1 => RemoteDeleteBehavior::Ignore,
            2 => RemoteDeleteBehavior::AutoDelete,
            3 => RemoteDeleteBehavior::UpdateStatusOnly,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn from_u32(value: u32) -> RemoteDeleteBehavior {
        match value {
            0 => RemoteDeleteBehavior::Unknown,
            1 => RemoteDeleteBehavior::Ignore,
            2 => RemoteDeleteBehavior::AutoDelete,
            3 => RemoteDeleteBehavior::UpdateStatusOnly,
            _ => panic!("Unknown value: {}", value),
        }
    }

    #[allow(dead_code)]
    pub fn to_u32(behavior: RemoteDeleteBehavior) -> u32 {
        match behavior {
            RemoteDeleteBehavior::Unknown => 0,
            RemoteDeleteBehavior::Ignore => 1,
            RemoteDeleteBehavior::AutoDelete => 2,
            RemoteDeleteBehavior::UpdateStatusOnly => 3,
        }
    }
}

/// Determines where data in table will be stored.
/// # Types
/// * 0 - None - This is the default and when a database has no participants.
/// * 1 - HostOnly - Data is only kept at the host.
/// * 2 - ParticpantOwned - Data is kept at the participant. Hashes of the data are kept at the host. If the participant
/// changes the data, the hash will no longer match unless the host has configured the table to accept changes.
/// * 3 - Shared - Data is at the host, and changes are automatically pushed to the participant. If data is deleted at the host,
/// it is not automatically deleted at the participant but rather a record marker showing it's been deleted is sent to the
/// participant, which the participant can act on or ignore (note: the marker will still exist.) This is a 'soft' delete
/// at the participant.
/// * 4 - Mirror - This is basically SQL replication - whatever changes happen at the host will automatically be replicated
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

    pub fn from_u32(value: u32) -> LogicalStoragePolicy {
        match value {
            0 => LogicalStoragePolicy::None,
            1 => LogicalStoragePolicy::HostOnly,
            2 => LogicalStoragePolicy::ParticpantOwned,
            3 => LogicalStoragePolicy::Shared,
            4 => LogicalStoragePolicy::Mirror,
            _ => panic!("Unknown value: {}", value),
        }
    }

    #[allow(dead_code)]
    pub fn to_u32(policy: LogicalStoragePolicy) -> u32 {
        match policy {
            LogicalStoragePolicy::None => 0,
            LogicalStoragePolicy::HostOnly => 1,
            LogicalStoragePolicy::ParticpantOwned => 2,
            LogicalStoragePolicy::Shared => 3,
            LogicalStoragePolicy::Mirror => 4,
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

    #[allow(dead_code)]
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
    Text = 9,
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

        if string_data_type.contains("char") {
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
            ColumnType::Text => 9,
        }
    }
}

#[derive(Debug)]
pub enum RcdDbError {
    General(String),
    DbNotFound(String),
    TableNotFound(String),
    LogicalStoragePolicyNotSet(String),
}

impl Error for RcdDbError {}

impl fmt::Display for RcdDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RcdDbError")
    }
}

#[derive(Debug)]
pub enum RcdGenerateContractError {
    General(String),
    NotAllTablesSet(String),
}

impl Error for RcdGenerateContractError {}

impl fmt::Display for RcdGenerateContractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RcdGenerateContractError")
    }
}

/// Represents the type of backing database rcd is hosting
/// # Types
/// * 0 - Unknown
/// * 1 - Sqlite
/// * 2 - Mysql
/// * 3 - Postgres
/// * 4 - Sqlserver
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DatabaseType {
    Unknown = 0,
    Sqlite = 1,
    Mysql = 2,
    Postgres = 3,
    Sqlserver = 4,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl DatabaseType {
    pub fn from_i64(value: i64) -> DatabaseType {
        match value {
            0 => DatabaseType::Unknown,
            1 => DatabaseType::Sqlite,
            2 => DatabaseType::Mysql,
            3 => DatabaseType::Postgres,
            4 => DatabaseType::Sqlserver,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn from_u32(value: u32) -> DatabaseType {
        match value {
            0 => DatabaseType::Unknown,
            1 => DatabaseType::Sqlite,
            2 => DatabaseType::Mysql,
            3 => DatabaseType::Postgres,
            4 => DatabaseType::Sqlserver,
            _ => panic!("Unknown value: {}", value),
        }
    }

    #[allow(dead_code)]
    pub fn to_u32(db_type: DatabaseType) -> u32 {
        match db_type {
            DatabaseType::Unknown => 0,
            DatabaseType::Sqlite => 1,
            DatabaseType::Mysql => 2,
            DatabaseType::Postgres => 3,
            DatabaseType::Sqlserver => 4,
        }
    }
}
