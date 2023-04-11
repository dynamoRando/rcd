use yew::prelude::*;

use crate::repo::Repo;

pub mod add;
pub mod view;

#[function_component]
pub fn Events() -> Html {
    // let events = Repo::get_events().await;

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Events" }</h1></p>

                        // <aside class="menu">
                        //     <p class="menu-label">
                        //      {"Events"}
                        //     </p>
                        //     <ul class="menu-list">
                        //         <li><a>{"Main Events"}</a></li>
                        //         <li><a>{"Associated Events"}</a></li>
                        //     </ul>
                        // </aside>


                    </div>
                </div>
            </div>
        </div>
    }
}
