use crate::{request, AppMessage, ExecuteSQLIntent, RcdAdminApp};
use rcd_messages::{
    client::{AuthRequest, ExecuteReadReply, ExecuteReadRequest},
    formatter,
};
use web_sys::{console, Event, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::{html::Scope, AttrValue, Context, Html};

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

pub fn view_input_for_sql(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let mut db_names: Vec<String> = Vec::new();

    for db in &app.state.conn_ui.conn.databases {
        db_names.push(db.database_name.clone());
    }

    // console::log_1(&"view_input_for_sql".into());
    // console::log_1(&db_names.len().to_string().into());

    html! {
        <div>
        <h1> {"Execute SQL"} </h1>
        <label for="execute_sql">{ "Enter SQL" }</label>
        <p>
        <label for="execute_sql_dbs">{ "Select Database " }</label>
        <select name="execute_sql_dbs" id="execute_sql_dbs"

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
            db_names.into_iter().map(|name| {
                // console::log_1(&name.clone().into());
                html!{
                <option value={name.clone()}>{name.clone()}</option>}
            }).collect::<Html>()
        }
        </select>
        </p>
        <p>
        <textarea rows="5" cols="60"  id ="execute_sql" placeholder="SELECT * FROM TABLE_NAME" ref={&app.state.conn_ui.sql.execute_sql}/>
        </p>
        <input type="button" id="read_at_host" value="Execute Read At Host" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::ReadAtHost)
            })}/>
            <input type="button" id="read_at_part" value="Execute Read At Part" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::ReadAtPart)
            })}/>
            <input type="button" id="write_at_host" value="Execute Write At Host" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::WriteAtHost)
            })}/>
            <input type="button" id="write_at_part" value="Execute Write At Part" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::WriteAtPart)
            })}/>
        </div>
    }
}


pub fn view_sql_result(app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    let text = app.state.conn_ui.sql_text_result.clone();

    html!(
      <div>
          <h1> {"SQL Results"} </h1>
          <label for="sql_result">{ "Results" }</label>
          <p>
          <textarea rows="5" cols="60"  id ="sql_Result" placeholder="SQL Results Will Be Displayed Here As Markdown Table"
          ref={&app.state.conn_ui.sql.sql_result} value={text}/>
          </p>
          </div>
    )
}
