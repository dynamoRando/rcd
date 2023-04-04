use yew::prelude::*;


#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Tracking" }</h1></p>
                        <p>{"Notes go here"}</p>
                    </div>
                </div>
            </div>

        </div>
    }
}