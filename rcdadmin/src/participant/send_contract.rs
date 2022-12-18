use crate::state::databases::RcdDatabases;
use crate::state::participant::RcdParticipants;
use crate::{AppMessage, ContractIntent, RcdAdminApp};
use web_sys::{HtmlSelectElement};
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

    let last_send_result = participants_ui.data.result.send_contract;
    let participants = &participants_ui.data.active.participants;

    html!(
        <div>
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
