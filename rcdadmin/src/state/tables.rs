use yew::NodeRef;

pub struct RcdTablesUi {
    pub current_policy: NodeRef,
    pub new_policy: NodeRef,
}

impl RcdTablesUi {
    pub fn new() -> RcdTablesUi {
        return RcdTablesUi {
            current_policy: NodeRef::default(),
            new_policy: NodeRef::default(),
        };
    }
}

pub struct RcdTablesData {
    pub active: RcdTablesDataActive,
}

impl RcdTablesData {
    pub fn new() -> RcdTablesData {
        return RcdTablesData {
            active: RcdTablesDataActive::new(),
        };
    }
}

pub struct RcdTablesDataActive {
    pub database_name: String,
    pub table_name: String,
    pub policy_value: u32,
    pub policy_name: String,
}

impl RcdTablesDataActive {
    pub fn new() -> RcdTablesDataActive {
        return RcdTablesDataActive {
            database_name: "".to_string(),
            table_name: "".to_string(),
            policy_value: 0,
            policy_name: "".to_string(),
        };
    }
}

pub struct RcdTables {
    pub ui: RcdTablesUi,
    pub data: RcdTablesData,
}

impl RcdTables {
    pub fn new() -> RcdTables {
        return RcdTables {
            ui: RcdTablesUi::new(),
            data: RcdTablesData::new(),
        };
    }
}
