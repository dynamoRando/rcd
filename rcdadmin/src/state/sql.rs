use yew::NodeRef;

pub struct RcdSqlUi {
    pub execute_sql: NodeRef,
    pub sql_result: NodeRef,
    pub db_name: NodeRef,
}

impl RcdSqlUi {
    pub fn new() -> RcdSqlUi {
        return RcdSqlUi {
            execute_sql: NodeRef::default(),
            sql_result: NodeRef::default(),
            db_name: NodeRef::default(),
        };
    }
}

pub struct RcdSqlData {
    pub active: RcdSqlDataActive,
}

impl RcdSqlData {
    pub fn new() -> RcdSqlData {
        return RcdSqlData {
            active: RcdSqlDataActive::new(),
        };
    }
}

pub struct RcdSqlDataActive {
    pub db_name: String,
}

impl RcdSqlDataActive {
    pub fn new() -> RcdSqlDataActive {
        return RcdSqlDataActive {
            db_name: "".to_string(),
        };
    }
}

pub struct RcdSqlResultUi {
    pub text: NodeRef,
}

impl RcdSqlResultUi {
    pub fn new() -> RcdSqlResultUi {
        return RcdSqlResultUi {
            text: NodeRef::default(),
        };
    }
}

pub struct RcdSqlResultData {
    pub text: String,
}

impl RcdSqlResultData {
    pub fn new() -> RcdSqlResultData {
        return RcdSqlResultData {
            text: "".to_string(),
        };
    }
}

pub struct RcdSqlResult {
    pub ui: RcdSqlResultUi,
    pub data: RcdSqlResultData,
}

impl RcdSqlResult {
    pub fn new() -> RcdSqlResult {
        return RcdSqlResult {
            ui: RcdSqlResultUi::new(),
            data: RcdSqlResultData::new(),
        };
    }
}

pub struct RcdSql {
    pub ui: RcdSqlUi,
    pub data: RcdSqlData,
    pub result: RcdSqlResult,
}

impl RcdSql {
    pub fn new() -> RcdSql {
        return RcdSql {
            ui: RcdSqlUi::new(),
            data: RcdSqlData::new(),
            result: RcdSqlResult::new(),
        };
    }
}
