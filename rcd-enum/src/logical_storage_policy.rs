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
