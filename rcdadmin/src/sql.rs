use crate::{request, AppMessage, ExecuteSQLIntent, RcdAdminApp};
use rcd_messages::{
    client::{AuthRequest, ExecuteReadReply, ExecuteReadRequest},
    formatter,
};
use web_sys::{console, HtmlInputElement};
use yew::{AttrValue, Context};

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

pub fn handle_sql_result(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    console::log_1(&json_response.to_string().clone().into());
    let read_reply: ExecuteReadReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if read_reply.authentication_result.unwrap().is_authenticated {
        let rows = read_reply.results.first().unwrap().rows.clone();

        let sql_table_text = formatter::rows_to_string_markdown_table(&rows);
        app.state.conn_ui.sql_text_result = sql_table_text;
    }
}
