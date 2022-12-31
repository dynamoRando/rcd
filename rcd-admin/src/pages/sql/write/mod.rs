use rcd_messages::client::{
    ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteWriteReply,
    ExecuteWriteRequest,
};
use yew::{AttrValue, Callback, UseStateHandle};

use crate::{
    log::log_to_console,
    request::{self, get_token},
};

pub fn write(db_name: String, text: String, state: UseStateHandle<Option<String>>, endpoint: &str) {
    let token = get_token();
    let auth = token.auth();

    let request = ExecuteWriteRequest {
        authentication: Some(auth),
        database_name: db_name,
        sql_statement: text,
        database_type: 1,
        where_clause: "".to_string(),
    };

    let write_request_json = serde_json::to_string(&request).unwrap();
    let url = format!("{}{}", token.addr, endpoint);

    let callback = Callback::from(move |response: AttrValue| {
        let response = response.to_string();
        log_to_console(response.clone());

        let write_reply: ExecuteWriteReply = serde_json::from_str(&response.to_string()).unwrap();

        if write_reply.authentication_result.unwrap().is_authenticated {
            let mut result_message = String::new();

            result_message = result_message
                + &format!(
                    "Is result successful: {}",
                    write_reply.is_successful.to_string()
                );

            result_message = result_message + &"\n";
            result_message = result_message
                + &format!(
                    "Total rows affected: {}",
                    write_reply.total_rows_affected.to_string()
                );
            result_message = result_message + &"\n";
            result_message =
                result_message + &format!("Error Message: {}", write_reply.error_message.clone());

            let sql_table_text = result_message.clone();

            state.set(Some(sql_table_text));
        }
    });

    request::get_data(url, write_request_json, callback);
}

pub fn cooperative_write(
    db_name: String,
    text: String,
    participant_alias: String,
    state: UseStateHandle<Option<String>>,
    endpoint: &str,
) {
    let token = get_token();
    let auth = token.auth();

    let request = ExecuteCooperativeWriteRequest {
        authentication: Some(auth),
        database_name: db_name.clone(),
        sql_statement: text,
        database_type: 1,
        where_clause: "".to_string(),
        alias: participant_alias,
        participant_id: "".to_string(),
    };

    let write_request_json = serde_json::to_string(&request).unwrap();
    let url = format!("{}{}", token.addr, endpoint);

    let callback = Callback::from(move |response: AttrValue| {
        let response = response.to_string();
        log_to_console(response.clone());

        let write_reply: ExecuteCooperativeWriteReply =
            serde_json::from_str(&response.to_string()).unwrap();

        if write_reply.authentication_result.unwrap().is_authenticated {
            let mut result_message = String::new();

            result_message = result_message
                + &format!(
                    "Is result successful: {}",
                    write_reply.is_successful.to_string()
                );

            result_message = result_message + &"\n";
            result_message = result_message
                + &format!(
                    "Total rows affected: {}",
                    write_reply.total_rows_affected.to_string()
                );
            result_message = result_message + &"\n";

            let sql_table_text = result_message.clone();

            state.set(Some(sql_table_text));
        }
    });

    request::get_data(url, write_request_json, callback);
}
