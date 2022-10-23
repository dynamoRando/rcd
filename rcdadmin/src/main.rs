use rcd_conn_ui::{RcdConn, RcdConnUi};
use serde::Deserialize;
use web_sys::{console, HtmlInputElement};
use yew::{html::Scope, prelude::*};
mod rcd_conn_ui;
use reqwasm::http::{Request, Headers, Method};
use rcd_messages::client::TestRequest;

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
            <input type="text" id ="ip_address" ref={&self.state.conn_ui.ip}/>
            <label for="port">{ "Port Number" }</label>
            <input type="text" id="port" ref={&self.state.conn_ui.port} />
            <label for="un">{ "User Name" }</label>
            <input type="text" id="un" ref={&self.state.conn_ui.un} />
            <label for="pw">{ "Pw" }</label>
            <input type="text" id="pw" ref={&self.state.conn_ui.pw} />
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
            un: "".to_string(),
            pw: "".to_string(),
            ip: "".to_string(),
            port: 0,
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

                console::log_1(&un_val.into());
                console::log_1(&pw_val.into());
                console::log_1(&ip_val.into());
                console::log_1(&port_val.into());

                let request = TestRequest {
                    request_time_utc: "".to_string(),
                    request_origin_url: "".to_string(),
                    request_origin_ip4: "".to_string(),
                    request_origin_ip6: "".to_string(),
                    request_port_number: 0,
                    request_echo_message: "rcdadmin-test".to_string(),
                };

                let url = "http://127.0.0.1:8000/client/version";

                let request_json = serde_json::to_string(&request).unwrap();
                let request_json2 = request_json.clone();
                let request_json3 = request_json.clone();

                // wasm_bindgen_futures::spawn_local(async move {                    
                //     let res = Request::get("http://127.0.0.1:8000/client/status")
                //         .send()
                //         .await
                //         .unwrap()
                //         .text()
                //         .await
                //         .unwrap();

                //     console::log_1(&res.into());
                // });

                // test to see if we can POST
                wasm_bindgen_futures::spawn_local(async move {
                    console::log_1(&"local".into());
                    let url = "http://localhost:8000/client/version";

                    // let js = wasm_bindgen::JsValue::from_str(&request_json2);
                    console::log_1(&request_json.into());
                    
                    let http_response = Request::new(url)
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
            }
        }
        true
    }
}

fn main() {
    yew::start_app::<RcdAdminApp>();
}
