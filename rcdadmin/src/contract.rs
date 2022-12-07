use crate::{get_auth_request, get_base_address, request, AppMessage, ContractIntent, RcdAdminApp};
use rcd_http_common::url::{GENERATE_CONTRACT, SEND_CONTRACT_TO_PARTICIPANT, GET_ACTIVE_CONTRACT};
use rcd_messages::client::{GenerateContractReply, GenerateContractRequest, GetActiveContractRequest, GetActiveContractReply, SendParticipantContractRequest, SendParticipantContractReply};
use rcd_messages::formatter;
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view_contracts(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.state.page_ui.contract_is_visible;
    let text = app
        .state
        .conn_ui
        .sql
        .current_contract
        .contract_markdown
        .clone();

    let active_contract = app
        .state
        .conn_ui
        .sql
        .current_contract
        .active_contract_markdown
        .clone();

    let mut db_names: Vec<String> = Vec::new();

    let last_gen_result = app
        .state
        .conn_ui
        .sql
        .current_contract
        .contract_gen_ui
        .last_gen_result;

    for db in &app.state.conn_ui.conn.databases {
        db_names.push(db.database_name.clone());
    }

    html!(
      <div hidden={is_visible}>
          <h1> {"Contracts"} </h1>
          <p>
          <input type="button" id="view_pending_contracts" value="View Pending Contracts" onclick={link.callback(|_|
              {
                  AppMessage::HandleContract(ContractIntent::GetPending)
              })}/>
          <input type="button" id="view_accepted_contracts" value="View Accepted Contracts" onclick={link.callback(|_|
              {
                  AppMessage::HandleContract(ContractIntent::GetAccepted)
              })}/>
          <input type="button" id="accepted_contracts" value="Accept Contract" onclick={link.callback(|_|
              {
                  AppMessage::HandleContract(ContractIntent::AcceptContract("".to_string()))
              })}/>
              <input type="button" id="reject_contracts" value="Reject Contract" onclick={link.callback(|_|
                {
                    AppMessage::HandleContract(ContractIntent::RejectContract("".to_string()))
                })}/>
          </p>
          <p>
          <textarea rows="5" cols="60"  id ="contract_details" placeholder="Contract Details Will Be Here As Markdown Table"
          ref={&app.state.conn_ui.sql.current_contract.contract_detail_ui} value={text}/>
          </p>
          <h2>{ "Generate Contract" }</h2>
          <p>{"Note: Before you can generate a contract, you must ensure that every user table in your target
          database has a Logical Storage Policy applied for it." }</p>
          <p>
          <label for="gen_contract_db">{ "Select Database " }</label>
          <select name="gen_contract_db" id="gen_contract_db"

          onchange={link.batch_callback(|e: Event| {
              if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                  // console::log_1(&"some onchange".into());
                  Some(AppMessage::SetExecuteSQLDatabase(input.value()))
              } else {
                  // console::log_1(&"none onchange".into());
                  None
              }
          })}
          >
          <option value="SELECT DATABASE">{"SELECT DATABASE"}</option>
          {
              db_names.clone().into_iter().map(|name| {
                  // console::log_1(&name.clone().into());
                  html!{
                  <option value={name.clone()}>{name.clone()}</option>}
              }).collect::<Html>()
          }
          </select>
          </p>
          <label for="gen_contract_hostname">{ "Host Name" }</label>
          <p>
          <textarea rows="2" cols="60"  id ="gen_contract_hostname" placeholder="Name you wish to identify to participants"
          ref={&app.state.conn_ui.sql.current_contract.contract_gen_ui.host_name_ui}/>
          </p>
          <label for="gen_contract_desc">{ "Description" }</label>
          <p>
          <textarea rows="5" cols="60"  id ="gen_contract_desc" placeholder="A bried description of the purpose of this database"
          ref={&app.state.conn_ui.sql.current_contract.contract_gen_ui.contract_desc_ui}/>
          </p>
          <p>
          <label for="set_remote_delete_behavior">{ "Set Remote Delete Behavior" }</label>
          <p>
          <select name="set_remote_delete_behavior" id="set_remote_delete_behavior"
          onchange={link.batch_callback(|e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                // console::log_1(&"some onchange".into());
                let val = input.value();
                Some(AppMessage::SetRemoteDeleteBehavior(val.parse::<u32>().unwrap()))
            } else {
                // console::log_1(&"none onchange".into());
                None
            }
        })}
          >
          <option value="0">{"SELECT BEHAVIOR"}</option>
          <option value="1">{"Ignore"}</option>
          <option value="2">{"AutoDelete"}</option>
          <option value="3">{"UpdateStatusOnly"}</option>
          </select>
          </p>
          </p>
          <p>{"Explanation: The Remote Delete Behavior determines how reference rows in the host database will be updated.
          The options are: "}
          <ul>
            <li>{"Ignore: If the participant has deleted the row, the host will take no action."}</li>
            <li>{"AutoDelete: If the participant has deleted the row, the host will also delete the reference on it's side."}</li>
            <li>{"UpdateStatusOnly: If the participant has deleted the row, the host will mark the reference as deleted, but keep the reference to the row."}</li>
          </ul>
          {"Note that a reference row in the host database, while having it's target marked as deleted, can itself be deleted at any time."}
          </p>
          <input type="button" id="generate_new_contract" value="Generate Contract" onclick={link.callback(move |_|
            {
                console::log_1(&"generate_new_contract".into());

                let intent = ContractIntent::GenerateContract;
                AppMessage::HandleContract(intent)
            })}/>
            <p><label for="last_gen_result">{ "Last Gen Result: "}</label>{last_gen_result.to_string()}</p>
            <h2>{ "View Active Contract" }</h2>
            <p>
          <label for="gen_contract_db">{ "Select Database " }</label>
          <select name="gen_contract_db" id="gen_contract_db"

          onchange={link.batch_callback(|e: Event| {
              if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                  // console::log_1(&"some onchange".into());
                  Some(AppMessage::SetExecuteSQLDatabase(input.value()))
              } else {
                  // console::log_1(&"none onchange".into());
                  None
              }
          })}
          >
          <option value="SELECT DATABASE">{"SELECT DATABASE"}</option>
          {
              db_names.clone().into_iter().map(|name| {
                  // console::log_1(&name.clone().into());
                  html!{
                  <option value={name.clone()}>{name.clone()}</option>}
              }).collect::<Html>()
          }
          </select>
          <input type="button" id="view_active_contract_for_db" value="View Active Contract" onclick={link.callback(move |_|
            {
                console::log_1(&"view_active_contract".into());
                let intent = ContractIntent::ViewCurrentContract;
                AppMessage::HandleContract(intent)
            })}/>
          </p>
          <p>
          <textarea rows="5" cols="60" id="current_contract_details" placeholder="Active Contract Details Will Be Here As Markdown Table"
          ref={&app.state.conn_ui.sql.current_contract.contract_detail_db_ui} value={active_contract}/>
          </p>
          </div>
    )
}

pub fn handle_contract_intent(
    app: &mut RcdAdminApp,
    intent: ContractIntent,
    link: &Scope<RcdAdminApp>,
) {
    match intent {
        ContractIntent::Unknown => todo!(),
        ContractIntent::GetPending => todo!(),
        ContractIntent::GetAccepted => todo!(),
        ContractIntent::GetRejected => todo!(),
        ContractIntent::AcceptContract(_) => todo!(),
        ContractIntent::GenerateContract => {
            let base_address = get_base_address(app);
            let url = format!("{}{}", base_address.clone(), GENERATE_CONTRACT);
            let auth = get_auth_request(app);
            let db_name = &app.state.conn_ui.sql.selected_db_name;

            let host_name_ui = &app
                .state
                .conn_ui
                .sql
                .current_contract
                .contract_gen_ui
                .host_name_ui;
            let host_name = host_name_ui.cast::<HtmlInputElement>().unwrap().value();

            let desc_ui = &app
                .state
                .conn_ui
                .sql
                .current_contract
                .contract_gen_ui
                .contract_desc_ui;
            let description = desc_ui.cast::<HtmlInputElement>().unwrap().value();

            let behavior = &app
                .state
                .conn_ui
                .sql
                .current_contract
                .contract_gen_ui
                .contract_gen_remote_delete_behavior;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let request = GenerateContractRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                host_name: host_name,
                description: description,
                remote_delete_behavior: *behavior,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = link.callback(AppMessage::HandleContractResponse);

            request::get_data(url, request_json, callback);
        }
        ContractIntent::SendContractToParticipant => {
            let base_address = get_base_address(app);
            let url = format!("{}{}", base_address.clone(), SEND_CONTRACT_TO_PARTICIPANT);
            let auth = get_auth_request(app);
            let db_name = &app.state.conn_ui.sql.selected_db_name;
            let participant_alias = app.state.conn_ui.send_participant_contract_ui.participant_alias.clone();

            let request = SendParticipantContractRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                participant_alias: participant_alias,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = link.callback(AppMessage::HandleContractSendToParticipant);

            request::get_data(url, request_json, callback);
        },
        ContractIntent::RejectContract(_) => todo!(),
        ContractIntent::ViewCurrentContract => {
            let base_address = get_base_address(app);
            let url = format!("{}{}", base_address.clone(), GET_ACTIVE_CONTRACT);
            let auth = get_auth_request(app);
            let db_name = &app.state.conn_ui.sql.selected_db_name;

            let request = GetActiveContractRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = link.callback(AppMessage::HandleGetActiveContractResponse);

            request::get_data(url, request_json, callback);
        },
        ContractIntent::SetParticipantForPendingContractSend(participant_alias) => {
            app.state.conn_ui.send_participant_contract_ui.participant_alias = participant_alias.clone();
        },
    }
}

pub fn handle_contract_response(app: &mut RcdAdminApp, json_response: String) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: GenerateContractReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        let mut result_message = String::new();

        result_message =
            result_message + &format!("Is result successful: {}", reply.is_successful.to_string());

        console::log_1(&result_message.to_string().clone().into());
        app.state
            .conn_ui
            .sql
            .current_contract
            .contract_gen_ui
            .last_gen_result = reply.is_successful;
    }
}

pub fn handle_view_active_contract(app: &mut RcdAdminApp, json_response: String) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: GetActiveContractReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        let contract_markdown = formatter::markdown::contract::contract_to_markdown_table(&reply.contract.unwrap());
        app.state.conn_ui.sql.current_contract.active_contract_markdown = contract_markdown;
    }
}

pub fn handle_send_contract_to_participant_response(app: &mut RcdAdminApp, json_response: String) { 
    console::log_1(&json_response.to_string().clone().into());
    let reply: SendParticipantContractReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
            app.state.conn_ui.send_participant_contract_ui.last_send_result = reply.is_sent;
    }
}

#[allow(dead_code)]
fn remote_delete_behavior_status_to_text(behavior: u32) -> String {
    match behavior {
        0 => "Unknown".to_string(),
        1 => "Ignore".to_string(),
        2 => "AutoDelete".to_string(),
        3 => "UpdateStatusOnly".to_string(),
        _ => "Unknown".to_string(),
    }
}
