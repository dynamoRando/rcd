use crate::{
    log::log_to_console,
    request::{self, get_databases, get_token},
};
use rcd_http_common::url::client::{GET_POLICY, SET_POLICY};
use rcd_messages::client::{
    DatabaseSchema, GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest,
    SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest, TableSchema,
};
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{
    function_component, html, use_node_ref, use_state, use_state_eq, AttrValue, Callback, Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub db: DatabaseSchema,
}

#[derive(Properties, PartialEq)]
pub struct ColumnProps {
    pub table: TableSchema,
}

#[function_component]
pub fn Databases() -> Html {
    let databases = get_databases();
    let mut database_names: Vec<String> = Vec::new();

    for db in &databases {
        database_names.push(db.database_name.clone());
    }

    let selected_database = use_state_eq(|| None);

    let tables = selected_database.as_ref().map(|db: &DatabaseSchema| {
        html! {
            <Tables db={db.clone()} />
        }
    });

    html! {
        <div>
            <div class="container">
                <div class="box">
                        <h1 class="subtitle"> {"Databases"} </h1>

                        <p>{"After loading, click on a database to view details."}</p>
                        <div class="content">
                            <ul>
                                {
                                    database_names.clone().into_iter().map(|name| {

                                    let db_name = name.clone();
                                    let db = db_name.clone();

                                    html!{<div key={db_name.clone()}>
                                    <li onclick={

                                        let selected_database = selected_database.clone();

                                        move |_| {
                                                let databases = get_databases();

                                                let database = databases
                                                    .iter()
                                                    .find(|x| x.database_name.as_str() == db_name)
                                                    .unwrap()
                                                    .clone();

                                                selected_database.set(Some(database));
                                            }
                                        }>{db.clone()}</li></div>
                                }
                                    }).collect::<Html>()
                                }
                            </ul>
                        </div>
                </div>
            </div>
            { tables }
        </div>
    }
}

#[function_component]
pub fn Tables(TableProps { db }: &TableProps) -> Html {
    let message = format!("{}{}", "entered tables for: ", db.database_name);
    log_to_console(message);

    let db = db.clone();
    let tables = db.tables.clone();

    let mut table_names: Vec<String> = Vec::new();

    for table in &*db.tables {
        table_names.push(table.table_name.clone());
    }

    let selected_table = use_state_eq(|| None);

    let get_policy = selected_table.as_ref().map(|table: &TableSchema| {
        html! {
            <GetTablePolicy table={table.clone()} />
        }
    });

    let set_policy = selected_table.as_ref().map(|table: &TableSchema| {
        html! {
            <SetTablePolicy table={table.clone()} />
        }
    });

    let columns = selected_table.as_ref().map(|table: &TableSchema| {
        html! {
            <Columns table={table.clone()} />
        }
    });

    html!(
        <div>
            <div class="container">
                    <div class="box">
                        <h1 class="subtitle"> {"Tables for database: "}{&db.database_name} </h1>

                        <p>{"Tables for the specified database will appear here. Click on a table name to view schema info."}</p>
                            <div class="content">
                                <ul>
                                {
                                    table_names.into_iter().map(|name| {
                                        let table_name = name.clone();
                                        html!{
                                            <div key={table_name.clone()}>
                                                <li onclick={

                                                    let selected_table = selected_table.clone();
                                                    let tables = tables.clone();
                                                    let name = name.clone();

                                                    move |_| {

                                                            let table = &tables
                                                                .iter()
                                                                .find(|x| x.table_name.as_str() == name.clone())
                                                                .unwrap()
                                                                .clone();

                                                                selected_table.set(Some(table.clone()));
                                                        }
                                                    }>{name.clone()}</li>
                                            </div>}
                                    }).collect::<Html>()
                                }
                                </ul>
                                { get_policy }
                                <p></p>
                                { set_policy }
                            </div>
                    </div>
            </div>
            { columns }
        </div>
    )
}

#[function_component]
pub fn GetTablePolicy(ColumnProps { table }: &ColumnProps) -> Html {
    let message = format!("{}{}", "entered table policy for: ", table.table_name);
    log_to_console(message);

    let database_name = table.database_name.clone();
    let table_name = table.table_name.clone();

    let table_policy = use_state_eq(|| None);
    let policy = table_policy.clone();

    let get_policy_response_cb = Callback::from(move |response: AttrValue| {
        log_to_console(response.to_string());

        let table_policy = table_policy.clone();

        let reply: GetLogicalStoragePolicyReply =
            serde_json::from_str(&&response.to_string()).unwrap();

        if reply.authentication_result.unwrap().is_authenticated {
            let policy_value = reply.policy_mode;

            let policy_name = match policy_value {
                0 => "None",
                1 => "Host Only",
                2 => "Participant Owned",
                3 => "Shared",
                4 => "Mirror",
                _ => "Unknown",
            };

            table_policy.set(Some(policy_name));
        }
    });

    let token = get_token();

    let request = GetLogicalStoragePolicyRequest {
        authentication: Some(token.auth()),
        database_name: database_name,
        table_name: table_name.clone(),
    };

    let request_json = serde_json::to_string(&request).unwrap();
    let url = format!("{}{}", token.addr, GET_POLICY);
    request::get_data(url, request_json, get_policy_response_cb);

    html!(
        <div class="container">
            <h2 class="subtitle">{"Table Policy"}</h2>
            <h3 class="subtitle">{"Get Policy"}</h3>
            <p>
                <p>{"Click on the table name FIRST before viewing/updating table policy."}</p>
                <p><label for="selected_table_policy">{"Current Policy For Table: "}{table_name}</label></p>
                <p><input class="input" type="text" id ="selected_table_policy" placeholder="Logical Storage Policy" value={*(policy)} readonly=true/></p>
            </p>
        </div>
    )
}

#[function_component]
pub fn SetTablePolicy(ColumnProps { table }: &ColumnProps) -> Html {
    let set_policy_result = use_state_eq(|| None);
    let set_new_policy = use_state_eq(|| None);

    let ui_new_policy = use_node_ref();

    let database_name = table.database_name.clone();
    let table_name = table.table_name.clone();

    let onchange = {
        let set_new_policy = set_new_policy.clone();
        let ui_new_policy = ui_new_policy.clone();

        Callback::from(move |_| {
            log_to_console("SetTablePolicy - Onchange".to_string());
            let policy = ui_new_policy.cast::<HtmlInputElement>();

            if policy.is_some() {
                let policy_val = ui_new_policy.cast::<HtmlInputElement>().unwrap().value();
                set_new_policy.set(Some(policy_val));
            }
        })
    };

    let onclick: Callback<MouseEvent> = {
        log_to_console("SetTablePolicy - Clicked".to_string());
        let set_new_policy = set_new_policy.clone();

        if set_new_policy.is_some() {
            let policy_val = set_new_policy.as_ref().unwrap();

            let message = format!("{}{}", "SetTablePolicy Value: ", policy_val.clone());
            log_to_console(message);

            let policy_num: u32 = policy_val.parse().unwrap();

            let database_name = database_name.clone();
            let table_name = table_name.clone();

            let set_policy_result = set_policy_result.clone();

            Callback::from(move |_| {
                let token = get_token();
                let set_policy_result = set_policy_result.clone();

                let cb = Callback::from(move |response: AttrValue| {
                    log_to_console(response.to_string());

                    let reply: SetLogicalStoragePolicyReply =
                        serde_json::from_str(&&response.to_string()).unwrap();

                    if reply.authentication_result.unwrap().is_authenticated {
                        set_policy_result.set(Some(reply.is_successful));
                    }
                });

                let database_name = database_name.clone();
                let table_name = table_name.clone();

                let request = SetLogicalStoragePolicyRequest {
                    authentication: Some(token.auth()),
                    database_name: database_name,
                    table_name: table_name,
                    policy_mode: policy_num,
                };

                let request_json = serde_json::to_string(&request).unwrap();
                let url = format!("{}{}", token.addr, SET_POLICY);

                let message = format!("{}{}", "SENDING REQUEST FOR NEW POLICY: ", request_json);
                log_to_console(message);

                request::get_data(url, request_json, cb);
            })
        } else {
            Callback::from(move |_| {})
        }
    };

    html!(
        <div class="container">
            <p>
                <h3 class="subtitle">{"Set Policy"}</h3>
                <p>{"Click on the table name FIRST before setting new policy."}</p>
                <label for="table_policy_value">{ "Set Policy For Table: " }{&table_name}</label>
                <p>
                    <div class="select is-multiple">
                        <select name="set_policy_for_table" id="set_policy_for_table" ref={&ui_new_policy} {onchange}>
                            /*
                                None = 0,
                                HostOnly = 1,
                                ParticpantOwned = 2,
                                Shared = 3,
                                Mirror = 4,
                            */
                            <option value={"0"}>{"None"}</option>
                            <option value={"1"}>{"Host Only"}</option>
                            <option value={"2"}>{"Participant Owned"}</option>
                            <option value={"3"}>{"Shared"}</option>
                            <option value={"4"}>{"Mirror"}</option>
                        </select>
                    </div>
                </p>
                <p><input class="button is-primary" type="button" id="submit_new_table_policy" value="Update Policy" {onclick}/></p>
                <p>{"Last result for table: "}{&table_name}{" was: "}{*(set_policy_result)}</p>
            </p>
        </div>
    )
}

#[function_component]
pub fn Columns(ColumnProps { table }: &ColumnProps) -> Html {
    let table_name = &table.table_name;
    let db_name = &table.database_name;

    let mut col_names: Vec<String> = Vec::new();

    for col in &table.columns {
        col_names.push(col.column_name.clone());
    }

    html! {
            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Columns for table "}{&table_name} {" in database "}{&db_name}</h1>
                        <div class="content">
                            <ul>
                            {
                                col_names.into_iter().map(|name| {
                                    let col_name = name.clone();
                                    html!{<div key={col_name.clone()}>
                                    <li>{col_name.clone()}</li></div>}
                                }).collect::<Html>()
                            }</ul>
                        </div>
                </div>
           </div>
    }
}
