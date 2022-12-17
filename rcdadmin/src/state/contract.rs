pub struct RcdContractGenerate {
    pub ui: RcdContractGenerateUi,
    pub data: RcdContractGenerateData,
}

impl RcdContractGenerate {
    pub fn new() -> RcdContractGenerate {
        return RcdContractGenerate { 
            ui: RcdContractGenerateUi::new(), 
            data: RcdContractGenerateData::new()
        }
    }
}

pub struct RcdContractGenerateUi {}

impl RcdContractGenerateUi {
    pub fn new() -> RcdContractGenerateUi {
        return RcdContractGenerateUi {};
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

pub struct RcdContractDataActive {}

impl RcdContractDataActive {
    pub fn new() -> RcdContractDataActive {
        return RcdContractDataActive {};
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

pub struct RcdContractUi {}

impl RcdContractUi {
    pub fn new() -> RcdContractUi {
        return RcdContractUi {};
    }
}

pub struct RcdContract {
    pub ui: RcdContractUi,
    pub data: RcdContractData,
    pub generate: RcdContractGenerate,
}

impl RcdContract {
    pub fn new() -> RcdContract {
        return RcdContract {
            ui: RcdContractUi::new(),
            data: RcdContractData::new(),
            generate: RcdContractGenerate::new(),
        };
    }
}
