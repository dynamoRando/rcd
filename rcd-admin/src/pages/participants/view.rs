use rcd_http_common::url::client::SEND_CONTRACT_TO_PARTICIPANT;
use rcd_messages::client::{
    ParticipantStatus, SendParticipantContractReply, SendParticipantContractRequest,
};

use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_state_eq, AttrValue, Callback, Html, Properties, UseStateHandle,
};

use crate::{
    log::log_to_console,
    pages::{common::select_database::SelectDatabase, participants::ActiveDbProps},
    request::{self, clear_status, get_client, get_token, set_status, update_token_login_status},
};

#[derive(Properties, PartialEq)]
pub struct ParticipantProps {
    pub participants: UseStateHandle<Vec<ParticipantStatus>>,
    pub db_name: UseStateHandle<String>,
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
            if !db_name.is_empty() && db_name != "SELECT DATABASE" {
                let participant_details = participant_details.clone();

                let mut client = get_client();
                let token = get_token();
                spawn_local(async move {
                    let reply = client.get_participants(token.auth(), &db_name).await;

                    match reply {
                        Ok(reply) => {
                            clear_status();

                            let is_authenticated = reply
                                .authentication_result
                                .as_ref()
                                .unwrap()
                                .is_authenticated;
                            update_token_login_status(is_authenticated);

                            if is_authenticated {
                                participant_details.set(reply.participants);
                            }
                        }
                        Err(e) => {
                            set_status(e);
                        }
                    }
                });
            }
        })
    };

    html! {
        <div>
        <h1 class="subtitle"> {"View Participants"} </h1>
            <p>
                <p><label for="execute_sql_dbs">{ "Select Database " }</label></p>
                <p>< SelectDatabase active_db_name={active_db.clone()} onclick_db={onclick_db}/></p>
            </p>
            < ViewParticipantsForDb participants={participant_details} db_name={active_db.clone()} />
        </div>
    }
}

#[function_component]
pub fn ViewParticipantsForDb(
    ParticipantProps {
        participants,
        db_name,
    }: &ParticipantProps,
) -> Html {
    let participants = participants.clone();
    let database_name = (*db_name).clone().to_string();
    let participant_send_contract_result = use_state_eq(move || String::from(""));

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
                                <th>{"Send Contract?"}</th>
                            </tr>
                        </thead>
                        {
                            (*participants).clone().into_iter().map(|p|{
                                let participant = p.participant.as_ref().unwrap().clone();
                                let status = get_contract_status_string(p.contract_status);
                                let participant_send_contract_result = participant_send_contract_result.clone();
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
                                        <td><button class="button" onclick=
                                        {
                                            let database_name = database_name.clone();
                                            let participant_send_contract_result = participant_send_contract_result.clone();
                                            move |_| {
                                                let participant_send_contract_result = participant_send_contract_result.clone();
                                                let alias = participant.alias.clone();
                                                let token = get_token().clone();

                                                let request = SendParticipantContractRequest {
                                                    authentication: Some(token.auth().clone()),
                                                    database_name: database_name.clone(),
                                                    participant_alias: alias.clone()
                                                };

                                                let json_request = serde_json::to_string(&request).unwrap();

                                                log_to_console(json_request.clone());

                                                let url = format!("{}{}", token.addr, SEND_CONTRACT_TO_PARTICIPANT);

                                                log_to_console(url.clone());

                                                let cb = Callback::from(move |response: Result<AttrValue, String>| {

                                                    if let Ok(ref x) = response {
                                                        let participant_send_contract_result = participant_send_contract_result.clone();
                                                        log_to_console(x.to_string());

                                                        let reply: SendParticipantContractReply =
                                                        serde_json::from_str(x).unwrap();

                                                        let is_authenticated = reply.authentication_result.unwrap().is_authenticated;
                                                        update_token_login_status(is_authenticated);

                                                        if is_authenticated {
                                                            if reply.is_sent {
                                                                let message = format!("{}{}{}","Contract sent to
                                                                participant ", alias.clone(), " is successful.");
                                                                participant_send_contract_result.set(message);
                                                            } else {
                                                                let message = format!("{}{}{}{}","Contract sent to
                                                                participant ", alias.clone(), " is NOT successful. Reason: ", reply.contract_status);
                                                                participant_send_contract_result.set(message);
                                                            }
                                                        }
                                                    } else {
                                                        set_status(response.err().unwrap());
                                                    }
                                                });

                                                request::post(url, json_request.clone(), cb.clone())
                                            }
                                        }>{"Send Active Contract"}</button></td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </table>
                    </div>
                    <p><h1 class="subtitle">{"Send Participant Result"}</h1></p>
                    <p>{(*participant_send_contract_result).clone()} </p>
                    <p>{"Note: If you have already sent the same contract to a participant, by default the client will not re-save a contract for the same database with the same version id."}</p>
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
