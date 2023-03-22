use yew::{prelude::*};
use yew_router::{prelude::{use_navigator}};

use crate::{request::proxy::has_proxy_token, app::Route};



#[function_component]
pub fn MyRcd() -> Html {
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
            <div>
                <p>{"my rcd placeholder"}</p>
            </div>
        }
    }
}
