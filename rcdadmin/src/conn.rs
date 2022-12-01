use rcd_messages::client::{AuthRequest, GetDatabasesRequest};
use web_sys::{console, HtmlInputElement};
use yew::html;
use yew::{html::Scope, Context, Html};

use crate::{request, AppMessage, RcdAdminApp};

pub fn handle_connect(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    let un = &app.state.conn_ui.un;
    let pw = &app.state.conn_ui.pw;
    let ip = &app.state.conn_ui.ip;
    let port = &app.state.conn_ui.port;

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
    let db_callback = ctx.link().callback(AppMessage::GetDatabases);
    let url = format!("{}{}", base_address.clone(), "/client/databases");
    request::get_data(url, db_request_json, db_callback);

    let auth_request_json = serde_json::to_string(&auth_request).unwrap();

    app.state.conn_ui.conn.auth_request_json = auth_request_json.clone();
    app.state.conn_ui.conn.url = base_address.clone();
}

pub fn view_input_for_connection(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {

    let is_visible = !app.state.page_ui.conn_is_visible;


    html! {
       <div hidden={is_visible}>
       <h1> {"Connect to rcd"} </h1>
       <label for="ip_address">{ "IP Address" }</label>
        <input type="text" id ="ip_address" placeholder="localhost" ref={&app.state.conn_ui.ip}/>
        <label for="port">{ "Port Number" }</label>
        <input type="text" id="port" placeholder="8000" ref={&app.state.conn_ui.port} />
        <label for="un">{ "User Name" }</label>
        <input type="text" id="un" placeholder="tester" ref={&app.state.conn_ui.un} />
        <label for="pw">{ "Pw" }</label>
        <input type="text" id="pw" placeholder="123456" ref={&app.state.conn_ui.pw} />
        <input type="button" id="submit" value="Connect" onclick={link.callback(|_|
            {
                console::log_1(&"clicked".into());
                AppMessage::Connect()
            })}/>
       </div>
    }
}
