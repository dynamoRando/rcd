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
        <h1> {"Configure UI"} </h1>
       <p>
        <label for="show_conn_options">{ "Hide Connection Options" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(conn_ui)
            })}
        />
        </p><p>
        <label for="show_db">{ "Hide Databases" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(db_ui)
            })}
        /></p><p>
        <label for="show_sql">{ "Hide Sql" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(sql_ui)
            })}
        /></p><p>
        <label for="show_contract">{ "Hide Contracts" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(contracts_ui)
            })}
        /></p><p>
        <label for="show_hosts">{ "Hide Hosts" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(hosts_ui)
            })}
        /></p><p>
        <label for="participants">{ "Hide Participants" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(participants_ui)
            })}
        /></p><p>
        <label for="show_behaviors">{ "Hide Behaviors" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(behaviors_ui)
            })}
        /></p><p>
        <label for="show_contract">{ "Hide Cooperative Hosts" }</label>
        <input
        type="checkbox"
        onclick={link.callback(move |_|
            {
                AppMessage::HandleToggleVisiblity(coop_hosts_ui)
            })}
        /></p>
        </div>
    }
}
