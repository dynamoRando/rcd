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
            _ => panic!("Unknown value: {value}"),
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

    pub fn to_string(value: UpdatesFromHostBehavior) -> String {
        match value {
            UpdatesFromHostBehavior::Unknown => "Unknown".to_string(),
            UpdatesFromHostBehavior::AllowOverwrite => "AllowOverwrite".to_string(),
            UpdatesFromHostBehavior::QueueForReview => "QueueForReview".to_string(),
            UpdatesFromHostBehavior::OverwriteWithLog => "OverwriteWithLog".to_string(),
            UpdatesFromHostBehavior::Ignore => "Ignore".to_string(),
            UpdatesFromHostBehavior::QueueForReviewAndLog => "QueueForReviewAndLog".to_string(),
        }
    }

    pub fn from_str(value: &str) -> UpdatesFromHostBehavior {
        match value {
            "Unknown" => UpdatesFromHostBehavior::Unknown,
            "AllowOverwrite" => UpdatesFromHostBehavior::AllowOverwrite,
            "QueueForReview" => UpdatesFromHostBehavior::QueueForReview,
            "OverwriteWithLog" => UpdatesFromHostBehavior::OverwriteWithLog,
            "QueueForReviewAndLog" => UpdatesFromHostBehavior::QueueForReviewAndLog,
            _ => UpdatesFromHostBehavior::Unknown,
        }
    }

    pub fn as_string(self) -> String {
        UpdatesFromHostBehavior::to_string(self)
    }
}
