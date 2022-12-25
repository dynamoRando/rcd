use rcd_messages::client::{AuthRequest, DatabaseSchema, GetDatabasesReply, GetDatabasesRequest};
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

use crate::request;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "RCD Admin" }</h1>
                </div>
            </div>

            <div class="tile is-parent container">
                <Connect />
            </div>
        </div>
    }
}

#[function_component]
pub fn Connect() -> Html {
    let ui = ConnectUi::new();

    let database_names = use_state(|| {
        let databases: Vec<String> = Vec::new();
        return databases;
    });

    let onclick = {
        let ui = ui.clone();
        let database_names = database_names.clone();

        Callback::from(move |_| {
            let database_names = database_names.clone();

            let ui = ui.clone();
            let un = &ui.un;
            let pw = &ui.pw;
            let ip = &ui.addr;
            let port = &ui.port;

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
            let ip_val = ip.cast::<HtmlInputElement>().unwrap().value();
            let port_val = port.cast::<HtmlInputElement>().unwrap().value();

            let base_address = format!("{}{}{}{}", "http://", ip_val.to_string(), ":", port_val);

            let auth_request = AuthRequest {
                user_name: un_val.to_string(),
                pw: pw_val.to_string(),
                pw_hash: Vec::new(),
                token: Vec::new(),
            };

            let db_request = GetDatabasesRequest {
                authentication: Some(auth_request.clone()),
            };

            let db_request_json = serde_json::to_string(&db_request).unwrap();

            let db_callback = Callback::from(move |response: AttrValue| {
                console::log_1(&response.to_string().into());

                let database_names = database_names.clone();

                let db_response: GetDatabasesReply =
                    serde_json::from_str(&response.to_string()).unwrap();
                if db_response.authentication_result.unwrap().is_authenticated {
                    let databases = db_response.databases.clone();

                    let mut db_names: Vec<String> = Vec::new();

                    for db in &databases {
                        db_names.push(db.database_name.clone());
                    }

                    database_names.set(db_names);
                }
            });

            let url = format!("{}{}", base_address.clone(), "/client/databases");
            request::get_data(url, db_request_json, db_callback);
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <h2 class="subtitle">{"Connect To RCD"}</h2>

                    <label for="ip_address">{ "IP Address" }</label>
                    <input type="text" class="input" id ="ip_address" placeholder="localhost" ref={&ui.addr}/>

                    <label for="port">{ "Port Number" }</label>
                    <input type="text" class="input"  id="port" placeholder="50055" ref={&ui.port} />

                    <label for="un">{ "User Name" }</label>
                    <input type="text" class="input"  id="un" placeholder="tester" ref={&ui.un} />

                    <label for="pw">{ "Pw" }</label>
                    <input type="text" class="input"  id="pw" placeholder="123456" ref={&ui.pw} />

                    <input type="button" class="button is-primary" id="submit" value="Connect" {onclick}/>
                </div>
            </div>

            <div class="container">
                <div class="box">
                    <h1 class="subtitle"> {"Databases"} </h1>

                    <p>{"After connecting, the list of databases on the rcd instance will appear here."}</p>
                    <div class="content">
                        <ul>
                            {
                                (*database_names).clone().into_iter().map(|name| {
                                    let db_name = name.clone();
                                    html!{<div key={db_name.clone()}>
                                    <li>{db_name.clone()}</li></div>
                            }
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct DatabaseDetails {
    pub dbs: Vec<DatabaseSchema>,
}

impl DatabaseDetails {
    pub fn new() -> DatabaseDetails {
        return DatabaseDetails { dbs: Vec::new() };
    }
}

#[derive(Clone, Debug)]
pub struct ConnectUi {
    pub addr: NodeRef,
    pub port: NodeRef,
    pub un: NodeRef,
    pub pw: NodeRef,
}

impl ConnectUi {
    pub fn new() -> ConnectUi {
        return ConnectUi {
            addr: NodeRef::default(),
            port: NodeRef::default(),
            un: NodeRef::default(),
            pw: NodeRef::default(),
        };
    }
}
