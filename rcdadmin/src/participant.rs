use crate::urls::url_add_participant;
use crate::{get_auth_request, get_base_address, request, AppMessage, RcdAdminApp};
use rcd_messages::client::{AddParticipantReply, AddParticipantRequest};
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view_participants(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.state.page_ui.participants_is_visible;

    let mut db_names: Vec<String> = Vec::new();

    let last_add_result = app.state.conn_ui.add_participant_ui.last_add_result;

    for db in &app.state.conn_ui.conn.databases {
        db_names.push(db.database_name.clone());
    }

    html!(
      <div hidden={is_visible}>
          <h1> {"Participants"} </h1>
          <p>
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
              db_names.into_iter().map(|name| {
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
          </p>
          <input type="button" id="add_participant" value="Add Participant" onclick={link.callback(|_|
            {
                console::log_1(&"clicked".into());
                AppMessage::HandleAddParticipant
            })}/>
            <p><label for="last_add_result">{ "Last Add Participant Result: "}</label>{last_add_result.to_string()}</p>
          </div>
    )
}

pub fn handle_add_participant(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    let base_address = get_base_address(app);
    let url = format!("{}{}", base_address.clone(), url_add_participant());
    let auth = get_auth_request(app);
    let db_name = &app.state.conn_ui.sql.selected_db_name;

    console::log_1(&"selected db".into());
    console::log_1(&db_name.into());

    let alias_ui = &app.state.conn_ui.add_participant_ui.alias_ui;
    let ip4_ui = &app.state.conn_ui.add_participant_ui.ip4_address_ui;
    let port_ui = &app.state.conn_ui.add_participant_ui.port_num_ui;

    let alias_val = alias_ui.cast::<HtmlInputElement>().unwrap().value();
    let ip_val = ip4_ui.cast::<HtmlInputElement>().unwrap().value();
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
