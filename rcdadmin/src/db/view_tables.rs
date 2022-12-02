use web_sys::console;
use yew::html;
use yew::{html::Scope, Html};

use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::{AppMessage, RcdAdminApp, TableIntent};

pub fn view_tables_for_database(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.state.page_ui.databases_is_visible;
    let db_name = app.state.conn_ui.conn.current_db_name.clone();
    let current_table_policy = app.state.conn_ui.sql.current_policy.policy_text.clone();

    if db_name == "" {
        html! {
            <div/>
        }
    } else {
        let tables = app
            .state
            .conn_ui
            .conn
            .databases
            .iter()
            .find(|x| x.database_name.as_str() == db_name)
            .unwrap()
            .tables
            .clone();

        let mut table_names: Vec<String> = Vec::new();
        for table in &tables {
            table_names.push(table.table_name.clone());
        }

        let table_names_clone = table_names.clone();

        html! {
           <div hidden={is_visible}>
           <h1> {"Tables for database "}{&db_name}</h1>
           <p>{"Tables for the specified database will appear here. Click on a table name to view schema info."}</p>
           <ul>
           {
            table_names.into_iter().map(|name| {
                let table_name = name.clone();
                let d_name = db_name.clone();
                html!{<div key={table_name.clone()}>
                <li onclick={link.callback(move |_| AppMessage::GetColumnsForTable(d_name.clone(), table_name.clone()))}>{name.clone()}</li></div>}
            }).collect::<Html>()
        }</ul>
        <p>
        <h2>{"Table Policies"}</h2>
        <label for="table_policy">{ "Table Name" }</label>
        <select name="select_table_for_policy" id="table_policy"

        onchange={link.batch_callback(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                // console::log_1(&"some onchange".into());
                let table_name = input.value();
                let d_name = db_name.clone();
                Some(AppMessage::HandleTablePolicy(TableIntent::GetTablePolicy((d_name.clone(), table_name))))
            } else {
                // console::log_1(&"none onchange".into());
                None
            }
        })}
        >
        <option value="SELECT TABLE">{"SELECT TABLE"}</option>
        {
            table_names_clone.into_iter().map(|name| {
                // console::log_1(&name.clone().into());
                html!{
                <option value={name.clone()}>{name.clone()}</option>}
            }).collect::<Html>()
        }
        </select>
        <label for="selected_table_policy">{"Current Policy:"}</label>
        <input type="text" id ="selected_table_policy" placeholder="Logical Storage Policy" ref={&app.state.conn_ui.sql.current_policy.policy_node}
         value={current_table_policy}/>
        <label for="table_policy_value">{ "Set Policy" }</label>
        <select name="set_policy_for_table" id="set_policy_for_table" ref={&app.state.conn_ui.sql.current_policy.new_policy}>
        /*
            None = 0,
            HostOnly = 1,
            ParticpantOwned = 2,
            Shared = 3,
            Mirror = 4,
         */
        <option value={"0"}>{"None"}</option>
        <option value={"1"}>{"Host Only"}</option>
        <option value={"2"}>{"Participant Owned"}</option>
        <option value={"3"}>{"Shared"}</option>
        <option value={"4"}>{"Mirror"}</option>
        </select>
        <input type="button" id="submit_new_table_policy" value="Update Policy" onclick={link.callback(move |_|
            {
                console::log_1(&"submit_new_table_policy".into());

                AppMessage::HandleTablePolicy(TableIntent::SetTablePolicy)
            })}/>
        </p>
           </div>
        }
    }
}
