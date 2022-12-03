use crate::{AppMessage, ContractIntent, RcdAdminApp, TableIntent};
use web_sys::{console, HtmlSelectElement};
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

    let mut db_names: Vec<String> = Vec::new();

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
              db_names.into_iter().map(|name| {
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
          <select name="set_remote_delete_behavior" id="set_remote_delete_behavior">
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
          </div>
    )
}

pub fn handle_contract_intent(app: &RcdAdminApp, intent: ContractIntent) {
    match intent {
        ContractIntent::Unknown => todo!(),
        ContractIntent::GetPending => todo!(),
        ContractIntent::GetAccepted => todo!(),
        ContractIntent::GetRejected => todo!(),
        ContractIntent::AcceptContract(_) => todo!(),
        ContractIntent::GenerateContract => todo!(),
        ContractIntent::SendContractToParticipant(_) => todo!(),
        ContractIntent::RejectContract(_) => todo!(),
    }
}

pub fn handle_contract_response(app: &RcdAdminApp, json_response: String){
    todo!()
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

