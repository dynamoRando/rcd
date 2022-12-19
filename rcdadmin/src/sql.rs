use crate::{
    get_auth_request, get_base_address,
    rcd_ui::PageUi,
    request,
    state::{databases::RcdDatabases, participant::RcdParticipants, sql::RcdSql},
    AppMessage, ExecuteSQLIntent, RcdAdminApp,
};
use rcd_http_common::url::client::{
    COOPERATIVE_WRITE_SQL_AT_HOST, READ_SQL_AT_HOST, READ_SQL_AT_PARTICIPANT, WRITE_SQL_AT_HOST,
    WRITE_SQL_AT_PARTICIPANT,
};
use rcd_messages::{
    client::{
        ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
        ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest,
    },
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
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), READ_SQL_AT_HOST);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let sql_text_node = &app.sql.ui.execute_sql;
            let sql_text = sql_text_node.cast::<HtmlInputElement>().unwrap().value();

            console::log_1(&"sql_text".into());
            console::log_1(&sql_text.clone().into());

            let request = ExecuteReadRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                sql_statement: sql_text,
                database_type: 1,
            };

            let read_request_json = serde_json::to_string(&request).unwrap();

            let sql_callback = ctx.link().callback(AppMessage::SQLReadResult);

            request::get_data(url, read_request_json, sql_callback);
        }
        ExecuteSQLIntent::ReadAtPart => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), READ_SQL_AT_PARTICIPANT);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;
            let participant_alias = &app.participants.data.active.alias;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());
            console::log_1(&participant_alias.into());

            let sql_text_node = &app.sql.ui.execute_sql;
            let sql_text = sql_text_node.cast::<HtmlInputElement>().unwrap().value();

            console::log_1(&"sql_text".into());
            console::log_1(&sql_text.clone().into());

            let request = ExecuteReadRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                sql_statement: sql_text.clone(),
                database_type: 1,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let sql_callback = ctx.link().callback(AppMessage::SQLReadResult);

            request::get_data(url, request_json, sql_callback);

            todo!()
        }
        ExecuteSQLIntent::WriteAtHost => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), WRITE_SQL_AT_HOST);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let sql_text_node = &app.sql.ui.execute_sql;
            let sql_text = sql_text_node.cast::<HtmlInputElement>().unwrap().value();

            console::log_1(&"sql_text".into());
            console::log_1(&sql_text.clone().into());

            let request = ExecuteWriteRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                sql_statement: sql_text,
                database_type: 1,
                where_clause: "".to_string(),
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let sql_callback = ctx.link().callback(AppMessage::SQLWriteResult);

            request::get_data(url, request_json, sql_callback);
        }
        ExecuteSQLIntent::WriteAtPart => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), WRITE_SQL_AT_PARTICIPANT);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let sql_text_node = &app.sql.ui.execute_sql;
            let sql_text = sql_text_node.cast::<HtmlInputElement>().unwrap().value();

            console::log_1(&"sql_text".into());
            console::log_1(&sql_text.clone().into());

            let request = ExecuteWriteRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                sql_statement: sql_text,
                database_type: 1,
                where_clause: "".to_string(),
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let sql_callback = ctx.link().callback(AppMessage::SQLWriteResult);

            request::get_data(url, request_json, sql_callback);
        }
        ExecuteSQLIntent::CoopWriteAtHost => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), COOPERATIVE_WRITE_SQL_AT_HOST);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;
            let participant_alias = &app.participants.data.active.alias;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let sql_text_node = &app.sql.ui.execute_sql;
            let sql_text = sql_text_node.cast::<HtmlInputElement>().unwrap().value();

            console::log_1(&"sql_text".into());
            console::log_1(&sql_text.clone().into());

            let request = ExecuteCooperativeWriteRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                sql_statement: sql_text,
                database_type: 1,
                where_clause: "".to_string(),
                alias: participant_alias.to_string(),
                participant_id: "".to_string(),
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let sql_callback = ctx.link().callback(AppMessage::SQLCooperativeWriteResult);

            request::get_data(url, request_json, sql_callback);
        }
    }
}

pub fn handle_sql_read_result(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    console::log_1(&json_response.to_string().clone().into());
    let read_reply: ExecuteReadReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if read_reply.authentication_result.unwrap().is_authenticated {
        let result = read_reply.results.first().unwrap();
        if !result.is_error {
            let rows = result.clone().rows;
            let sql_table_text = formatter::rows_to_string_markdown_table(&rows);
            app.sql.result.data.text = sql_table_text;
        } else {
            let mut message = String::new();
            message = message + &"ERROR: ";
            message = message + &result.execution_error_message.clone();
            app.sql.result.data.text = message;
        }
    }
}

pub fn handle_cooperative_write_result(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    console::log_1(&json_response.to_string().clone().into());
    let write_reply: ExecuteCooperativeWriteReply =
        serde_json::from_str(&&json_response.to_string()).unwrap();

    if write_reply.authentication_result.unwrap().is_authenticated {
        let mut result_message = String::new();

        result_message = result_message
            + &format!(
                "Is result successful: {}",
                write_reply.is_successful.to_string()
            );

        result_message = result_message + &"\n";
        result_message = result_message
            + &format!(
                "Total rows affected: {}",
                write_reply.total_rows_affected.to_string()
            );
        result_message = result_message + &"\n";

        // result_message =
        //     result_message + &format!("Error Message: {}", write_reply.error_message.clone());

        let sql_table_text = result_message.clone();
        app.sql.result.data.text = sql_table_text;
    }
    todo!()
}

pub fn handle_sql_write_result(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    console::log_1(&json_response.to_string().clone().into());
    let write_reply: ExecuteWriteReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if write_reply.authentication_result.unwrap().is_authenticated {
        let mut result_message = String::new();

        result_message = result_message
            + &format!(
                "Is result successful: {}",
                write_reply.is_successful.to_string()
            );

        result_message = result_message + &"\n";
        result_message = result_message
            + &format!(
                "Total rows affected: {}",
                write_reply.total_rows_affected.to_string()
            );
        result_message = result_message + &"\n";
        result_message =
            result_message + &format!("Error Message: {}", write_reply.error_message.clone());

        let sql_table_text = result_message.clone();
        app.sql.result.data.text = sql_table_text;
    }
}

pub fn view_input_for_sql(
    page: &PageUi,
    link: &Scope<RcdAdminApp>,
    databases: &RcdDatabases,
    participants: &RcdParticipants,
    sql_ui: &RcdSql,
) -> Html {
    let is_visible = !page.sql_is_visible;
    let mut db_names: Vec<String> = Vec::new();

    for db in &databases.data.databases {
        db_names.push(db.database_name.clone());
    }

    let participants = &participants.data.active.participants;

    let mut participant_aliases: Vec<String> = Vec::new();

    for p in participants {
        participant_aliases.push(p.participant.as_ref().unwrap().alias.clone());
    }

    html! {
        <div hidden={is_visible}>
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
        <textarea rows="5" cols="60"  id ="execute_sql" placeholder="SELECT * FROM TABLE_NAME" ref={&sql_ui.ui.execute_sql}/>
        </p>
        <h3> {"Choose Participant"} </h3>
        <p>{"Select the participant to execute on, if applicable."}</p>
        <p>
        <label for="select_participant_for_execute">{ "Select Participant " }</label>
        <select name="select_participant_for_execute" id="select_participant_for_execute"

        onchange={link.batch_callback(|e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                // console::log_1(&"some onchange".into());
                Some(AppMessage::SetExecuteSQLForParticipant(input.value()))
            } else {
                // console::log_1(&"none onchange".into());
                None
            }
        })}
        >
        <option value="SELECT PARTICIPANT">{"SELECT PARTICIPANT"}</option>
        {
            participant_aliases.clone().into_iter().map(|name| {
                // console::log_1(&name.clone().into());
                html!{
                <option value={name.clone()}>{name.clone()}</option>}
            }).collect::<Html>()
        }
        </select>
        <p>{"The following commands denote if you wish to execute your SQL action (read or write) against the specified type of database (host or partial). To write data to a participant, use Cooperative Write."}</p>
        </p>
        <input type="button" id="read_at_host" value="Execute Read On Host Db" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::ReadAtHost)
            })}/>
            <input type="button" id="read_at_part" value="Execute Read On Partial Db" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::ReadAtPart)
            })}/>
            <input type="button" id="write_at_host" value="Execute Write On Host Db" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::WriteAtHost)
            })}/>
            <input type="button" id="write_at_part" value="Execute Write On Part Db" onclick={link.callback(|_|
            {
                AppMessage::ExecuteSQL(ExecuteSQLIntent::WriteAtPart)
            })}/>
            <input type="button" id="coop_write_at_part" value="Execute Coop Write On Host Db" onclick={link.callback(|_|
                {
                    AppMessage::ExecuteSQL(ExecuteSQLIntent::CoopWriteAtHost)
                })}/>
        </div>
    }
}

pub fn view_sql_result(app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.page.sql_is_visible;
    let text = app.sql.result.data.text.clone();

    html!(
      <div hidden={is_visible}>
          <h1> {"SQL Results"} </h1>
          <label for="sql_result">{ "Results" }</label>
          <p>
          <textarea rows="5" cols="60"  id ="sql_Result" placeholder="SQL Results Will Be Displayed Here As Markdown Table"
          ref={&app.sql.result.ui.text} value={text}/>
          </p>
          </div>
    )
}

pub fn handle_set_sql_participant(
    participant_alias: &str,
    app: &mut RcdAdminApp,
    _link: &Context<RcdAdminApp>,
) {
    app.participants.data.active.alias = participant_alias.to_string().clone();
    console::log_1(
        &app
            .participants
            .data
            .active
            .alias
            .clone()
            .into(),
    );
}
