use std::{error::Error, fmt};
use substring::Substring;

/// Represents the kinds of databases in rcd
/// # Kinds
/// - 0 - Unknown
/// - 1 - Rcd database itself
/// - 2 - Host database
/// - 3 - Partial database
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    pub fn to_u32(db: RcdDatabaseType) -> u32 {
        match db {
            RcdDatabaseType::Unknown => 0,
            RcdDatabaseType::Rcd => 1,
            RcdDatabaseType::Host => 2,
            RcdDatabaseType::Partial => 3,
        }
    }
}

/// Specifies the UpdateStatus in a UpdateDataResult message
/// in rcdp.proto
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdateStatusForPartialData {
    Unknown = 0,
    SucessOverwriteOrLog = 1,
    Pending = 2,
    Ignored = 3,
}

impl UpdateStatusForPartialData {
    pub fn from_u32(value: u32) -> UpdateStatusForPartialData {
        match value {
            0 => UpdateStatusForPartialData::Unknown,
            1 => UpdateStatusForPartialData::SucessOverwriteOrLog,
            2 => UpdateStatusForPartialData::Pending,
            3 => UpdateStatusForPartialData::Ignored,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(db: UpdateStatusForPartialData) -> u32 {
        match db {
            UpdateStatusForPartialData::Unknown => 0,
            UpdateStatusForPartialData::SucessOverwriteOrLog => 1,
            UpdateStatusForPartialData::Pending => 2,
            UpdateStatusForPartialData::Ignored => 3,
        }
    }
}

/// From the perspective of a participant: if we execute an `DELETE` statement
/// against our partial database, we can define how we want to notify the database host:
/// 1. Send Notification - send a note to the host of deleted row id
/// 2. Do Nothing - the host and the participant may potentially be out of sync
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeletesToHostBehavior {
    Unknown = 0,
    SendNotification = 1,
    DoNothing = 2,
}

impl DeletesToHostBehavior {
    pub fn from_u32(value: u32) -> DeletesToHostBehavior {
        match value {
            0 => DeletesToHostBehavior::Unknown,
            1 => DeletesToHostBehavior::SendNotification,
            2 => DeletesToHostBehavior::DoNothing,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(behavior: DeletesToHostBehavior) -> u32 {
        match behavior {
            DeletesToHostBehavior::Unknown => 0,
            DeletesToHostBehavior::SendNotification => 1,
            DeletesToHostBehavior::DoNothing => 2,
        }
    }
}

/// From the perspective of a participant: if we execute an `UPDATE` statement
/// against our partial database, we can define how we want to notify the database host:
/// 1. Send Data Hash Change - send a note to the host of the changed data hash, if applicable
/// 2. Do Nothing - the host and the participant may potentially have out of sync data hashes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdatesToHostBehavior {
    Unknown = 0,
    SendDataHashChange = 1,
    DoNothing = 2,
}

impl UpdatesToHostBehavior {
    pub fn from_u32(value: u32) -> UpdatesToHostBehavior {
        match value {
            0 => UpdatesToHostBehavior::Unknown,
            1 => UpdatesToHostBehavior::SendDataHashChange,
            2 => UpdatesToHostBehavior::DoNothing,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(behavior: UpdatesToHostBehavior) -> u32 {
        match behavior {
            UpdatesToHostBehavior::Unknown => 0,
            UpdatesToHostBehavior::SendDataHashChange => 1,
            UpdatesToHostBehavior::DoNothing => 2,
        }
    }
}

/// From the perspective of a participant: if we get an `UPDATE` statement from a database host
/// we can define how we want to respond:
/// 1. Allow Overwrite - will execute the `UPDATE` statement
/// 2. Queue For Review  - will add a "Pending" flag on the row
/// 3. Overwrite With Log - will copy the row to _HISTORY table and then overwrite
/// 4. Ignore - will not update the row but respond to the host with FALSE on the success reply
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdatesFromHostBehavior {
    Unknown = 0,
    AllowOverwrite = 1,
    QueueForReview = 2,
    OverwriteWithLog = 3,
    Ignore = 4,
    QueueForReviewAndLog = 5,
}

impl UpdatesFromHostBehavior {
    pub fn from_u32(value: u32) -> UpdatesFromHostBehavior {
        match value {
            0 => UpdatesFromHostBehavior::Unknown,
            1 => UpdatesFromHostBehavior::AllowOverwrite,
            2 => UpdatesFromHostBehavior::QueueForReview,
            3 => UpdatesFromHostBehavior::OverwriteWithLog,
            4 => UpdatesFromHostBehavior::Ignore,
            5 => UpdatesFromHostBehavior::QueueForReviewAndLog,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(behavior: UpdatesFromHostBehavior) -> u32 {
        match behavior {
            UpdatesFromHostBehavior::Unknown => 0,
            UpdatesFromHostBehavior::AllowOverwrite => 1,
            UpdatesFromHostBehavior::QueueForReview => 2,
            UpdatesFromHostBehavior::OverwriteWithLog => 3,
            UpdatesFromHostBehavior::Ignore => 4,
            UpdatesFromHostBehavior::QueueForReviewAndLog => 5,
        }
    }
}

/// From the perspective of a participant: if we get an `DELETE` statement from a database host
/// we can define how we want to respond:
/// 1. Allow Removal - will execute the `DELETE` statement
/// 2. Queue For Review  - will add a "Pending" flag on the row
/// 3. Delete With Log - will copy the row to _HISTORY table and then delete
/// 4. Ignore - will not delete the row but respond to the host with FALSE on the success reply
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeletesFromHostBehavior {
    Unknown = 0,
    AllowRemoval = 1,
    QueueForReview = 2,
    DeleteWithLog = 3,
    Ignore = 4,
    QueueForReviewAndLog = 5,
}

impl DeletesFromHostBehavior {
    pub fn from_u32(value: u32) -> DeletesFromHostBehavior {
        match value {
            0 => DeletesFromHostBehavior::Unknown,
            1 => DeletesFromHostBehavior::AllowRemoval,
            2 => DeletesFromHostBehavior::QueueForReview,
            3 => DeletesFromHostBehavior::DeleteWithLog,
            4 => DeletesFromHostBehavior::Ignore,
            5 => DeletesFromHostBehavior::QueueForReviewAndLog,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u32(behavior: DeletesFromHostBehavior) -> u32 {
        match behavior {
            DeletesFromHostBehavior::Unknown => 0,
            DeletesFromHostBehavior::AllowRemoval => 1,
            DeletesFromHostBehavior::QueueForReview => 2,
            DeletesFromHostBehavior::DeleteWithLog => 3,
            DeletesFromHostBehavior::Ignore => 4,
            DeletesFromHostBehavior::QueueForReviewAndLog => 5,
        }
    }
}

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

    pub fn to_u32(dml_type: HostStatus) -> u32 {
        match dml_type {
            HostStatus::Unknown => 0,
            HostStatus::Allow => 1,
            HostStatus::Deny => 2,
        }
    }
}

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
    pub fn data_type_as_string_sqlite(self: &Self) -> String {
        match self {
            ColumnType::Unknown => panic!(),
            ColumnType::Int => String::from("INT"),
            ColumnType::Bit => String::from("TINYINT"),
            ColumnType::Char => String::from("CHAR"),
            ColumnType::DateTime => String::from("DATETIME"),
            ColumnType::Decimal => String::from("DECIMAL"),
            ColumnType::Varchar => String::from("VARCHAR"),
            ColumnType::Binary => String::from("BLOB"),
            ColumnType::Varbinary => String::from("BLOB"),
            ColumnType::Text => String::from("TEXT"),
        }
    }

    pub fn data_type_to_enum_u32(desc: String) -> u32 {
        let ct = ColumnType::try_parse(&desc).unwrap();
        return ColumnType::to_u32(ct);
    }

    pub fn data_type_len(desc: String) -> u32 {
        let idx_first_paren = desc.find("(");

        if idx_first_paren.is_none() {
            return 0;
        } else {
            let idx_first = idx_first_paren.unwrap();
            let idx_last = desc.find(")").unwrap();
            let str_length = desc.substring(idx_first, idx_last);
            let length: u32 = str_length.parse().unwrap();
            return length;
        }
    }

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
