use rcd_enum::updates_to_host_behavior::UpdatesToHostBehavior;

use crate::request;
use crate::{
    log::log_to_console,
    request::rcd::{clear_status, get_rcd_token, set_status, update_token_login_status},
};
use rcd_messages::client::{ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest};
use rcd_messages::proxy::request_type::RequestType;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_node_ref, AttrValue, Callback, Html, Properties, UseStateHandle,
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

            let behavior_value = UpdatesToHostBehavior::from_u32(behavior.parse::<u32>().unwrap());
            let behavior_value = UpdatesToHostBehavior::to_u32(behavior_value);

            let token = get_rcd_token();

            let request = ChangeUpdatesToHostBehaviorRequest {
                authentication: Some(token.auth()),
                database_name: (*database).clone(),
                table_name: (*table).clone(),
                behavior: behavior_value,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    let database = database.clone();
                    let table = table.clone();
                    clear_status();
                    log_to_console(&x);

                    let reply: ChangeUpdatesToHostBehaviorReply = serde_json::from_str(x).unwrap();
                    let is_authenticated = reply
                        .authentication_result
                        .as_ref()
                        .unwrap()
                        .is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        if reply.is_successful {
                            let behavior = UpdatesToHostBehavior::from_u32(behavior_value);
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
                            let behavior = UpdatesToHostBehavior::from_u32(behavior_value);
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
                } else {
                    let error_message = response.err().unwrap();
                    set_status(error_message);
                }
            });

            request::post(RequestType::ChangeUpdatesToHostBehavior, &body, cb);
        })
    };

    html!(
        <div>
            <p><h1 class="subtitle">{"Change Behavior"}</h1></p>
                <div class ="select is-multiple">
                <select name="set_deletes_from_host_behavior" id="set_deletes_from_host_behavior" ref={ui_behavior} >
                    <option value="0">{"SELECT BEHAVIOR"}</option>
                    <option value="1">{"SendDataHashChange"}</option>
                    <option value="2">{"DoNothing"}</option>
                </select>
                </div>
                <button
                    class="button"
                    type="button"
                    id="updae_behavior"
                    value="Update Behavior"
                    onclick={onclick}>
                        <span class="mdi mdi-eject-circle">{" Update Behavior"}</span>
                </button>
        </div>
    )
}
