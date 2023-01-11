use rcd_http_common::url::client::NEW_DATABASE;
use rcd_messages::client::{CreateUserDatabaseReply, CreateUserDatabaseRequest};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    request::{self, get_token, update_token_login_status, set_status, clear_status},
};

#[function_component]
pub fn Create() -> Html {
    let ui_db_name = use_node_ref();
    let last_created_result = use_state_eq(move || String::from(""));

    let onclick = {
        let ui_db_name = ui_db_name.clone();
        let last_created_result = last_created_result.clone();

        Callback::from(move |_| {
            let last_created_result = last_created_result.clone();
            let db_name = ui_db_name.cast::<HtmlInputElement>().unwrap().value();

            let token = get_token();

            let request = CreateUserDatabaseRequest {
                authentication: Some(token.auth()),
                database_name: db_name.clone(),
            };

            let json_request = serde_json::to_string(&request).unwrap();
            let url = format!("{}{}", token.addr, NEW_DATABASE);

            let cb = {
                let last_created_result = last_created_result.clone();
                Callback::from(move |response: Result<AttrValue, String>| {
                    if response.is_ok() {
                        let response = response.unwrap();
                        log_to_console(response.to_string());
                        clear_status();

                        let reply: CreateUserDatabaseReply =
                            serde_json::from_str(&response.to_string()).unwrap();

                        let is_authenticated = reply
                            .authentication_result
                            .as_ref()
                            .unwrap()
                            .is_authenticated;
                        update_token_login_status(is_authenticated);

                        if is_authenticated {
                            last_created_result.set(reply.is_created.to_string());
                        }
                    } else {
                        set_status(response.err().unwrap());
                    }
                })
            };

            request::post(url, json_request, cb);
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Create New Database"} </h1>
                    <label for="db_name">{ "Database Name (include suffix '.db')" }</label>
                    <input type="text" class="input"  id="db_name" placeholder="Enter Database Name" ref={&ui_db_name} />
                    <button type="button" class="button is-primary" id="submit" value="Create" {onclick}>
                                <span class="mdi mdi-database-plus">{" Create"}</span>
                    </button>
                    <p>
                    <label for="last_result">{ "Last Create Database Result:" }</label>
                    { (*last_created_result).clone() }
                    </p>
                </div>
            </div>
        </div>
    }
}
