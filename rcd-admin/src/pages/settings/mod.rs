use yew::{function_component, html, Html};

#[function_component]
pub fn Settings() -> Html {
    html!(
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle">{"Settings"}</h1>
                </div>
            </div>
        </div>
    )
}