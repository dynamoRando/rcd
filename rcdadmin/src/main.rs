use rcd_conn_ui::{RcdConn, RcdConnUi};
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement};
use yew::{html::Scope, prelude::*, virtual_dom::AttrValue};
mod rcd_conn_ui;
use rcd_messages::client::{AuthRequest, GetDatabasesReply, GetDatabasesRequest, TestRequest};
use reqwasm::http::{Method, Request};

pub enum AppMessage {
    Connect(),
    GetDatabases(AttrValue),
    GetTablesForDatabase(String),
}

struct ApplicationState {
    conn_ui: RcdConnUi,
}

impl ApplicationState {}

struct RcdAdminApp {
    state: ApplicationState,
}

#[derive(Clone, PartialEq, Deserialize)]
struct AdminMsg {
    msg: String,
}

impl RcdAdminApp {
    pub fn view_input_for_connection(&self, link: &Scope<Self>) -> Html {
        html! {
           <div>
           <h1> {"Connect to rcd"} </h1>
           <label for="ip_address">{ "IP Address" }</label>
            <input type="text" id ="ip_address" placeholder="localhost" ref={&self.state.conn_ui.ip}/>
            <label for="port">{ "Port Number" }</label>
            <input type="text" id="port" placeholder="8000" ref={&self.state.conn_ui.port} />
            <label for="un">{ "User Name" }</label>
            <input type="text" id="un" placeholder="tester" ref={&self.state.conn_ui.un} />
            <label for="pw">{ "Pw" }</label>
            <input type="text" id="pw" placeholder="123456" ref={&self.state.conn_ui.pw} />
            <input type="button" id="submit" value="Connect" onclick={link.callback(|_|
                {
                    console::log_1(&"clicked".into());
                    AppMessage::Connect()
                })}/>
           </div>
        }
    }

    pub fn view_databases(&self, link: &Scope<Self>) -> Html {
        let mut db_names: Vec<String> = Vec::new();

        for db in &self.state.conn_ui.conn.databases {
            db_names.push(db.database_name.clone());
        }

        html! {
           <div>
           <h1> {"Databases"} </h1>
           <ul>
           {
            db_names.into_iter().map(|name| {
                let db_name = name.clone();
                html!{<div key={db_name.clone()}>
                <li onclick={link.callback(move |_| AppMessage::GetTablesForDatabase(name.clone()))}>{db_name.clone()}</li></div>}
            }).collect::<Html>()
        }</ul>
           </div>
        }
    }

    pub fn view_tables_for_database(&self, _link: &Scope<Self>) -> Html {
        let db_name = self.state.conn_ui.conn.current_db_name.clone();

        if db_name == "" {
            html! {
                <div/>
            }
        } else {
            let tables = self
                .state
                .conn_ui
                .conn
                .databases
                .iter()
                .find(|x| x.database_name.as_str() == db_name)
                .unwrap()
                .tables
                .clone();

            let mut table_names: Vec<String> = Vec::new();
            for table in &tables {
                table_names.push(table.table_name.clone());
            }

            html! {
               <div>
               <h1> {"Tables"} </h1>
               <ul>
               {
                table_names.into_iter().map(|name| {
                    let table_name = name.clone();
                    html!{<div key={table_name.clone()}>
                    <li>{table_name.clone()}</li></div>}
                }).collect::<Html>()
            }</ul>
               </div>
            }
        }
    }
}

impl Component for RcdAdminApp {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let conn = RcdConn {
            un: "tester".to_string(),
            pw: "123456".to_string(),
            ip: "localhost".to_string(),
            port: 8000,
            databases: Vec::new(),
            current_db_name: "".to_string(),
        };

        let conn_ui = RcdConnUi {
            conn,
            un: NodeRef::default(),
            pw: NodeRef::default(),
            ip: NodeRef::default(),
            port: NodeRef::default(),
            databases: NodeRef::default(),
        };

        let app_state = ApplicationState { conn_ui };

        Self { state: app_state }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <h1>{ "Rcd Admin" }</h1>
               <section class ="rcdadmin">
                <header class="header">
                    { self.view_input_for_connection(ctx.link()) }
                </header>
               </section>
               <section class ="databases">
                {self.view_databases(ctx.link())}
               </section>
               <section class ="tables">
               {self.view_tables_for_database(ctx.link())}
              </section>
            </div>
        }
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // console::log_1(&"update".into());
        match msg {
            AppMessage::Connect() => {
                let un = &self.state.conn_ui.un;
                let pw = &self.state.conn_ui.pw;
                let ip = &self.state.conn_ui.ip;
                let port = &self.state.conn_ui.port;

                let un_val = un.cast::<HtmlInputElement>().unwrap().value();
                let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
                let ip_val = ip.cast::<HtmlInputElement>().unwrap().value();
                let port_val = port.cast::<HtmlInputElement>().unwrap().value();

                /*
                   console::log_1(&un_val.clone().into());
                   console::log_1(&pw_val.clone().into());
                   console::log_1(&ip_val.clone().into());
                   console::log_1(&port_val.clone().into());
                */

                let request = TestRequest {
                    request_time_utc: "".to_string(),
                    request_origin_url: "".to_string(),
                    request_origin_ip4: "".to_string(),
                    request_origin_ip6: "".to_string(),
                    request_port_number: 0,
                    request_echo_message: "rcdadmin-test".to_string(),
                };

                let base_address =
                    format!("{}{}{}{}", "http://", ip_val.to_string(), ":", port_val);

                let request_json = serde_json::to_string(&request).unwrap();

                let auth_request = AuthRequest {
                    user_name: "tester".to_string(),
                    pw: "123456".to_string(),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let db_request = GetDatabasesRequest {
                    authentication: Some(auth_request),
                };

                let db_request_json = serde_json::to_string(&db_request).unwrap();
                let db_callback = ctx.link().callback(AppMessage::GetDatabases);
                let url = format!("{}{}", base_address.clone(), "/client/databases");
                get_data(url, db_request_json, db_callback);
            }
            AppMessage::GetDatabases(db_response) => {
                console::log_1(&db_response.to_string().clone().into());
                let db_response: GetDatabasesReply =
                    serde_json::from_str(&db_response.to_string()).unwrap();
                if db_response.authentication_result.unwrap().is_authenticated {
                    self.state.conn_ui.conn.databases = db_response.databases.clone();
                }
            }
            AppMessage::GetTablesForDatabase(db_name) => {
                // console::log_1(&"AppMessage::GetTablesForDatabase".into());
                // console::log_1(&db_name.clone().into());
                self.state.conn_ui.conn.current_db_name = db_name;
                self.view_tables_for_database(ctx.link());
            }
        }
        true
    }
}

fn main() {
    yew::start_app::<RcdAdminApp>();
}

pub fn get_data(url: String, body: String, callback: Callback<AttrValue>) {
    spawn_local(async move {
        let http_response = Request::new(&url)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        callback.emit(AttrValue::from(http_response));
    });
}
