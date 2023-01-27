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
            _ => panic!("Unknown value: {value}"),
        }
    }

    pub fn to_u32(behavior: DeletesToHostBehavior) -> u32 {
        match behavior {
            DeletesToHostBehavior::Unknown => 0,
            DeletesToHostBehavior::SendNotification => 1,
            DeletesToHostBehavior::DoNothing => 2,
        }
    }

    pub fn to_string(value: DeletesToHostBehavior) -> String {
        match value {
            DeletesToHostBehavior::Unknown => "Unknown".to_string(),
            DeletesToHostBehavior::SendNotification => "SendNotification".to_string(),
            DeletesToHostBehavior::DoNothing => "DoNothing".to_string(),
        }
    }

    pub fn from_str(value: &str) -> DeletesToHostBehavior {
        match value {
            "Unknown" => DeletesToHostBehavior::Unknown,
            "SendNotification" => DeletesToHostBehavior::SendNotification,
            "DoNothing" => DeletesToHostBehavior::DoNothing,
            _ => DeletesToHostBehavior::Unknown,
        }
    }

    pub fn as_string(self) -> String {
        DeletesToHostBehavior::to_string(self)
    }
}
