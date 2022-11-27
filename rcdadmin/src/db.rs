use crate::RcdAdminApp;
use rcd_messages::client::GetDatabasesReply;
use web_sys::console;
use yew::{AttrValue, Context};

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