use yew::{function_component, html, Html};
use crate::pages::sql::sql::SqlProps;

#[allow(unused_variables)]
#[function_component]
pub fn SqlResult(SqlProps { state }: &SqlProps) -> Html {

    let mut text = String::from("");

    if state.is_some() {
        let state = state.as_ref().clone();
        text = state.unwrap().clone();
    }
    
    html!{
        <div>{text}</div>
    }
}