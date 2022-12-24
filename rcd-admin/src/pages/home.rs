use yew::{function_component, Html, html};

use crate::components::connection::{Connection};


#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Welcome to RCD Admin" }</h1>
                </div>
            </div>

            <div class="tile is-parent container">
                <Connection />
            </div>
        </div>
    }
}
