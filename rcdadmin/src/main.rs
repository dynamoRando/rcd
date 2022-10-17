use rcd::{RcdConn, RcdDbMetadata};
use rcdproto::rcdp::DatabaseSchema;
use yew::html;
use yew::prelude::*;

mod rcd;

#[allow(dead_code)]
// holds the state of the application and data
#[derive(Debug)]
struct ApplicationState {
    connection: RcdConn,
    databases: Vec<RcdDbMetadata>,
    active_database: RcdDbMetadata,
    active_database_schema: DatabaseSchema,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
