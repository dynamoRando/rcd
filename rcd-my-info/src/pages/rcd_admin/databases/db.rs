use rcd_messages::{
    client::{AuthRequest, DatabaseSchema, GetDatabasesReply, GetDatabasesRequest},
    proxy::request_type::RequestType,
};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_state, use_state_eq, Callback, Html, UseStateHandle};
use yew_router::prelude::use_navigator;


use crate::{
    app::Route,
    log::log_to_console,
    pages::{
        login,
        rcd_admin::databases::{
            add::Create,
            enable_coop::EnableCoop,
            tables::{self, Tables},
        },
    },
    request::{
        proxy::{get_proxy, get_proxy_token, has_proxy_token},
        rcd::{
            get_database, get_databases, get_rcd_token, set_databases, set_status,
            update_token_login_status,
        },
    },
};

#[function_component]
pub fn RcdDb() -> Html {
    let databases = get_databases();
    let mut database_names: Vec<String> = Vec::new();

    let reload_db_onclick = Callback::from(move |_| {
        spawn_local(async move {
            login::databases().await;
        })
    });

    for db in &databases {
        database_names.push(db.database_name.clone());
    }

    let selected_database = use_state_eq(|| None);

    let tables = selected_database.as_ref().map(|db: &DatabaseSchema| {
        html! {
            <Tables db={db.clone()} />
        }
    });

    let selected_database = use_state_eq(|| None);

    let navigator = use_navigator().unwrap();
    if !has_proxy_token() {
        navigator.push(&Route::Login);
        html! {
            <div>
                <p>{"You are not logged in, redirecting to login page."}</p>
            </div>
        }
    } else {
        let database_names = database_names.clone();
        html! {
            <div>
            <div class="container">
                <div class="box">
                        <h1 class="subtitle"> {"Databases"} </h1>
                        <p>{"View database schema information and configure properties of schema objects from this page."}</p>
                        <p>{"After loading, click on a database to view details."}</p>
                        <button type="button" class="button is-primary" id="get_databases" value="Reload databases"
                        onclick={reload_db_onclick}>
                        <span class="mdi mdi-database-refresh">{" Reload"}</span>
                        </button>
                        <h2 class="subtitle">{"Icon Key"}</h2>
                        <div class="table-container">
                            <table class="table is-narrow">
                                <thead>
                                    <tr>
                                        <th>{"Icon"}</th>
                                        <th>{"Value"}</th>
                                    </tr>
                                </thead>
                                <tr>
                                    <td><span class="mdi mdi-database"></span></td>
                                    <td>{"Database"}</td>
                                </tr>
                                <tr>
                                    <td><span class="mdi mdi-handshake"></span></td>
                                    <td>{"Cooperation Is Enabled"}</td>
                                </tr>
                                <tr>
                                    <td><span class="mdi mdi-account-multiple"></span></td>
                                    <td>{"Has Participants"}</td>
                                </tr>
                            </table>
                        </div>
                        <div class="content">
                            <ul>
                                {
                                    database_names.clone().into_iter().map(|name| {

                                    let db_name = name.clone();
                                    let db = db_name.clone();

                                    let database = get_database(&db_name);

                                    let cooperation_enabled = database.cooperation_enabled;
                                    let has_participants = database.has_participants;

                                    html!{
                                    <div key={db_name.clone()}>
                                        <li onclick={
                                            let selected_database = selected_database.clone();

                                            move |_| {
                                                    let database = get_database(&db_name);
                                                    selected_database.set(Some(database));
                                                }
                                            }>
                                            <span class="mdi mdi-database"></span>
                                            {
                                                if cooperation_enabled {
                                                    html!{
                                                        <span class="mdi mdi-handshake"></span>
                                                    }
                                                } else {
                                                    html!{
                                                        <a></a>
                                                    }
                                                }
                                            }
                                            {
                                                if has_participants {
                                                    html!{
                                                        <span class="mdi mdi-account-multiple"></span>
                                                    }
                                                } else {
                                                    html!{
                                                        <a></a>
                                                    }
                                                }
                                            }
                                            {" "}
                                            {db.clone()}
                                        </li>
                                    </div>
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
}
