use rcd_messages::client::DatabaseSchema;

pub struct RcdDatabases {
    pub data: RcdDatabasesData,
}

impl RcdDatabases {
    pub fn new() -> RcdDatabases {
        return RcdDatabases {
            data: RcdDatabasesData::new(),
        };
    }
}

pub struct RcdDatabasesData {
    pub databases: Vec<DatabaseSchema>,
    pub active: RcdDatabasesDataActive,
}

impl RcdDatabasesData {
    pub fn new() -> RcdDatabasesData {
        return RcdDatabasesData {
            databases: Vec::new(),
            active: RcdDatabasesDataActive::new(),
        };
    }
}

pub struct RcdDatabasesDataActive {
    pub database_name: String,
}

impl RcdDatabasesDataActive {
    pub fn new() -> RcdDatabasesDataActive {
        return RcdDatabasesDataActive {
            database_name: "".to_string(),
        };
    }
}
