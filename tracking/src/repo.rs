use chrono::NaiveDateTime;
use rcd_messages::{
    client::{ExecuteReadReply, ExecuteReadRequest},
    proxy::request_type::RequestType,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{AttrValue, Callback};

use crate::{
    event::SharkEvent,
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
    pub fn get_events() -> Result<Vec<SharkEvent>, String> {
        
        spawn_local(async move {
            let mut proxy = Proxy::get_from_session_storage();
            proxy.login().await;
        });
        
        let token = Proxy::get_rcd_token_from_session_storage();
        let auth = token.auth();

        let request = ExecuteReadRequest {
            authentication: Some(auth),
            database_name: DB_NAME.to_string(),
            sql_statement: SQL_GET_ASSOCIATED_EVENTS.to_string(),
            database_type: 1,
        };

        let read_request_json = serde_json::to_string(&request).unwrap();

        let callback_associated_events =
            Callback::from(move |response: Result<AttrValue, String>| {
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
                                let mut event_id: u32;
                                let event_type: u32;
                                let event_date: NaiveDateTime;
                                let notes: String;

                                for value in &row.values {
                                    if let Some(column) = &value.column {
                                        if column.column_name == "event_id" {
                                            let result_event_id = value.string_value.parse::<u32>();
                                            if let Ok(eid) = result_event_id {
                                                event_id = eid;
                                            } else {
                                                event_id = 0;
                                            }
                                        }
                                    }
                                }
                            }

                            todo!()
                        }
                    } else {
                        log_to_console("warning: we are not logged in to rcd");
                    }
                } else {
                    log_to_console("warning: we are not logged in to proxy");
                }
            });

        request::post(
            RequestType::ReadAtHost,
            &read_request_json,
            callback_associated_events,
        );

        todo!()
    }
}
