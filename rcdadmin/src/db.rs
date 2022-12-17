use crate::rcd_ui::PageUi;
use crate::state::databases::RcdDatabases;
use crate::AppMessage;
use crate::RcdAdminApp;
use rcd_messages::client::GetDatabasesReply;
use web_sys::console;
use yew::prelude::*;
use yew::{html::Scope, Html};
use yew::{AttrValue, Context};

pub mod view_columns;
pub mod view_tables;

pub fn handle_execute_sql_db(app: &mut RcdAdminApp, db_name: String) {
    // console::log_1(&db_name.into());
    app.state.instance.databases.data.active.database_name = db_name.clone();
    console::log_1(&app.state.instance.databases.data.active.database_name.clone().into());
}

pub fn handle_get_databases(app: &mut RcdAdminApp, db_response: AttrValue) {
    console::log_1(&db_response.to_string().clone().into());
    let db_response: GetDatabasesReply = serde_json::from_str(&db_response.to_string()).unwrap();
    if db_response.authentication_result.unwrap().is_authenticated {
        app.state.instance.databases.data.databases = db_response.databases.clone();
    }
}

pub fn handle_get_tables_for_database(
    app: &RcdAdminApp,
    ctx: &Context<RcdAdminApp>
) {
    app.view_tables_for_database(ctx.link());
}

pub fn handle_get_columns_for_table(
    app: &mut RcdAdminApp,
    db_name: String,
    table_name: String,
    ctx: &Context<RcdAdminApp>,
) {
    app.state.instance.tables.data.active.database_name = db_name.clone();
    app.state.instance.tables.data.active.table_name = table_name.clone();
    app.view_columns_for_table(ctx.link());
}

pub fn view_databases(page: &PageUi, link: &Scope<RcdAdminApp>, databases: &RcdDatabases) -> Html {
    let is_visible = !page.databases_is_visible;

    let mut db_names: Vec<String> = Vec::new();

    for db in &databases.data.databases {
        db_names.push(db.database_name.clone());
    }

    html! {
       <div hidden={is_visible}>
       <h1> {"Databases"} </h1>
       <p>{"After connecting, the list of databases on the rcd instance will appear here. Click on one to view schema details."}</p>
       <ul>
       {
        db_names.into_iter().map(|name| {
            let db_name = name.clone();
            html!{<div key={db_name.clone()}>
            <li onclick={link.callback(move |_| AppMessage::GetTablesForDatabase(name.clone()))}>{db_name.clone()}</li></div>}
        }).collect::<Html>()
    }</ul>
       </div>
    }
}
