use rcd_messages::{
    client::{ExecuteReadReply, ExecuteReadRequest},
    formatter,
};
use yew::{AttrValue, Callback, UseStateHandle};

use crate::{
    log::log_to_console,
    request::{self, clear_status, get_token, set_status, update_token_login_status},
};

pub fn read(db_name: String, text: String, state: UseStateHandle<Option<String>>, endpoint: &str) {
    let token = get_token();
    let auth = token.auth();

    let request = ExecuteReadRequest {
        authentication: Some(auth),
        database_name: db_name,
        sql_statement: text,
        database_type: 1,
    };

    let read_request_json = serde_json::to_string(&request).unwrap();
    let url = format!("{}{}", token.addr, endpoint);

    let callback = Callback::from(move |response: Result<AttrValue, String>| {
        if let Ok(ref x) = response {
            log_to_console(x.to_string());
            clear_status();

            let read_reply: ExecuteReadReply = serde_json::from_str(x).unwrap();

            let is_authenticated = read_reply
                .authentication_result
                .as_ref()
                .unwrap()
                .is_authenticated;
            update_token_login_status(is_authenticated);

            if is_authenticated {
                let result = read_reply.results.first().unwrap();
                if !result.is_error {
                    let rows = result.clone().rows;
                    let sql_table_text = formatter::rows_to_string_markdown_table(&rows);

                    state.set(Some(sql_table_text));
                } else {
                    let mut message = String::new();
                    message += "ERROR: ";
                    message += &result.execution_error_message.clone();

                    state.set(Some(message));
                }
            }
        } else {
            set_status(response.err().unwrap());
        }
    });

    request::post(url, read_request_json, callback);
}
