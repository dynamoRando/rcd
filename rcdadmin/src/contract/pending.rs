use yew::prelude::*;
use yew::{html::Scope, Html};
use crate::{AppMessage, RcdAdminApp, ContractIntent};

pub fn view(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let text = app.state.instance.contract.pending.data.markdown.clone();
    html!(
        <div>
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
          ref={&app.state.instance.contract.pending.ui.details} value={text}/>
          </p>
          </div>
    )
}