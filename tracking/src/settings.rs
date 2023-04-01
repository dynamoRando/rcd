use crate::{logging::log_to_console, SETTINGS_TOML, event::Event};
use config::Config;
use serde::{Deserialize, Serialize};

use gloo::{
    storage::{SessionStorage, Storage},
};

const SHARK_SETTINGS: &str = "sharksettings.key";

const SQL_GET_EVENTS: &str = "
SELECT 
    id, 
    event_date, 
    notes 
FROM 
    event
;";

const SQL_GET_ASSOCIATED_EVENTS: &str = "
SELECT 
    event_id,
    event_type,
    event_date,
    notes
FROM 
    associated_event
;
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharkSettings {
    address: String,
    account: String,
}

impl SharkSettings {
    pub fn new(address: &str, account: &str) -> Self {
        Self {
            address: address.to_string(),
            account: account.to_string(),
        }
    }

    pub fn read_from_config(path_to_file: &str) -> SharkSettings {
        let error_message = format!("{}{}", "Could not find ", SETTINGS_TOML);

        let settings = Config::builder()
            .add_source(config::File::with_name(path_to_file))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .expect(&error_message);

        let addr = settings.get_string(&String::from("address")).unwrap();
        let acc = settings.get_string(&String::from("account")).unwrap();

        SharkSettings {
            address: addr,
            account: acc,
        }
    }

    pub fn addr(&self) -> String {
        self.address.clone()
    }

    pub fn account(&self) -> String {
        self.account.clone()
    }

    pub fn get_events(&self) -> Vec<Event> {
        todo!()
    }

    pub fn save_to_session_storage(&self) {
        let json = serde_json::to_string(&self).unwrap();
        log_to_console(&json);
        SessionStorage::set(SHARK_SETTINGS, json).expect("failed to set");
    }

    pub fn get_from_session_storage() -> SharkSettings {
        let json = SessionStorage::get(SHARK_SETTINGS).unwrap_or_else(|_| String::from(""));

        if !json.is_empty() {
            let settings: SharkSettings = serde_json::from_str(&json).unwrap();
            return settings;
        };

        SharkSettings {
            address: "proxy.home:50040".to_string(),
            account: "shark".to_string(),
        }
    }
}
