use rcd_http_common::url::client::GET_PARTICIPANTS;
use rcd_messages::client::{GetParticipantsRequest, GetParticipantsReply};
use web_sys::HtmlInputElement;
use yew::{function_component, Html, html, use_node_ref, use_state_eq, Callback, AttrValue};

use crate::request::{get_token, self, get_databases};

#[function_component]
pub fn ViewParticipants() -> Html {

    let active_database = use_state_eq(move || None);
    
    let participant_aliases = use_state_eq(move || {
        let list: Vec<String> = Vec::new();
        Some(list)
    });

    // drop-down
    let ui_active_database = use_node_ref();

    let mut database_names: Vec<String> = Vec::new();

    let databases = get_databases();

    for database in &databases {
        database_names.push(database.database_name.clone());
    }

    let onchange_db = {
        let active_database = active_database.clone();
        let ui_active_database = ui_active_database.clone();
        let participant_aliases = participant_aliases.clone();

        Callback::from(move |_| {
            let participant_aliases = participant_aliases.clone();
            let active_database = active_database.clone();
            let ui_active_database = ui_active_database.clone();

            let selected_db = ui_active_database.cast::<HtmlInputElement>();

            if selected_db.is_some() {
                let participant_aliases = participant_aliases.clone();

                let selected_db_val = ui_active_database
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                active_database.set(Some(selected_db_val.clone()));

                let token = get_token();
                let auth = token.auth().clone();

                let get_participants_request = GetParticipantsRequest {
                    authentication: Some(auth),
                    database_name: selected_db_val.clone(),
                };

                let request_json = serde_json::to_string(&get_participants_request).unwrap();
                let url = format!("{}{}", token.addr, GET_PARTICIPANTS);

                let cb = Callback::from(move |response: AttrValue| {
                    let participant_aliases = participant_aliases.clone();
                    let reply: GetParticipantsReply =
                        serde_json::from_str(&&response.to_string()).unwrap();

                    if reply.authentication_result.unwrap().is_authenticated {
                        let participants = reply.participants.clone();

                        let mut aliases: Vec<String> = Vec::new();
                        for p in &participants {
                            aliases.push(p.participant.as_ref().unwrap().alias.clone());
                        }

                        participant_aliases.set(Some(aliases));
                    }
                });

                request::get_data(url, request_json, cb);
            }
        })
    };

    html! {
        <div>
        <h1 class="subtitle"> {"View Participants"} </h1>
            <p>
                <p><label for="execute_sql_dbs">{ "Select Database " }</label></p>
                <p>
                    <div class="select is-multiple">
                        <select
                            name="execute_sql_dbs"
                            id="execute_sql_dbs"
                            ref={&ui_active_database}
                            onchange={onchange_db}
                        >
                        <option value="SELECT DATABASE">{"SELECT DATABASE"}</option>
                        {
                            database_names.into_iter().map(|name| {
                                // console::log_1(&name.clone().into());
                                html!{
                                <option value={name.clone()}>{name.clone()}</option>}
                            }).collect::<Html>()
                        }
                        </select>
                    </div>
                </p>
            </p>
        </div>
    }
}