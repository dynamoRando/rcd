use crate::request::get_databases;
use rcd_messages::client::{DatabaseSchema, TableSchema};
use yew::{function_component, html, use_state, Html, Properties};

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

    let selected_database = use_state(|| None);

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
    let db = db.clone();
    let tables = db.tables.clone();

    let mut table_names: Vec<String> = Vec::new();

    for table in &*db.tables {
        table_names.push(table.table_name.clone());
    }

    let selected_table = use_state(|| None);

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
                            </div>
                    </div>
            </div>
            { columns }
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
