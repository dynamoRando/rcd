use crate::{ExecuteSQLIntent, RcdAdminApp, request, AppMessage};
use rcd_messages::client::{AuthRequest, ExecuteReadRequest};
use web_sys::{console, HtmlInputElement};
use yew::Context;

pub fn handle_execute_sql(
    app: &mut RcdAdminApp,
    ctx: &Context<RcdAdminApp>,
    intent: ExecuteSQLIntent,
) {
    match intent {
        ExecuteSQLIntent::Unknown => todo!(),
        ExecuteSQLIntent::ReadAtHost => {
            let base_address = app.state.conn_ui.conn.url.clone();
            let url = format!("{}{}", base_address.clone(), "/client/sql/host/read/");
            let auth_json = &app.state.conn_ui.conn.auth_request_json;
            let auth: AuthRequest = serde_json::from_str(&auth_json).unwrap();
            let db_name = &app.state.conn_ui.sql.selected_db_name;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let sql_text_node = &app.state.conn_ui.sql.execute_sql;
            let sql_text = sql_text_node.cast::<HtmlInputElement>().unwrap().value();

            console::log_1(&"sql_text".into());
            console::log_1(&sql_text.clone().into());

            let read_request = ExecuteReadRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                sql_statement: sql_text,
                database_type: 1,
            };

            let read_request_json = serde_json::to_string(&read_request).unwrap();

            let sql_callback = ctx.link().callback(AppMessage::SQLResult);

            request::get_data(url, read_request_json, sql_callback);
        }
        ExecuteSQLIntent::ReadAtPart => todo!(),
        ExecuteSQLIntent::WriteAtHost => todo!(),
        ExecuteSQLIntent::WriteAtPart => todo!(),
    }
}
