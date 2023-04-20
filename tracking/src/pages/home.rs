use yew::{platform::spawn_local, prelude::*};

use crate::{logging::log_to_console, repo::{Repo}, storage::get_api_addr};

#[function_component]
pub fn Home() -> Html {
    let website_version = env!("CARGO_PKG_VERSION");


    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Shark!" }</h1></p>
                        <p>{"Website v: "}{website_version}</p>
                    </div>
                </div>
            </div>

        </div>
    }
}
