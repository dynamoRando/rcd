use rcd_conn_ui::{RcdConn, RcdConnUi};
use serde::Deserialize;
use web_sys::{console, HtmlInputElement};
use yew::{html::Scope, prelude::*};
mod rcd_conn_ui;
use rcd_messages::client::{AuthRequest, GetDatabasesRequest, TestRequest};
use reqwasm::http::{Method, Request};

pub enum AppMessage {
    Connect(),
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

    #[allow(dead_code)]
    fn view_connection(&self, _link: &Scope<Self>) -> Html {
        html! {
            <div>
            <li>
                <label>{ self.state.conn_ui.conn.ip.to_string() }</label>
                <label>{ self.state.conn_ui.conn.port.to_string() }</label>
            </li>
            </div>
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
        };

        let conn_ui = RcdConnUi {
            conn,
            un: NodeRef::default(),
            pw: NodeRef::default(),
            ip: NodeRef::default(),
            port: NodeRef::default(),
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
            </div>
        }
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        console::log_1(&"update".into());
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

                console::log_1(&un_val.clone().into());
                console::log_1(&pw_val.clone().into());
                console::log_1(&ip_val.clone().into());
                console::log_1(&port_val.clone().into());

                let request = TestRequest {
                    request_time_utc: "".to_string(),
                    request_origin_url: "".to_string(),
                    request_origin_ip4: "".to_string(),
                    request_origin_ip6: "".to_string(),
                    request_port_number: 0,
                    request_echo_message: "rcdadmin-test".to_string(),
                };

                let base_address = format!("{}{}{}{}", "http://", ip_val.to_string(), ":", port_val);
                let url =  format!("{}{}", base_address.clone(), "/client/version");

                let base_address2 = base_address.clone();
                let base_address3 = base_address.clone();
                let base_address4 = base_address.clone();

                let request_json = serde_json::to_string(&request).unwrap();
                let request_json2 = request_json.clone();
                let request_json3 = request_json.clone();

                // we expect to get the "Status From Rocket" message
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("{}{}", base_address2.clone(), "/client/status");
                    let res = Request::get(&url.to_string())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    console::log_1(&res.into());
                });

                // test to see if we can POST
                wasm_bindgen_futures::spawn_local(async move {
                    console::log_1(&"local".into());
                    let url = format!("{}{}", base_address3.clone(), "/client/version");

                    // let js = wasm_bindgen::JsValue::from_str(&request_json2);
                    console::log_1(&request_json.into());

                    let http_response = Request::new(&url)
                        .method(Method::POST)
                        .header("Content-Type", "application/json")
                        .body(request_json2)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    console::log_1(&"response".into());
                    console::log_1(&http_response.into());
                });

                // test to see if we can get databases
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
                let db_request_json2 = serde_json::to_string(&db_request).unwrap();
                wasm_bindgen_futures::spawn_local(async move {
                    console::log_1(&"local".into());
                    let url = format!("{}{}", base_address4.clone(), "/client/databases");

                    // let js = wasm_bindgen::JsValue::from_str(&request_json2);
                    console::log_1(&db_request_json.into());

                    let http_response = Request::new(&url)
                        .method(Method::POST)
                        .header("Content-Type", "application/json")
                        .body(db_request_json2)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    console::log_1(&"response".into());
                    console::log_1(&http_response.into());
                });
            }
        }
        true
    }
}

fn main() {
    yew::start_app::<RcdAdminApp>();
}
