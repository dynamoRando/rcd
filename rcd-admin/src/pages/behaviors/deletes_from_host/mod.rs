use rcd_enum::deletes_from_host_behavior::DeletesFromHostBehavior;
use rcd_http_common::url::client::{
    GET_DELETES_FROM_HOST_BEHAVIOR, 
};
use rcd_messages::client::{
    GetDeletesFromHostBehaviorReply,
    GetDeletesFromHostBehaviorRequest,
};
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html,
};

use crate::{
    log::log_to_console,
    pages::{common::{
        select_database::SelectDatabase, select_table::SelectTable,
    }, behaviors::deletes_from_host::view_pending_deletes::ViewPendingDeletes},
    request::{
        self, clear_status, get_databases, get_token, set_status, update_token_login_status,
    },
};

mod view_pending_deletes;

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