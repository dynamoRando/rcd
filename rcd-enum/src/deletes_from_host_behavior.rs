

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
