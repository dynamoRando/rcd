use rcd_messages::client::DatabaseSchema;
use yew::NodeRef;

pub struct RcdConnectionUi {
    pub username: NodeRef,
    pub password: NodeRef,
    pub ip: NodeRef,
    pub port: NodeRef,
    pub http_port: NodeRef,
    pub databases: NodeRef,
}

impl RcdConnectionUi {
    pub fn new() -> RcdConnectionUi {
        return RcdConnectionUi {
            username: NodeRef::default(),
            password: NodeRef::default(),
            ip: NodeRef::default(),
            port: NodeRef::default(),
            http_port: NodeRef::default(),
            databases: NodeRef::default(),
        };
    }
}

pub struct RcdConnectionData {
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: u32,
    pub databases: Vec<DatabaseSchema>,
    pub active: RcdConnectionDataActive,
}

impl RcdConnectionData {
    pub fn new() -> RcdConnectionData {
        return RcdConnectionData {
            username: "".to_string(),
            password: "".to_string(),
            ip: "".to_string(),
            port: 0,
            databases: Vec::new(),
            active: RcdConnectionDataActive::new(),
        };
    }
}

pub struct RcdConnectionDataActive {
    pub database: String,
    pub table: String,
    pub url: String,
    pub authentication_json: String,
}

impl RcdConnectionDataActive {
    pub fn new() -> RcdConnectionDataActive {
        return RcdConnectionDataActive {
            database: "".to_string(),
            table: "".to_string(),
            url: "".to_string(),
            authentication_json: "".to_string(),
        };
    }
}

pub struct RcdConnection {
    pub ui: RcdConnectionUi,
    pub data: RcdConnectionData,
}

impl RcdConnection {
    pub fn new() -> RcdConnection {
        return RcdConnection {
            ui: RcdConnectionUi::new(),
            data: RcdConnectionData::new(),
        };
    }
}
