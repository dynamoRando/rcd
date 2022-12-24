use crate::rcd::RcdConnection;

use self::ui::ConnectionUi;
use rcd_messages::client::DatabaseSchema;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Html, Properties, use_state};
pub mod ui;

#[derive(Properties, PartialEq)]
struct ViewDatabaseProperties {
    is_visible: bool,
    databases: Vec<DatabaseSchema>,
}

#[function_component]
pub fn Connection() -> Html {
    let ui = ConnectionUi::new();

    let p = ViewDatabaseProperties {
        is_visible: false,
        databases: Vec::new(),
    };

    let ui_state = use_state(|| ui.clone());

   let on_connect = {
        let ui_state = ui_state.clone();
        Callback::from(move |ui: ConnectionUi| ui_state.set(ui.clone()))
    };

    let onclick = Callback::from(move |_| {

        // let c = on_connect.clone();
        // c.emit(ui.clone());

        let ui = ui_state.clone();
        let ui = ui.clone();
        
        let un = &ui.un;
        let pw = &ui.pw;
        let ip = &ui.addr;
        let port = &ui.port;

        let un_val = un.cast::<HtmlInputElement>().unwrap().value();
        let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
        let ip_val = ip.cast::<HtmlInputElement>().unwrap().value();
        let port_val = port.cast::<HtmlInputElement>().unwrap().value();

        let connection = RcdConnection {
            http_addr: ip_val,
            http_port: port_val.parse::<u32>().unwrap(),
            un: un_val,
            pw: pw_val,
        };

        // let databases = connection.databases();

        // p.is_visible = true;
        // p.databases = databases.clone();

        todo!()
    });

    html!(
        <div>
            <label for="address">{ "Address" }</label>
            <input type="text" id ="address" placeholder="localhost" ref={&ui.addr}/>

            <label for="port">{ "Port Number" }</label>
            <input type="text" id="port" placeholder="50055" ref={&ui.port} />

            <label for="un">{ "User Name" }</label>
            <input type="text" id="un" placeholder="tester" ref={&ui.un} />

            <label for="pw">{ "Pw" }</label>
            <input type="text" id="pw" placeholder="123456" ref={&ui.pw} />

            <input type="button" id="submit" value="Connect" {onclick}/>
            <ViewDatabases is_visible = {p.is_visible} databases = {p.databases} />
        </div>
    )
}

#[function_component(ViewDatabases)]
fn view_databases(props: &ViewDatabaseProperties) -> Html {
    let is_visible = props.is_visible;

    let mut db_names: Vec<String> = Vec::new();

    for db in &props.databases {
        db_names.push(db.database_name.clone());
    }

    html!(
        <div hidden={is_visible}>
        <ul>
        {
         db_names.into_iter().map(|name| {
             let db_name = name.clone();
             html!{<div key={db_name.clone()}>
             <li>{db_name.clone()}</li></div>}
         }).collect::<Html>()
     }</ul>
     </div>
    )
}
