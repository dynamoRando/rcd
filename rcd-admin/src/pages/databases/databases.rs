use crate::{
    pages::{
        databases::{add::Create, enable_coop::EnableCoop, tables::Tables},
        home,
    },
    request::get_databases,
};
use rcd_messages::client::DatabaseSchema;

use yew::{function_component, html, use_state_eq, Callback, Html};

/// Represents viewing and configuring schema data for tables in an RCD instance
#[function_component]
pub fn Databases() -> Html {
    let databases = get_databases();
    let mut database_names: Vec<String> = Vec::new();

    for db in &databases {
        database_names.push(db.database_name.clone());
    }

    let x = use_state_eq(move || {
        let x: Vec<String> = Vec::new();
        return x;
    });

    let selected_database = use_state_eq(|| None);

    let tables = selected_database.as_ref().map(|db: &DatabaseSchema| {
        html! {
            <Tables db={db.clone()} />
        }
    });

    let reload_db_onclick = Callback::from(move |_| {
        home::databases(x.clone());
    });

    html! {
        <div>
            <div class="container">
                <div class="box">
                        <h1 class="subtitle"> {"Databases"} </h1>

                        <p>{"After loading, click on a database to view details."}</p>
                        <button type="button" class="button is-primary" id="get_databases" value="Reload databases"
                        onclick={reload_db_onclick}>
                        <span class="mdi mdi-database-refresh">{" Reload"}</span>
                        </button>
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
                                        }><span class="mdi mdi-database"></span>{" "}{db.clone()}</li></div>
                                }
                                    }).collect::<Html>()
                                }
                            </ul>
                        </div>
                </div>
            </div>
            { tables }
            < Create />
            < EnableCoop />
        </div>
    }
}
