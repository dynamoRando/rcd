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

    pub fn to_string(value: UpdatesToHostBehavior) -> String {
        match value {
            UpdatesToHostBehavior::Unknown => "Unknown".to_string(),
            UpdatesToHostBehavior::SendDataHashChange => "SendDataHashChange".to_string(),
            UpdatesToHostBehavior::DoNothing => "DoNothing".to_string(),
        }
    }

    pub fn from_str(value: &str) -> UpdatesToHostBehavior {
        match value {
            "Unknown" => UpdatesToHostBehavior::Unknown,
            "SendDataHashChange" => UpdatesToHostBehavior::SendDataHashChange,
            "DoNothing" => UpdatesToHostBehavior::DoNothing,
            _ => UpdatesToHostBehavior::Unknown,
        }
    }

    pub fn as_string(self) -> String {
        return UpdatesToHostBehavior::to_string(self);
    }
}
