use serde::{Serialize, Deserialize};
use yew::{NodeRef, Html, Component};

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

    fn create(_ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        todo!()
    }
}