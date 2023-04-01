use rcd_messages::{
    client::{ExecuteReadReply, ExecuteReadRequest},
    proxy::request_type::RequestType,
};
use serde::{Deserialize, Serialize};
use yew::{AttrValue, Callback};

use crate::{
    event::Event,
    logging::log_to_console,
    settings::{request, Proxy, DB_NAME},
};

pub const SQL_GET_EVENTS: &str = "
SELECT 
    id, 
    event_date, 
    notes 
FROM 
    event
;";

pub const SQL_GET_ASSOCIATED_EVENTS: &str = "
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
pub struct Repo {}

impl Repo {
    pub fn get_events() -> Result<Vec<Event>, String> {
        let token = Proxy::get_token_from_session_storage();
        let auth = token.auth();

        let request = ExecuteReadRequest {
            authentication: Some(auth),
            database_name: DB_NAME.to_string(),
            sql_statement: SQL_GET_ASSOCIATED_EVENTS.to_string(),
            database_type: 1,
        };

        let read_request_json = serde_json::to_string(&request).unwrap();

        let callback_associated_events = Callback::from(move |response: Result<AttrValue, String>| {
            if let Ok(ref x) = response {
                log_to_console(x);

                let read_reply: ExecuteReadReply = serde_json::from_str(x).unwrap();

                let is_authenticated = read_reply
                    .authentication_result
                    .as_ref()
                    .unwrap()
                    .is_authenticated;

                if is_authenticated {
                    let result = read_reply.results.first().unwrap();
                    if !result.is_error {
                        let rows = result.clone().rows;

                        for row in &rows {
                            
                        }

                        todo!()
                    }
                }
            } else {
                log_to_console("warning: we are not logged in");
            }
        });

        request::post(RequestType::ReadAtHost, &read_request_json, callback_associated_events);

        todo!()
    }
}
