use crate::rcd_enum::ContractStatus;
use guid_create::GUID;

/*
"CREATE TABLE IF NOT EXISTS COOP_PARTICIPANT
    (
        INTERNAL_PARTICIPANT_ID CHAR(36) NOT NULL,
        ALIAS VARCHAR(50) NOT NULL,
        IP4ADDRESS VARCHAR(25),
        IP6ADDRESS VARCHAR(25),
        PORT INT,
        CONTRACT_STATUS INT,
        ACCEPTED_CONTRACT_VERSION_ID CHAR(36),
        TOKEN BLOB NOT NULL,
        PARTICIPANT_ID CHAR(36)
    );",
*/

#[derive(Clone, Debug)]
pub struct CoopDatabaseParticipant {
    pub internal_id: GUID,
    pub alias: String,
    pub ip4addr: String,
    pub ip6addr: String,
    pub db_port: u32,
    pub contract_status: ContractStatus,
    pub accepted_contract_version: GUID,
    pub token: Vec<u8>,
    pub id: GUID,
    pub http_addr: String,
    pub http_port: u16
}

#[derive(Clone, Debug)]
pub struct CoopDatabaseParticipantData {
    pub participant: CoopDatabaseParticipant,
    pub db_name: String,
    pub table_name: String,
    pub row_data: Vec<(u32, Vec<u8>)>,
}
