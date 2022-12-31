use crate::{
    pages::sql::{read::read, sql::SqlProps},
    request::get_databases,
};
use rcd_http_common::url::client::READ_SQL_AT_HOST;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, Callback, Html};

#[function_component]
pub fn EnterSql(SqlProps { state }: &SqlProps) -> Html {
    let state = state.clone();
    let databases = get_databases();

    let mut database_names: Vec<String> = Vec::new();
    let participant_aliases: Vec<String> = Vec::new();

    for database in &databases {
        database_names.push(database.database_name.clone());
    }

    let active_database = use_state_eq(move || None);
    let active_participant = use_state_eq(move || None);

    // drop-down
    let ui_active_database = use_node_ref();

    // text-box
    let ui_enter_sql_text = use_node_ref();

    // drop-down
    let ui_active_participant = use_node_ref();

    let onchange_db = {
        let active_database = active_database.clone();
        let ui_active_database = ui_active_database.clone();

        Callback::from(move |_| {
            let active_database = active_database.clone();
            let ui_active_database = ui_active_database.clone();

            let selected_db = ui_active_database.cast::<HtmlInputElement>();

            if selected_db.is_some() {
                let selected_db_val = ui_active_database
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                active_database.set(Some(selected_db_val));
            }
        })
    };

    let onchange_participant = {
        let active_participant = active_participant.clone();
        let ui_active_participant = ui_active_participant.clone();

        Callback::from(move |_| {
            let selected_particpant = ui_active_participant.cast::<HtmlInputElement>();
            let ui_active_participant = ui_active_participant.clone();

            if selected_particpant.is_some() {
                let selected_participant_val = ui_active_participant
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                active_participant.set(Some(selected_participant_val));
            }
        })
    };

    let onclick_read_at_host = {
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            if active_database.is_some() {
                let db_name = active_database.as_ref().unwrap().clone();
                let text = ui_enter_sql_text
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                read(db_name.to_string(), text, state.clone(), READ_SQL_AT_HOST)
            }
        })
    };

    html! {
        <div>
            <h1 class="subtitle is-5"> {"Execute SQL"} </h1>

            <div class="section">
                <label for="execute_sql">{ "Enter SQL" }</label>
                <p>
                    <label for="execute_sql_dbs">{ "Select Database " }</label>

                    <div class="select is-multiple">
                        <select
                            name="execute_sql_dbs"
                            id="execute_sql_dbs"
                            ref={&ui_active_database}
                            onchange={onchange_db}
                        >

                        <option value="SELECT DATABASE">{"SELECT DATABASE"}</option>
                        {
                            database_names.into_iter().map(|name| {
                                // console::log_1(&name.clone().into());
                                html!{
                                <option value={name.clone()}>{name.clone()}</option>}
                            }).collect::<Html>()
                        }
                        </select>
                    </div>
                </p>

            <p>
                <textarea class="textarea" rows="5" cols="60"  id ="execute_sql" placeholder="SELECT * FROM TABLE_NAME" ref={&ui_enter_sql_text}/>
            </p>

            <h3> {"Choose Participant"} </h3>
            <p>{"Select the participant to execute on, if applicable."}</p>
            <p>
                <label for="select_participant_for_execute">{ "Select Participant " }</label>

                <div class="select is-multiple">
                <select
                    name="select_participant_for_execute"
                    id="select_participant_for_execute"
                    ref={&ui_active_participant}
                    onchange={&onchange_participant}
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
                </div>

            <p>{"The following commands denote if you wish to execute your SQL action (read or write) against the specified type of database (host or partial). To write data to a participant, use Cooperative Write."}</p>
            </p>
            <input class="button" type="button" id="read_at_host" value="Execute Read On Host Db" onclick={&onclick_read_at_host}/>
                // <input class="button" type="button" id="read_at_part" value="Execute Read On Partial Db" onclick={link.callback(|_|
                // {
                //     AppMessage::Sql_HttpRequest(ExecuteSQLIntent::ReadAtPart)
                // })}/>
                // <input  class="button"  type="button" id="write_at_host" value="Execute Write On Host Db" onclick={link.callback(|_|
                // {
                //     AppMessage::Sql_HttpRequest(ExecuteSQLIntent::WriteAtHost)
                // })}/>
                // <input class="button" type="button" id="write_at_part" value="Execute Write On Part Db" onclick={link.callback(|_|
                // {
                //     AppMessage::Sql_HttpRequest(ExecuteSQLIntent::WriteAtPart)
                // })}/>
                // <input class="button"  type="button" id="coop_write_at_part" value="Execute Coop Write On Host Db" onclick={link.callback(|_|
                //     {
                //         AppMessage::Sql_HttpRequest(ExecuteSQLIntent::CoopWriteAtHost)
                //     })}/>
            </div>
        </div>
    }
}
