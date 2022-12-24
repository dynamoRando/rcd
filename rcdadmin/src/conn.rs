use rcd_messages::client::{AuthRequest, GetDatabasesRequest};
use web_sys::{console, HtmlInputElement};
use yew::html;
use yew::{html::Scope, Context, Html};

use crate::rcd_ui::PageUi;
use crate::state::connection::RcdConnection;
use crate::{request, AppMessage, RcdAdminApp};

pub fn handle_connect(connection: &mut RcdConnection, ctx: &Context<RcdAdminApp>) {
    let un = &connection.ui.username;
    let pw = &connection.ui.password;
    let ip = &connection.ui.ip;
    let port = &connection.ui.port;
    let http_port = &connection.ui.http_port;

    let un_val = un.cast::<HtmlInputElement>().unwrap().value();
    let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
    let ip_val = ip.cast::<HtmlInputElement>().unwrap().value();
    let _port_val = port.cast::<HtmlInputElement>().unwrap().value();
    let http_port_val = http_port.cast::<HtmlInputElement>().unwrap().value();

    let base_address = format!(
        "{}{}{}{}",
        "http://",
        ip_val.to_string(),
        ":",
        http_port_val
    );

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
    let db_callback = ctx
        .link()
        .callback(AppMessage::Db_HttpResponse_GetDatabases);
    let url = format!("{}{}", base_address.clone(), "/client/databases");
    request::get_data(url, db_request_json, db_callback);

    let auth_request_json = serde_json::to_string(&auth_request).unwrap();

    connection.data.active.url = base_address.clone();
    connection.data.active.authentication_json = auth_request_json.clone();
}

pub fn view_input_for_connection(
    page: &PageUi,
    link: &Scope<RcdAdminApp>,
    connection: &RcdConnection,
) -> Html {
    let is_visible = !page.conn_is_visible;

    html! {
    <div hidden={is_visible}>
        <div class="box">
            <h1 class="subtitle"> {"Connect to rcd"} </h1>

                <label for="ip_address">{ "IP Address" }</label>
                <input type="text" class="input" id ="ip_address" placeholder="localhost" ref={&connection.ui.ip}/>

                <label for="port">{ "Port Number" }</label>
                <input type="text" class="input"  id="port" placeholder="50051" ref={&connection.ui.port} />
                
                <label for="http_port">{ "HTTP Port Number" }</label>
                <input type="text" class="input"  id="http_port" placeholder="50055" ref={&connection.ui.http_port} />
                
                <label for="un">{ "User Name" }</label>
                <input type="text" class="input"  id="un" placeholder="tester" ref={&connection.ui.username} />
                
                <label for="pw">{ "Pw" }</label>
                <input type="text" class="input"  id="pw" placeholder="123456" ref={&connection.ui.password} />
                
                <input type="button" class="button is-primary" id="submit" value="Connect" onclick={link.callback(|_|
                {
                    console::log_1(&"clicked".into());
                    AppMessage::Connect()
                })}/>
        </div>
    </div>
    }
}
