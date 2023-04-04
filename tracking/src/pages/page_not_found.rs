use yew::prelude::*;


#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Page Not Found" }</h1></p>
                    </div>
                </div>
            </div>

        </div>
    }
}