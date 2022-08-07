use crate::{database_participant::DatabaseParticipant, host_info::HostInfo, database_contract::DatabaseContract, cdata::GetRowFromPartialDatabaseResult};

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn send_participant_contract(participant: DatabaseParticipant, host_info: HostInfo, contract: DatabaseContract) -> bool {
    unimplemented!();
}


#[allow(dead_code, unused_assignments, unused_variables)]
pub fn get_row_from_participant(participant: DatabaseParticipant, host_info: HostInfo, db_name: &str, table_name: &str) -> GetRowFromPartialDatabaseResult {
    unimplemented!();
}
