use rcd_messages::client::DatabaseSchema;
use serde::{Deserialize, Serialize};
use yew::{Component, Html, NodeRef};

use crate::AppMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct RcdConn {
    pub un: String,
    pub pw: String,
    pub ip: String,
    pub port: u32,
    pub databases: Vec<DatabaseSchema>,
    pub current_db_name: String,
    pub current_table_name: String,
    pub sql_input: String,
    pub sql_output: String,
    pub url: String,
    pub auth_request_json: String,
}

pub struct RcdConnUi {
    pub conn: RcdConn,
    pub un: NodeRef,
    pub pw: NodeRef,
    pub ip: NodeRef,
    pub port: NodeRef,
    pub databases: NodeRef,
    pub sql: RcdInputOutputUi,
    pub sql_text_result: String,
}

impl RcdConnUi {}

impl Component for RcdConnUi {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        todo!()
    }
}

#[allow(dead_code)]
pub struct RcdInputOutputUi {
    pub execute_sql: NodeRef,
    pub sql_result: NodeRef,
    pub db_name: NodeRef,
    pub selected_db_name: String,
}
