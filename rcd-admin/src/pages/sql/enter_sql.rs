use crate::{
    pages::sql::{read::read, sql::SqlProps, write::cooperative_write, write::write},
    request::get_databases,
};
use rcd_http_common::url::client::{
    COOPERATIVE_WRITE_SQL_AT_HOST, READ_SQL_AT_HOST, READ_SQL_AT_PARTICIPANT, WRITE_SQL_AT_HOST,
    WRITE_SQL_AT_PARTICIPANT,
};
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
        let state = state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
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

    let onclick_read_at_part = {
        let state = state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            if active_database.is_some() {
                let db_name = active_database.as_ref().unwrap().clone();
                let text = ui_enter_sql_text
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                read(
                    db_name.to_string(),
                    text,
                    state.clone(),
                    READ_SQL_AT_PARTICIPANT,
                )
            }
        })
    };

    let onclick_write_at_host = {
        let state = state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            if active_database.is_some() {
                let db_name = active_database.as_ref().unwrap().clone();
                let text = ui_enter_sql_text
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                write(db_name.to_string(), text, state.clone(), WRITE_SQL_AT_HOST)
            }
        })
    };

    let onclick_write_at_part = {
        let state = state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            if active_database.is_some() {
                let db_name = active_database.as_ref().unwrap().clone();
                let text = ui_enter_sql_text
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                write(
                    db_name.to_string(),
                    text,
                    state.clone(),
                    WRITE_SQL_AT_PARTICIPANT,
                )
            }
        })
    };

    let onclick_coop_write_at_host = {
        let state = state.clone();
        let ui_enter_sql_text = ui_enter_sql_text.clone();
        let active_database = active_database.clone();
        Callback::from(move |_| {
            let active_database = active_database.clone();
            if active_database.is_some() {
                let alias = active_participant.as_ref().unwrap().clone();

                let db_name = active_database.as_ref().unwrap().clone();
                let text = ui_enter_sql_text
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                cooperative_write(
                    db_name.to_string(),
                    text,
                    alias,
                    state.clone(),
                    COOPERATIVE_WRITE_SQL_AT_HOST,
                )
            }
        })
    };

    html! {
        <div>
            <h1 class="subtitle"> {"Execute SQL"} </h1>
                <p>
                    <p><label for="execute_sql_dbs">{ "Select Database " }</label></p>
                    <p>
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
                </p>
            <p><label for="execute_sql">{ "Enter SQL" }</label></p>
            <p>
                <textarea class="textarea" rows="5" cols="60"  id ="execute_sql" placeholder="SELECT * FROM TABLE_NAME" ref={&ui_enter_sql_text}/>
            </p>

            <p><h3> {"Choose Participant"} </h3></p>
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
            <div class="buttons">
                <button class="button is-primary" type="button" id="read_at_host" value="Read On Host Db" onclick={&onclick_read_at_host}>
                    <span class="mdi mdi-database">{" Read At Host"}</span>
                </button>
                <button class="button is-primary" type="button" id="read_at_part" value="Read On Partial Db" onclick={&onclick_read_at_part}>
                    <span class="mdi mdi-database-outline">{" Read At Partial"}</span>
                </button>
                <button class="button is-warning" type="button" id="write_at_host" value="Write On Host Db" onclick={&onclick_write_at_host}>
                    <span class="mdi mdi-database">{" Write At Host"}</span>
                </button>
                <button class="button is-warning" type="button" id="write_at_part" value="Write On Part Db" onclick={&onclick_write_at_part}>
                    <span class="mdi mdi-database-outline">{" Write At Partial"}</span>
                </button>
                <button class="button is-warning" type="button" id="coop_write_at_part" value="Cooperative Write On Host Db" onclick={&onclick_coop_write_at_host}>
                    <span class="mdi mdi-database-export"></span><span class="mdi mdi-database-import-outline"></span>{" Cooperative Write At Host"}
                </button>
            </div>
        </div>
    }
}
