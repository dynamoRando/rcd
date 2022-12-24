use yew::prelude::*;
use yew::{html::Scope, Html};

use crate::{AppMessage, RcdAdminApp, UiVisibility};

#[allow(dead_code, unused_variables)]
pub fn view_ui_options(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let conn_is_visible = !app.page.conn_is_visible;
    let conn_ui = UiVisibility::Connection(conn_is_visible);

    let db_is_visible = !app.page.databases_is_visible;
    let db_ui = UiVisibility::Databases(db_is_visible);

    let sql_is_visible = !app.page.sql_is_visible;
    let sql_ui = UiVisibility::SQL(sql_is_visible);

    let contracts_is_visible = !app.page.contract_is_visible;
    let contracts_ui = UiVisibility::Contract(contracts_is_visible);

    let hosts_is_visible = !app.page.host_is_visible;
    let hosts_ui = UiVisibility::Host(hosts_is_visible);

    let participants_is_visible = !app.page.participants_is_visible;
    let participants_ui = UiVisibility::Participant(participants_is_visible);

    let behaviors_is_visible = !app.page.behaviors_is_visible;
    let behaviors_ui = UiVisibility::Behaviors(behaviors_is_visible);

    let coop_hosts_is_visible = !app.page.coop_hosts_is_visible;
    let coop_hosts_ui = UiVisibility::CoopHosts(coop_hosts_is_visible);

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Configure UI"} </h1>
            
                    <p>{"Hide any of the following:"}</p>

                    <div class="columns">

                        <div class="column">
                            <label class="checkbox" for="show_conn_options">
                                <input type="checkbox" id="show_conn_options" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(conn_ui)
                                    })}
                                />
                            { " Connection Options " }
                            </label>
                        </div>

                        <div class="column">
                            <label class="checkbox" for="show_db">
                                <input type="checkbox" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(db_ui)
                                    })}
                                />
                            { " Databases " }
                            </label>
                        </div>

                        <div class="column">
                            <label class="checkbox" for="show_sql">
                                <input type="checkbox" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(sql_ui)
                                    })}
                                />
                            { " Sql " }
                            </label>
                        </div>

                        <div class="column">
                            <label class="checkbox" for="show_contract">
                                <input type="checkbox" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(contracts_ui)
                                    })}
                                />
                            { " Contracts " }
                            </label>
                        </div>
                
                        <div class="column">
                            <label class="checkbox" for="show_hosts">
                                <input type="checkbox" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(hosts_ui)
                                    })}
                                />
                            { " Hosts " }
                            </label>
                        </div>
                
                        <div class="column">
                            <label class="checkbox" for="participants">
                                <input type="checkbox" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(participants_ui)
                                    })}
                                />
                            { " Participants " }
                            </label>
                        </div>
                
                        <div class="column">
                            <label class="checkbox" for="show_behaviors">
                                <input type="checkbox" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(behaviors_ui)
                                    })}
                                />
                            { " Behaviors " }
                            </label>
                        </div>
                
                        <div class="column">
                            <label class="checkbox" for="show_contract">
                                <input type="checkbox" onclick={link.callback(move |_|
                                    {
                                        AppMessage::Page_Set_Visibility(coop_hosts_ui)
                                    })}
                                />
                            { " Cooperative Hosts " }
                            </label>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
