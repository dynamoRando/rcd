use yew::NodeRef;


use yew::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct ConnectionUi {
    pub addr: NodeRef,
    pub port: NodeRef,
    pub un: NodeRef,
    pub pw: NodeRef,
}

impl ConnectionUi{
    pub fn new() -> ConnectionUi {
        return ConnectionUi{
            addr: NodeRef::default(),
            port: NodeRef::default(),
            un: NodeRef::default(),
            pw: NodeRef::default()
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ConnectionData {
    pub addr: String,
    pub port: u32,
    pub un: String,
    pub pw: String,
}