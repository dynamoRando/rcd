use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use rcd_http_common::url::client::CHANGE_UPDATES_FROM_HOST_BEHAVIOR;
use rcd_messages::client::{
    ChangeUpdatesFromHostBehaviorRequest, ChangesUpdatesFromHostBehaviorReply,
};
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_node_ref, AttrValue, Callback, Html, Properties, UseStateHandle,
};

use crate::{
    log::log_to_console,
    request::{self, clear_status, get_token, set_status, update_token_login_status},
};

#[derive(Properties, PartialEq)]
pub struct ChangeBehaviorProps {
    pub active_database: UseStateHandle<String>,
    pub active_table: UseStateHandle<String>,
}

#[function_component]
pub fn ChangeBehavior(
    ChangeBehaviorProps {
        active_database,
        active_table,
    }: &ChangeBehaviorProps,
) -> Html {
    let ui_behavior = use_node_ref();
    let database = active_database.clone();
    let table = active_table.clone();

    let onclick = {
        let ui_behavior = ui_behavior.clone();
        let database = database;
        let table = table;
        Callback::from(move |_| {
            let behavior = ui_behavior.cast::<HtmlInputElement>().unwrap().value();
            let database = database.clone();
            let table = table.clone();

            let behavior_value =
                UpdatesFromHostBehavior::from_u32(behavior.parse::<u32>().unwrap());
            let behavior_value = UpdatesFromHostBehavior::to_u32(behavior_value);

            let token = get_token();
            let url = format!("{}{}", token.addr, CHANGE_UPDATES_FROM_HOST_BEHAVIOR);
            let request = ChangeUpdatesFromHostBehaviorRequest {
                authentication: Some(token.auth()),
                database_name: (*database).clone(),
                table_name: (*table).clone(),
                behavior: behavior_value,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                match response {
                    Ok(response) => {
                        let database = database.clone();
                        let table = table.clone();
                        clear_status();

                        log_to_console(response.to_string());

                        let reply: ChangesUpdatesFromHostBehaviorReply =
                            serde_json::from_str(&response).unwrap();
                        let is_authenticated = reply
                            .authentication_result
                            .as_ref()
                            .unwrap()
                            .is_authenticated;
                        update_token_login_status(is_authenticated);

                        if is_authenticated {
                            if reply.is_successful {
                                let behavior = UpdatesFromHostBehavior::from_u32(behavior_value);
                                let behavior = behavior.as_string();

                                let message = format!(
                                    "{}{}{}{}{}{}",
                                    "Behavior Updated For: ",
                                    (*database),
                                    " table: ",
                                    (*table),
                                    " behavior to: ",
                                    behavior
                                );
                                set_status(message);
                            } else {
                                let behavior = UpdatesFromHostBehavior::from_u32(behavior_value);
                                let behavior = behavior.as_string();

                                let message = format!(
                                    "{}{}{}{}{}{}",
                                    "Behavior Updated FAILED For: ",
                                    (*database),
                                    " table: ",
                                    (*table),
                                    " behavior to: ",
                                    behavior
                                );
                                set_status(message);
                            }
                        }
                    }
                    Err(error_message) => {
                        set_status(error_message);
                    }
                };
            });

            request::post(url, body, cb);
        })
    };

    html!(
        <div>
            <p><h1 class="subtitle">{"Change Behavior"}</h1></p>
                <div class ="select is-multiple">
                <select name="set_updates_from_host_behavior" id="set_updates_from_host_behavior" ref={ui_behavior} >
                    <option value="0">{"SELECT BEHAVIOR"}</option>
                    <option value="1">{"AllowOverwrite"}</option>
                    <option value="2">{"QueueForReview"}</option>
                    <option value="3">{"OverwriteWithLog"}</option>
                    <option value="4">{"Ignore"}</option>
                    <option value="5">{"QueueForReviewAndLog"}</option>
                </select>
                </div>
                <button
                    class="button"
                    type="button"
                    id="update_behavior"
                    value="Update Behavior"
                    onclick={onclick}>
                        <span class="mdi mdi-eject-circle">{" Update Behavior"}</span>
                </button>
        </div>
    )
}
