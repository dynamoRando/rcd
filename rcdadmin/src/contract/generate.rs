use web_sys::{HtmlSelectElement, console};
use yew::prelude::*;
use yew::{html::Scope, Html};
use crate::{AppMessage, RcdAdminApp, ContractIntent};


pub fn view(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {

    let generate = &app.state.instance.contract.generate;
    let databases = &app.state.instance.databases.data.databases;

    let mut db_names: Vec<String> = Vec::new();

    for db in databases {
        db_names.push(db.database_name.clone());
    }

    let last_gen_result = generate
        .result
        .data
        .is_successful;

    html!(
        <div>
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
        ref={&generate.ui.host_name}/>
        </p>
        <label for="gen_contract_desc">{ "Description" }</label>
        <p>
        <textarea rows="5" cols="60"  id ="gen_contract_desc" placeholder="A bried description of the purpose of this database"
        ref={&generate.ui.description}/>
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
        </div>
    )
}