use rcd_messages::client::{CreateUserDatabaseReply, CreateUserDatabaseRequest};
use rcd_messages::proxy::request_type::RequestType;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    request::{
        self,
        rcd::{clear_status, get_rcd_token, set_status, update_token_login_status},
    },
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

            let token = get_rcd_token();

            let request = CreateUserDatabaseRequest {
                authentication: Some(token.auth()),
                database_name: db_name,
            };

            let json_request = serde_json::to_string(&request).unwrap();

            let cb = {
                let last_created_result = last_created_result;
                Callback::from(move |response: Result<AttrValue, String>| {
                    if let Ok(ref x) = response {
                        log_to_console(&x);
                        clear_status();

                        let reply: CreateUserDatabaseReply = serde_json::from_str(x).unwrap();

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

            request::post(RequestType::CreateUserDatabase, &json_request, cb);
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
