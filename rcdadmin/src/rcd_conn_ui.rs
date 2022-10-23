use serde::{Serialize, Deserialize};
use web_sys::console;
use yew::{NodeRef, Html, html::{Scope}, Component};
use yew::{html};

use crate::AppMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct RcdConn{
    pub un: String,
    pub pw: String,
    pub ip: String,
    pub port: u32,   
}

pub struct RcdConnUi{
    pub conn: RcdConn,
    pub un: NodeRef,
    pub pw: NodeRef,
    pub ip: NodeRef,
    pub port: NodeRef,
}

impl RcdConnUi {
   
}

impl Component for RcdConnUi {
    type Message = AppMessage;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        todo!()
    }
}