use rcd_conn_ui::{RcdConn, RcdConnUi};
use serde::{Serialize, Deserialize};
use web_sys::console;
use yew::{prelude::*, html::Scope};
mod rcd_conn_ui;

pub enum AppMessage{
    Connect(),
}

struct ApplicationState {
    conn_ui: RcdConnUi,
}

impl ApplicationState {
}


struct RcdAdminApp{
    state: ApplicationState,
}

impl RcdAdminApp{
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

    fn view_connection(&self, link: &Scope<Self>) -> Html {
        html!{
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

    fn create(ctx: &Context<Self>) -> Self {
        let conn = RcdConn {
            un: "".to_string(),
            pw: "".to_string(),
            ip: "".to_string(),
            port: 0,
        };

        let conn_ui = RcdConnUi{
            conn,
            un: NodeRef::default(),
            pw: NodeRef::default(),
            ip: NodeRef::default(),
            port: NodeRef::default(),
        };

        let app_state = ApplicationState {
            conn_ui,
        };

        Self { state: app_state }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
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
}

fn main() {
    yew::start_app::<RcdAdminApp>();
}

