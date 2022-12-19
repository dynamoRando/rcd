use crate::state::databases::RcdDatabases;
use crate::state::participant::RcdParticipants;
use crate::{get_auth_request, get_base_address, request, AppMessage, RcdAdminApp};
use rcd_http_common::url::client::ADD_PARTICIPANT;
use rcd_messages::client::{AddParticipantReply, AddParticipantRequest};
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view(
    databases: &RcdDatabases,
    participants_ui: &RcdParticipants,
    link: &Scope<RcdAdminApp>,
) -> Html {
    let mut db_names: Vec<String> = Vec::new();
    for db in &databases.data.databases {
        db_names.push(db.database_name.clone());
    }

    let last_add_result = participants_ui.data.result.add_participant;

    html!(
        <div>
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
        <input type="text" id ="participant_alias" placeholder="Alias" ref={&participants_ui.ui.add.alias}/></p>
        <p><label for="participant_ip_address">{ "Participant IP Address" }</label>
        <input type="text" id="participant_ip_address" placeholder="127.0.0.1" ref={&participants_ui.ui.add.addr} /></p>
        <p><label for="participant_db_port">{ "Participant Data Port Number" }</label>
        <input type="text" id="participant_db_port" placeholder="50052" ref={&participants_ui.ui.add.port} /></p>
        <p><label for="participant_http_addr">{ "Participant HTTP Addr" }</label>
        <input type="text" id="participant_http_addr" placeholder="localhost" ref={&participants_ui.ui.add.http_addr} /></p>
        <p><label for="participant_http_port">{ "Participant HTTP Port Number" }</label>
        <input type="text" id="participant_http_port" placeholder="50055" ref={&participants_ui.ui.add.http_port} /></p>
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

pub fn request(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    let ui = &app.participants.ui.add;

    let base_address = get_base_address(&app.connection.data);
    let url = format!("{}{}", base_address.clone(), ADD_PARTICIPANT);
    let auth = get_auth_request(&app.connection.data);
    let db_name = &app.databases.data.active.database_name;

    console::log_1(&"selected db".into());
    console::log_1(&db_name.into());

    let alias_ui = &ui.alias;
    let ip4_ui = &ui.addr;
    let port_ui = &ui.port;

    let http_addr_ui = &ui.http_addr;
    let http_port_ui = &ui.http_port;

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

pub fn response(app: &mut RcdAdminApp, _ctx: &Context<RcdAdminApp>, json_response: AttrValue) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: AddParticipantReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        app.participants.data.result.add_participant = reply.is_successful
    }
}
