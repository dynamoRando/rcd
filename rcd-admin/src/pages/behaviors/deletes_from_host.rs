use rcd_enum::deletes_from_host_behavior::DeletesFromHostBehavior;
use rcd_http_common::url::client::{
    ACCEPT_PENDING_ACTION, GET_DELETES_FROM_HOST_BEHAVIOR, GET_PENDING_ACTIONS,
};
use rcd_messages::client::{
    AcceptPendingActionReply, AcceptPendingActionRequest, GetDeletesFromHostBehaviorReply,
    GetDeletesFromHostBehaviorRequest, GetPendingActionsReply, GetPendingActionsRequest,
    PendingStatement,
};
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, Properties, UseStateHandle,
};

use crate::{
    log::log_to_console,
    pages::common::{
        pending_actions::PendingActions, select_database::SelectDatabase, select_table::SelectTable,
    },
    request::{
        self, clear_status, get_databases, get_token, set_status, update_token_login_status,
    },
};

#[function_component]
pub fn DeletesFromHost() -> Html {
    let active_database = use_state_eq(move || String::from(""));
    let active_table_database = active_database.clone();
    let active_database_pending = active_database.clone();

    let active_table = use_state_eq(move || String::from(""));
    let active_table_pending = active_table.clone();

    let behavior_type_state = use_state_eq(move || String::from(""));

    let table_names = use_state_eq(move || {
        let x: Vec<String> = Vec::new();
        return x;
    });

    let onclick_db = {
        let table_names = table_names.clone();
        Callback::from(move |db_name: String| {
            let databases = get_databases();

            let database = databases
                .iter()
                .find(|x| x.database_name.as_str() == db_name)
                .unwrap()
                .clone();

            let mut names: Vec<String> = Vec::new();

            for table in &database.tables {
                names.push(table.table_name.clone());
            }

            table_names.set(names);
        })
    };

    let onclick_table = {
        let active_database = active_database.clone();
        let behavior_type_state = behavior_type_state.clone();
        Callback::from(move |table_name: String| {
            let behavior_type_state = behavior_type_state.clone();
            if table_name != "" {
                log_to_console(table_name.clone());

                let token = get_token();

                let request = GetDeletesFromHostBehaviorRequest {
                    authentication: Some(token.auth()),
                    database_name: active_database.to_string(),
                    table_name: table_name,
                };

                let body = serde_json::to_string(&request).unwrap();
                let url = format!("{}{}", token.addr, GET_DELETES_FROM_HOST_BEHAVIOR);

                let cb = Callback::from(move |response: Result<AttrValue, String>| {
                    if response.is_ok() {
                        let response = response.unwrap();
                        log_to_console(response.to_string());
                        clear_status();

                        let reply: GetDeletesFromHostBehaviorReply =
                            serde_json::from_str(&response).unwrap();

                        let is_authenticated = reply
                            .authentication_result
                            .as_ref()
                            .unwrap()
                            .is_authenticated;
                        update_token_login_status(is_authenticated);

                        if is_authenticated {
                            let behavior = reply.behavior;
                            let behavior_value =
                                DeletesFromHostBehavior::from_u32(behavior).as_string();
                            behavior_type_state.set(behavior_value);
                        }
                    } else {
                        set_status(response.err().unwrap());
                    }
                });

                request::post(url, body, cb);
            }
        })
    };

    html! {
        <div>
            <div class="box">
                <p><h1 class="subtitle">{"Deletes From Host"}</h1></p>
                <p><label for="databases">{ "Select Database " }</label></p>
                <p>< SelectDatabase active_db_name={active_database} onclick_db={onclick_db}/></p>
                <p><label for="tables">{ "Select Table " }</label></p>
                <p>< SelectTable
                    active_database_name={active_table_database}
                    active_table_name = {active_table}
                    onclick_table={onclick_table}/>
                </p>
                <p>{"Current Behavior: "}</p>
                <p>{(*behavior_type_state).clone()}</p>
                < ViewPendingDeletes active_database={active_database_pending} active_table={active_table_pending}/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ViewPendingDeleteProps {
    pub active_database: UseStateHandle<String>,
    pub active_table: UseStateHandle<String>,
}

#[function_component]
pub fn ViewPendingDeletes(
    ViewPendingDeleteProps {
        active_database,
        active_table,
    }: &ViewPendingDeleteProps,
) -> Html {
    let pending_actions = use_state_eq(move || {
        let x: Vec<PendingStatement> = Vec::new();
        return x;
    });

    let active_database = active_database.clone();
    let active_table = active_table.clone();

    let callback_accept = {
        let active_database = active_database.clone();
        let active_table = active_table.clone();

        Callback::from(move |accepted_row_id: u32| {
            let active_database = active_database.clone();
            let active_table = active_table.clone();

            let token = get_token();
            let url = format!("{}{}", token.addr, ACCEPT_PENDING_ACTION);
            let request = AcceptPendingActionRequest {
                authentication: Some(token.auth()),
                database_name: (*active_database).clone(),
                table_name: (*active_table).clone(),
                row_id: accepted_row_id,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if response.is_ok() {
                    clear_status();
                    let response = response.unwrap();
                    log_to_console(response.clone().to_string());

                    let reply: AcceptPendingActionReply = serde_json::from_str(&response).unwrap();
                    let is_authenticated = reply
                        .authentication_result
                        .as_ref()
                        .unwrap()
                        .is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        todo!()
                    }
                } else {
                    let error_message = response.err().unwrap();
                    set_status(error_message);
                }
            });

            request::post(url, body, cb);
        })
    };

    let callback_reject = {
        let active_database = active_database.clone();
        let active_table = active_table.clone();

        Callback::from(move |rejected_row_id: u32| {
            // LOL: We never wrote a reject message
            let active_database = active_database.clone();
            let active_table = active_table.clone();

            let token = get_token();
            let url = format!("{}{}", token.addr, ACCEPT_PENDING_ACTION);
            let request = AcceptPendingActionRequest {
                authentication: Some(token.auth()),
                database_name: (*active_database).clone(),
                table_name: (*active_table).clone(),
                row_id: rejected_row_id,
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if response.is_ok() {
                    clear_status();
                    let response = response.unwrap();
                    log_to_console(response.clone().to_string());

                    let reply: AcceptPendingActionReply = serde_json::from_str(&response).unwrap();
                    let is_authenticated = reply
                        .authentication_result
                        .as_ref()
                        .unwrap()
                        .is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        todo!()
                    }
                } else {
                    let error_message = response.err().unwrap();
                    set_status(error_message);
                }
            });

            request::post(url, body, cb);
            todo!("LOL: We never wrote a reject message");
        })
    };

    let onclick_view = {
        let active_database = active_database.clone();
        let active_table = active_table.clone();
        let pending_actions = pending_actions.clone();

        Callback::from(move |_| {
            let pending_actions = pending_actions.clone();
            let token = get_token();
            let url = format!("{}{}", token.addr, GET_PENDING_ACTIONS);
            let request = GetPendingActionsRequest {
                authentication: Some(token.auth()),
                database_name: (*active_database).clone(),
                table_name: (*active_table).clone(),
                action: "DELETE".to_string(),
            };

            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if response.is_ok() {
                    clear_status();
                    let response = response.unwrap();
                    log_to_console(response.clone().to_string());

                    let reply: GetPendingActionsReply = serde_json::from_str(&response).unwrap();
                    let is_authenticated = reply
                        .authentication_result
                        .as_ref()
                        .unwrap()
                        .is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        let actions = reply.pending_statements.clone();
                        pending_actions.set(actions);
                    }
                } else {
                    let error_message = response.err().unwrap();
                    set_status(error_message);
                }
            });

            request::post(url, body, cb);
        })
    };

    html!(
        <div>
            <p><h1 class="subtitle">{"View Pending Deletes From Host"}</h1></p>
                <button class="button" type="button" id="view_pending_deletes" value="View Pending Deletes" onclick={onclick_view}>
                    <span class="mdi mdi-magnify"></span>{" View Pending Deletes"}
                </button>
            <p><h1 class="subtitle">{"Pending Deletes From Host"}</h1></p>
            < PendingActions pending_actions={pending_actions} onclick_accept={callback_accept} onclick_reject={callback_reject} />
        </div>
    )
}
