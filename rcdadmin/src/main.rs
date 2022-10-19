use rcd::{RcdConn, RcdDbMetadata};
use rcdproto::rcdp::DatabaseSchema;
use yew::html;
use yew::html::Scope;
use yew::prelude::*;
use yew::{Html, NodeRef};
use serde_derive::{Deserialize, Serialize};

mod rcd;


// holds the state of the application and data
#[derive(Debug, Serialize, Deserialize)]
struct ApplicationState {
    connection: RcdConn,
    databases: Vec<RcdDbMetadata>,
    active_database: RcdDbMetadata,
    active_database_schema: DatabaseSchema,
    active_connection: RcdConnUi,
}

// used to pass messages back to the application about what action is requested
enum AppMsg {
}

#[derive(Debug, Serialize, Deserialize)]
struct RcdConnUi {
    connection: RcdConn,
    node_references: Vec<NodeRef>
}

#[derive(Debug, Serialize, Deserialize)]
struct RcdAdminApp {
    application_state: ApplicationState
}

impl RcdAdminApp {
    fn view_input_for_connection(&self, link: &Scope<Self>) -> Html {
        unimplemented!()
    }
}

impl Component for RcdAdminApp {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "rcd admin" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
