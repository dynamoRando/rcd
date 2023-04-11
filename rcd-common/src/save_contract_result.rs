use rcd_enum::contract_status::ContractStatus;
use rcdproto::rcdp::Participant;

#[derive(Debug, Clone)]
pub struct RcdSaveContractResult {
    pub is_successful: bool,
    pub contract_status: ContractStatus,
    pub participant_information: Option<Participant>,
}
