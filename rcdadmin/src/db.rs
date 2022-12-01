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
    app.state.conn_ui.sql.selected_db_name = db_name.clone();
    console::log_1(&app.state.conn_ui.sql.selected_db_name.clone().into());
}

pub fn handle_get_databases(app: &mut RcdAdminApp, db_response: AttrValue) {
    console::log_1(&db_response.to_string().clone().into());
    let db_response: GetDatabasesReply = serde_json::from_str(&db_response.to_string()).unwrap();
    if db_response.authentication_result.unwrap().is_authenticated {
        app.state.conn_ui.conn.databases = db_response.databases.clone();
    }
}

pub fn handle_get_tables_for_database(
    app: &mut RcdAdminApp,
    db_name: String,
    ctx: &Context<RcdAdminApp>,
) {
    app.state.conn_ui.conn.current_db_name = db_name;
    app.view_tables_for_database(ctx.link());
}

pub fn handle_get_columns_for_table(
    app: &mut RcdAdminApp,
    db_name: String,
    table_name: String,
    ctx: &Context<RcdAdminApp>,
) {
    app.state.conn_ui.conn.current_db_name = db_name;
    app.state.conn_ui.conn.current_table_name = table_name;
    app.view_columns_for_table(ctx.link());
}

pub fn view_databases(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    
    let is_visible = !app.state.page_ui.databases_is_visible;

    let mut db_names: Vec<String> = Vec::new();

    for db in &app.state.conn_ui.conn.databases {
        db_names.push(db.database_name.clone());
    }

    html! {
       <div hidden={is_visible}>
       <h1> {"Databases"} </h1>
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
