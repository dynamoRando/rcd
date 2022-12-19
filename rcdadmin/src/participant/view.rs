use crate::state::databases::RcdDatabases;
use crate::state::participant::RcdParticipants;
use crate::{get_auth_request, get_base_address, request, AppMessage, RcdAdminApp};
use rcd_http_common::url::client::GET_PARTICIPANTS;
use rcd_messages::client::{GetParticipantsReply, GetParticipantsRequest};
use web_sys::{console, HtmlSelectElement};
use yew::prelude::*;
use yew::{html::Scope, Html};

use super::get_contract_status_string;

pub fn view(
    databases: &RcdDatabases,
    participants_ui: &RcdParticipants,
    link: &Scope<RcdAdminApp>,
) -> Html {
    let mut db_names: Vec<String> = Vec::new();
    for db in &databases.data.databases {
        db_names.push(db.database_name.clone());
    }

    let participants = &participants_ui.data.active.participants;

    html!(
        <div>
        <p>
          <h3> {"View Participants"} </h3>
          <label for="execute_sql_dbs">{ "Select Database " }</label>
          <select name="execute_sql_dbs" id="execute_sql_dbs"

          onchange={link.batch_callback(|e: Event| {
              if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                  Some(AppMessage::SetExecuteSQLDatabase(input.value()))
              } else {
                  None
              }
          })}
          >
          <option value="SELECT DATABASE">{"SELECT DATABASE"}</option>
          {
              db_names.clone().into_iter().map(|name| {
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
        </div>
    )
}

pub fn request(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    let base_address = get_base_address(&app.connection.data);
    let url = format!("{}{}", base_address.clone(), GET_PARTICIPANTS);
    let auth = get_auth_request(&app.connection.data);
    let db_name = &app.databases.data.active.database_name;

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

pub fn response(app: &mut RcdAdminApp, _ctx: &Context<RcdAdminApp>, json_response: AttrValue) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: GetParticipantsReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        app.participants.data.active.participants = reply.participants.clone();
    }
}
