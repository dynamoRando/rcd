use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

use crate::{request::proxy::has_proxy_token, app::Route};

#[function_component]
pub fn RcdDb() -> Html {
    let navigator = use_navigator().unwrap();
    if !has_proxy_token() {
        navigator.push(&Route::Login);
        html! {
            <div>
                <p>{"You are not logged in, redirecting to login page."}</p>
            </div>
        }
    } else {
        html! {
            <div>{"Databases"}</div>
        }
    }
}