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
