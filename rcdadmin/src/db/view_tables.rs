use web_sys::console;
use yew::html;
use yew::{html::Scope, Html};

use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::rcd_ui::PageUi;
use crate::state::databases::RcdDatabases;
use crate::state::tables::RcdTables;
use crate::{AppMessage, RcdAdminApp, TableIntent};

pub fn view_tables_for_database(
    page: &PageUi,
    link: &Scope<RcdAdminApp>,
    databases: &RcdDatabases,
    tables_ui: &RcdTables,
) -> Html {
    let is_visible = !page.databases_is_visible;

    let db_name = databases.data.active.database_name.clone();

    let current_table_policy = tables_ui.data.active.policy_name.clone();

    if db_name == "" {
        html! {
            <div/>
        }
    } else {
        let tables = databases
            .data
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
                <div class="container">
                    <div class="box">
                        <h1 class="subtitle"> {"Tables for database "}{&db_name}</h1>

                        <p>{"Tables for the specified database will appear here. Click on a table name to view schema info."}</p>
                            <div class="content">
                                <ul>
                                {
                                    table_names.into_iter().map(|name| {
                                        let table_name = name.clone();
                                        let d_name = db_name.clone();
                                        html!{
                                            <div key={table_name.clone()}>
                                                <li onclick={link.callback(move |_| AppMessage::Db_SetAndView_Columns(d_name.clone(), table_name.clone()))}>{name.clone()}</li>
                                            </div>}
                                    }).collect::<Html>()
                                }
                                </ul>
                            </div>
                        <p>
                            <h2 class="subtitle">{"Table Policies"}</h2>
                                <label for="table_policy">{ "Table Name" }</label>
                                <div class="select is-multiple">
                                    <select name="select_table_for_policy" id="table_policy" onchange={link.batch_callback(move |e: Event| {
                                        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                                            let table_name = input.value();
                                            let d_name = db_name.clone();
                                            Some(AppMessage::Policy_HttpRequest(TableIntent::GetTablePolicy((d_name.clone(), table_name))))
                                        } else {
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
                                </div>

                                <p>
                                    <label for="selected_table_policy">{"Current Policy:"}</label>
                                    <input class="input" type="text" id ="selected_table_policy" placeholder="Logical Storage Policy" ref={&tables_ui.ui.current_policy}
                                    value={current_table_policy} readonly=true/>
                                </p>

                                <p>
                                    <label for="table_policy_value">{ "Set Policy" }</label>
                                    <div class="select is-multiple">
                                        <select name="set_policy_for_table" id="set_policy_for_table" ref={&tables_ui.ui.new_policy}>
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
                                    </div>
                                    <input class="button is-primary" type="button" id="submit_new_table_policy" value="Update Policy" onclick={link.callback(move |_|
                                    {
                                        console::log_1(&"submit_new_table_policy".into());
                                        AppMessage::Policy_HttpRequest(TableIntent::SetTablePolicy)
                                    })}/>
                                </p>
                        </p>
                    </div>
                </div>
           </div>
        }
    }
}
