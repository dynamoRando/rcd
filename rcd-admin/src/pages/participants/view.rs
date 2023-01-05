use rcd_http_common::url::client::GET_PARTICIPANTS;
use rcd_messages::client::{GetParticipantsReply, GetParticipantsRequest, ParticipantStatus};

use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, Properties, UseStateHandle,
};

use crate::{
    log::log_to_console,
    pages::{common::select_database::SelectDatabase, participants::ActiveDbProps},
    request::{self, get_token},
};

#[derive(Properties, PartialEq)]
pub struct ParticipantProps {
    pub participants: UseStateHandle<Vec<ParticipantStatus>>,
}

#[function_component]
pub fn ViewParticipants(ActiveDbProps { active_db }: &ActiveDbProps) -> Html {
    let active_db = active_db.clone();

    let participant_details = use_state_eq(move || {
        let details: Vec<ParticipantStatus> = Vec::new();
        details
    });

    let onclick_db = {
        let participant_details = participant_details.clone();

        Callback::from(move |db_name: String| {
            if db_name != "" && db_name != "SELECT DATABASE" {
                let participant_details = participant_details.clone();

                let token = get_token();
                let auth = token.auth().clone();

                let get_participants_request = GetParticipantsRequest {
                    authentication: Some(auth),
                    database_name: db_name.clone(),
                };

                let request_json = serde_json::to_string(&get_participants_request).unwrap();
                let url = format!("{}{}", token.addr, GET_PARTICIPANTS);

                let cb = Callback::from(move |response: AttrValue| {
                    log_to_console(response.to_string());

                    let participant_details = participant_details.clone();
                    let reply: GetParticipantsReply =
                        serde_json::from_str(&&response.to_string()).unwrap();

                    if reply.authentication_result.unwrap().is_authenticated {
                        participant_details.set(reply.participants.clone());
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
                <p>< SelectDatabase active_db_name={active_db} onclick_db={onclick_db}/></p>
            </p>
            < ViewParticipantsForDb participants={participant_details} />
        </div>
    }
}

#[function_component]
pub fn ViewParticipantsForDb(ParticipantProps { participants }: &ParticipantProps) -> Html {
    let participants = participants.clone();

    html!(
        <div>
            <div class="container">
                <p><h1 class="subtitle">{"Participant Details"}</h1></p>
                <p>
                    <div class="table-container">
                    <table class="table is-narrow">
                        <thead>
                            <tr>
                                <th>{"GUID"}</th>
                                <th>{"Alias"}</th>
                                <th>{"IP4"}</th>
                                <th>{"IP6"}</th>
                                <th>{"DB Port"}</th>
                                <th>{"Internal GUID"}</th>
                                <th>{"HTTP Addr"}</th>
                                <th>{"HTTP Port"}</th>
                                <th>{"Contract Status"}</th>
                            </tr>
                        </thead>
                        {
                            (*participants).clone().into_iter().map(|p|{
                                let participant = p.participant.as_ref().unwrap().clone();
                                let status = get_contract_status_string(p.contract_status);
                                html!{
                                    <tr>
                                        <td>{participant.participant_guid.clone()}</td>
                                        <td>{participant.alias.clone()}</td>
                                        <td>{participant.ip4_address.clone()}</td>
                                        <td>{participant.ip6_address.clone()}</td>
                                        <td>{participant.database_port_number.to_string()}</td>
                                        <td>{participant.internal_participant_guid.clone()}</td>
                                        <td>{participant.http_addr.clone()}</td>
                                        <td>{participant.http_port.to_string()}</td>
                                        <td>{status}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </table>
                    </div>
                </p>
            </div>
        </div>
    )
}

fn get_contract_status_string(status: u32) -> String {
    match status {
        1 => "Not Sent".to_string(),
        2 => "Pending".to_string(),
        3 => "Accepted".to_string(),
        4 => "Rejected".to_string(),
        _ => "Unknown".to_string(),
    }
}
