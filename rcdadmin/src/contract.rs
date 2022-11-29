use crate::{AppMessage, ContractIntent, RcdAdminApp};
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view_contracts(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {

    let text = app.state.conn_ui.sql.current_contract.contract_markdown.clone();

    html!(
      <div>
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
          <textarea rows="5" cols="60"  id ="contract_details" placeholder="Contract Details Will Be Here"
          ref={&app.state.conn_ui.sql.current_contract.contract_detail_ui} value={text}/>
          </p>
          </div>
    )
}
