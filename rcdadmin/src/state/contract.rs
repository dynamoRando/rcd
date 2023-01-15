use yew::NodeRef;

pub struct RcdContractGenerate {
    pub ui: RcdContractGenerateUi,
    pub data: RcdContractGenerateData,
    pub result: RcdContractGenerateResult,
}

impl RcdContractGenerate {
    pub fn new() -> RcdContractGenerate {
        return RcdContractGenerate {
            ui: RcdContractGenerateUi::new(),
            data: RcdContractGenerateData::new(),
            result: RcdContractGenerateResult::new(),
        };
    }
}

pub struct RcdContractGenerateResult {
    pub ui: RcdContractGenerateResultUi,
    pub data: RcdContractGenerateResultData,
}

impl RcdContractGenerateResult {
    pub fn new() -> RcdContractGenerateResult {
        return RcdContractGenerateResult {
            ui: RcdContractGenerateResultUi::new(),
            data: RcdContractGenerateResultData::new(),
        };
    }
}

pub struct RcdContractGenerateResultUi {}

impl RcdContractGenerateResultUi {
    pub fn new() -> RcdContractGenerateResultUi {
        return RcdContractGenerateResultUi {};
    }
}

pub struct RcdContractGenerateResultData {
    pub is_successful: bool,
}

impl RcdContractGenerateResultData {
    pub fn new() -> RcdContractGenerateResultData {
        return RcdContractGenerateResultData {
            is_successful: false,
        };
    }
}

pub struct RcdContractGenerateUi {
    pub host_name: NodeRef,
    pub description: NodeRef,
}

impl RcdContractGenerateUi {
    pub fn new() -> RcdContractGenerateUi {
        return RcdContractGenerateUi {
            host_name: NodeRef::default(),
            description: NodeRef::default(),
        };
    }
}

pub struct RcdContractGenerateData {
    pub delete_behavior: u32,
}

impl RcdContractGenerateData {
    pub fn new() -> RcdContractGenerateData {
        return RcdContractGenerateData { delete_behavior: 0 };
    }
}

pub struct RcdContractDataActive {
    pub markdown: String,
}

impl RcdContractDataActive {
    pub fn new() -> RcdContractDataActive {
        return RcdContractDataActive {
            markdown: "".to_string(),
        };
    }
}

pub struct RcdContractPendingData {
    pub markdown: String,
}

impl RcdContractPendingData {
    pub fn new() -> RcdContractPendingData {
        return RcdContractPendingData {
            markdown: "".to_string(),
        };
    }
}

pub struct RcdContractPendingUi {
    pub details: NodeRef,
}

impl RcdContractPendingUi {
    pub fn new() -> RcdContractPendingUi {
        return RcdContractPendingUi {
            details: NodeRef::default(),
        };
    }
}

pub struct RcdContractPending {
    pub data: RcdContractPendingData,
    pub ui: RcdContractPendingUi,
}

impl RcdContractPending {
    pub fn new() -> RcdContractPending {
        return RcdContractPending {
            data: RcdContractPendingData::new(),
            ui: RcdContractPendingUi::new(),
        };
    }
}

pub struct RcdContractData {
    pub active: RcdContractDataActive,
}

impl RcdContractData {
    pub fn new() -> RcdContractData {
        return RcdContractData {
            active: RcdContractDataActive::new(),
        };
    }
}

pub struct RcdContractUi {
    pub details: NodeRef,
}

impl RcdContractUi {
    pub fn new() -> RcdContractUi {
        return RcdContractUi {
            details: NodeRef::default(),
        };
    }
}

pub struct RcdContract {
    pub ui: RcdContractUi,
    pub data: RcdContractData,
    pub generate: RcdContractGenerate,
    pub pending: RcdContractPending,
}

impl RcdContract {
    pub fn new() -> RcdContract {
        return RcdContract {
            ui: RcdContractUi::new(),
            data: RcdContractData::new(),
            generate: RcdContractGenerate::new(),
            pending: RcdContractPending::new(),
        };
    }
}
