use crate::{get_auth_request, get_base_address, request, AppMessage, ContractIntent, RcdAdminApp};
use rcd_http_common::url::client::{ADD_PARTICIPANT, GET_PARTICIPANTS};
use rcd_messages::client::{
    AddParticipantReply, AddParticipantRequest, GetParticipantsReply, GetParticipantsRequest,
};
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view_participants(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.state.page_ui.participants_is_visible;

    let mut db_names: Vec<String> = Vec::new();

    let last_add_result = app.state.conn_ui.add_participant_ui.last_add_result;

    let last_send_result = app
        .state
        .conn_ui
        .send_participant_contract_ui
        .last_send_result;

    for db in &app.state.conn_ui.conn.databases {
        db_names.push(db.database_name.clone());
    }

    let participants = app
        .state
        .conn_ui
        .add_participant_ui
        .current_participants
        .clone();

    html!(
      <div hidden={is_visible}>
          <h1> {"Participants"} </h1>
          <p>
          <h3> {"View Participants"} </h3>
          <label for="execute_sql_dbs">{ "Select Database " }</label>
          <select name="execute_sql_dbs" id="execute_sql_dbs"

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
          <input type="button" id="view_participants" value="View Participants" onclick={link.callback(|_|
            {
                console::log_1(&"clicked".into());
                AppMessage::HandleViewParticipants
            })}/>
          </p>
          <p>
          <ul>
          {
            participants.clone().into_iter().map(|p| {
                let part = p.participant.as_ref().unwrap().clone();
                let status = get_contract_status_string(p.contract_status);
                html!{
                <li value={part.alias.clone()}>{part.alias.clone()}
                    <ul>
                        <li>{"Internal Id: "} { part.internal_participant_guid } </li>
                        <li>{"Self Id: "} { part.participant_guid } </li>
                        <li>{"Alias: "} { part.alias } </li>
                        <li>{"IP 4: "} { part.ip4_address } </li>
                        <li>{"IP 6: "} { part.ip6_address } </li>
                        <li>{"Db Port: "} { part.database_port_number } </li>
                        <li>{"Contract Status: "} { status } </li>
                    </ul>
                </li>}
            }).collect::<Html>()
        }
          </ul>
          </p>
          <h3> {"Add Participant"} </h3>
          <p>
          <label for="execute_sql_dbs">{ "Select Database " }</label>
          <select name="execute_sql_dbs" id="execute_sql_dbs"

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
          <p><label for="participant_alias">{ "Participant Alias" }</label>
          <input type="text" id ="participant_alias" placeholder="Alias" ref={&app.state.conn_ui.add_participant_ui.alias_ui}/></p>
          <p><label for="participant_ip_address">{ "Participant IP Address" }</label>
          <input type="text" id="participant_ip_address" placeholder="127.0.0.1" ref={&app.state.conn_ui.add_participant_ui.ip4_address_ui} /></p>
          <p><label for="participant_db_port">{ "Participant Data Port Number" }</label>
          <input type="text" id="participant_db_port" placeholder="50052" ref={&app.state.conn_ui.add_participant_ui.port_num_ui} /></p>
          <p><label for="participant_http_addr">{ "Participant HTTP Addr" }</label>
          <input type="text" id="participant_http_addr" placeholder="localhost" ref={&app.state.conn_ui.add_participant_ui.participant_http_addr_ui} /></p>
          <p><label for="participant_http_port">{ "Participant HTTP Port Number" }</label>
          <input type="text" id="participant_http_port" placeholder="50055" ref={&app.state.conn_ui.add_participant_ui.participant_http_port_num_ui} /></p>
          </p>
          <input type="button" id="add_participant" value="Add Participant" onclick={link.callback(|_|
            {
                console::log_1(&"clicked".into());
                AppMessage::HandleViewParticipants
            })}/>
            <p><label for="last_add_result">{ "Last Add Participant Result: "}</label>{last_add_result.to_string()}</p>

            <h3>{ "Send Contract To Participant" }</h3>
            <p>
            <label for="send_contract_to_participant">{ "Select Participant " }</label>
          <select name="send_contract_to_participnat" id="send_contract_to_participant"

          onchange={link.batch_callback(|e: Event| {
              if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                  // console::log_1(&"some onchange".into());
                  let intent = ContractIntent::SetParticipantForPendingContractSend(input.value());
                  Some(AppMessage::HandleContract(intent))
              } else {
                  // console::log_1(&"none onchange".into());
                  None
              }
          })}
          >
          <option value="SELECT PARTICIPANT">{"SELECT PARTICIPANT"}</option>
          {
              participants.clone().into_iter().map(|ps| {
                let participant = ps.participant.unwrap();
                  html!{
                  <option value={participant.alias.clone()}>{participant.alias.clone()}</option>}
              }).collect::<Html>()
          }
          </select>
          <input type="button" id="send_contract_to_part" value="Send Contract" onclick={link.callback(|_|
            {
                let intent = ContractIntent::SendContractToParticipant;
                AppMessage::HandleContract(intent)
            })}/>
            <p><label for="last_send_result">{ "Last Send Participant Contract Result: "}</label>{last_send_result.to_string()}</p>
            </p>
          </div>
    )
}

pub fn handle_add_participant(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    let base_address = get_base_address(app);
    let url = format!("{}{}", base_address.clone(), ADD_PARTICIPANT);
    let auth = get_auth_request(app);
    let db_name = &app.state.conn_ui.sql.selected_db_name;

    console::log_1(&"selected db".into());
    console::log_1(&db_name.into());

    let alias_ui = &app.state.conn_ui.add_participant_ui.alias_ui;
    let ip4_ui = &app.state.conn_ui.add_participant_ui.ip4_address_ui;
    let port_ui = &app.state.conn_ui.add_participant_ui.port_num_ui;

    let http_addr_ui = &app
        .state
        .conn_ui
        .add_participant_ui
        .participant_http_addr_ui;
    let http_port_ui = &app
        .state
        .conn_ui
        .add_participant_ui
        .participant_http_port_num_ui;

    let alias_val = alias_ui.cast::<HtmlInputElement>().unwrap().value();
    let ip_val = ip4_ui.cast::<HtmlInputElement>().unwrap().value();

    let http_addr_val = http_addr_ui.cast::<HtmlInputElement>().unwrap().value();
    let http_port_val = http_port_ui
        .cast::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<u32>()
        .unwrap();

    let port_val = port_ui
        .cast::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<u32>()
        .unwrap();

    let request = AddParticipantRequest {
        authentication: Some(auth),
        database_name: db_name.clone(),
        alias: alias_val,
        ip4_address: ip_val,
        port: port_val,
        http_addr: http_addr_val.clone(),
        http_port: http_port_val,
    };

    let request_json = serde_json::to_string(&request).unwrap();

    let callback = ctx
        .link()
        .callback(AppMessage::HandleAddParticipantResponse);

    request::get_data(url, request_json, callback);
}

pub fn handle_add_participant_response(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: AddParticipantReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        app.state.conn_ui.add_participant_ui.last_add_result = reply.is_successful
    }
}

pub fn handle_view_participants(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    let base_address = get_base_address(app);
    let url = format!("{}{}", base_address.clone(), GET_PARTICIPANTS);
    let auth = get_auth_request(app);
    let db_name = &app.state.conn_ui.sql.selected_db_name;

    let request = GetParticipantsRequest {
        authentication: Some(auth),
        database_name: db_name.clone(),
    };

    let request_json = serde_json::to_string(&request).unwrap();

    let callback = ctx
        .link()
        .callback(AppMessage::HandleViewParticipantsResponse);

    request::get_data(url, request_json, callback);
}

pub fn handle_view_participant_response(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: GetParticipantsReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        app.state.conn_ui.add_participant_ui.current_participants = reply.participants.clone();
    }
}

fn get_contract_status_string(status: u32) -> String {
    match status {
        1 => "NotSent".to_string(),
        2 => "Pending".to_string(),
        3 => "Accepted".to_string(),
        4 => "Rejected".to_string(),
        _ => "Unknown".to_string(),
    }
}
