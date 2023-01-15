use rcd_messages::client::ParticipantStatus;
use yew::NodeRef;

pub struct RcdParticipantDataActive {
    pub participants: Vec<ParticipantStatus>,
    pub alias: String,
}

impl RcdParticipantDataActive {
    pub fn new() -> RcdParticipantDataActive {
        return RcdParticipantDataActive {
            participants: Vec::new(),
            alias: "".to_string(),
        };
    }
}

pub struct RcdParticipantData {
    pub active: RcdParticipantDataActive,
    pub result: RcdParticipantDataResult,
}

impl RcdParticipantData {
    pub fn new() -> RcdParticipantData {
        return RcdParticipantData {
            active: RcdParticipantDataActive::new(),
            result: RcdParticipantDataResult::new(),
        };
    }
}

pub struct RcdParticipantDataResult {
    pub add_participant: bool,
    pub send_contract: bool,
}

impl RcdParticipantDataResult {
    pub fn new() -> RcdParticipantDataResult {
        return RcdParticipantDataResult {
            add_participant: false,
            send_contract: false,
        };
    }
}

pub struct RcdParticipantsUiAdd {
    pub alias: NodeRef,
    pub addr: NodeRef,
    pub port: NodeRef,
    pub http_addr: NodeRef,
    pub http_port: NodeRef,
}

impl RcdParticipantsUiAdd {
    pub fn new() -> RcdParticipantsUiAdd {
        return RcdParticipantsUiAdd {
            alias: NodeRef::default(),
            addr: NodeRef::default(),
            port: NodeRef::default(),
            http_addr: NodeRef::default(),
            http_port: NodeRef::default(),
        };
    }
}

pub struct RcdParticipantsUi {
    pub add: RcdParticipantsUiAdd,
}

impl RcdParticipantsUi {
    pub fn new() -> RcdParticipantsUi {
        return RcdParticipantsUi {
            add: RcdParticipantsUiAdd::new(),
        };
    }
}

pub struct RcdParticipantsSendContractResult {
    pub is_successful: bool,
}

impl RcdParticipantsSendContractResult {
    pub fn new() -> RcdParticipantsSendContractResult {
        return RcdParticipantsSendContractResult {
            is_successful: false,
        };
    }
}

pub struct RcdParticipantSendContractUi {
    pub alias: NodeRef,
}

impl RcdParticipantSendContractUi {
    pub fn new() -> RcdParticipantSendContractUi {
        return RcdParticipantSendContractUi {
            alias: NodeRef::default(),
        };
    }
}

pub struct RcdParticipantsSendContractData {
    pub alias: String,
}

impl RcdParticipantsSendContractData {
    pub fn new() -> RcdParticipantsSendContractData {
        return RcdParticipantsSendContractData {
            alias: String::from(""),
        };
    }
}

pub struct RcdParticipantsSendContract {
    pub ui: RcdParticipantSendContractUi,
    pub data: RcdParticipantsSendContractData,
    pub result: RcdParticipantsSendContractResult,
}

impl RcdParticipantsSendContract {
    pub fn new() -> RcdParticipantsSendContract {
        return RcdParticipantsSendContract {
            ui: RcdParticipantSendContractUi::new(),
            data: RcdParticipantsSendContractData::new(),
            result: RcdParticipantsSendContractResult::new(),
        };
    }
}

pub struct RcdParticipants {
    pub ui: RcdParticipantsUi,
    pub data: RcdParticipantData,
    pub send_contract: RcdParticipantsSendContract,
}

impl RcdParticipants {
    pub fn new() -> RcdParticipants {
        return RcdParticipants {
            ui: RcdParticipantsUi::new(),
            data: RcdParticipantData::new(),
            send_contract: RcdParticipantsSendContract::new(),
        };
    }
}
