use web_sys::{HtmlSelectElement, console};
use yew::prelude::*;
use yew::{html::Scope, Html};
use crate::{AppMessage, RcdAdminApp, ContractIntent};


pub fn view(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {

    let databases = &app.databases.data.databases;

    let mut db_names: Vec<String> = Vec::new();

    for db in databases {
        db_names.push(db.database_name.clone());
    }

    let active_contract = app.contract.data.active.markdown.clone();

    html!(
        <div>
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
          ref={&app.contract.ui.details} value={active_contract}/>
          </p>
        </div>
    )
}