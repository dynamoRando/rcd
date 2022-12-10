pub mod client {
    pub const READ_SQL_AT_HOST: &str = "/client/sql/host/read/";
    pub const SEND_CONTRACT_TO_PARTICIPANT: &str = "/client/databases/participant/send-contract/";
    pub const READ_SQL_AT_PARTICIPANT: &str = "/client/sql/participant/read/";
    pub const WRITE_SQL_AT_HOST: &str = "/client/sql/host/write/";
    pub const WRITE_SQL_AT_PARTICIPANT: &str = "/client/sql/participant/write/";
    pub const GENERATE_CONTRACT: &str = "/client/databases/contract/generate/";
    pub const ADD_PARTICIPANT: &str = "/client/databases/participant/add/";
    pub const GET_PARTICIPANTS: &str = "/client/databases/participant/get/";
    pub const GET_ACTIVE_CONTRACT: &str = "/client/databases/contract/get/";
    pub const IS_ONLINE: &str = "/client/version";
    pub const NEW_DATABASE: &str = "/client/databases/new/";
    pub const ENABLE_COOPERATIVE_FEATURES: &str = "/client/databases/enable-cooperative-features";
    pub const SET_POLICY: &str = "/client/databases/table/policy/set/";
    pub const GENERATE_HOST_INFO: &str = "/client/host/generate";
    pub const VIEW_PENDING_CONTRACTS: &str = "/client/contract/review";
    pub const ACCEPT_PENDING_CONTRACT: &str = "/client/contract/accept/";
}

pub mod data {
    pub const SAVE_CONTRACT: &str = "/data/contract/save/";
    pub const PARTICIPANT_ACCEPTS_CONTRACT: &str = "/data/contract/accepted-by-participant";
}
