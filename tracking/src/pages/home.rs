use yew::{platform::spawn_local, prelude::*};

use crate::{logging::log_to_console, repo::{Repo, REPO_LOCATION}};

#[function_component]
pub fn Home() -> Html {
    let website_version = env!("CARGO_PKG_VERSION");

    let api_version_state = use_state_eq(|| "".to_string());
    let api_location = use_state_eq(|| "".to_string());

    {
        let api_version_state = api_version_state.clone();
        let api_location = api_location.clone();
        spawn_local(async move {
            let api_version_state = api_version_state.clone();
            let api_location = api_location.clone();
            let api_location_val = REPO_LOCATION;
            api_location.set(api_location_val.to_string());
            let api_verison = Repo::get_api_version().await.unwrap();
            log_to_console(&api_verison);
            api_version_state.set(api_verison);
        });
    }

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Shark!" }</h1></p>
                        <p>{"Website v: "}{website_version}</p>
                        <p>{"API v: "}{(*api_version_state).clone()}</p>
                        <p>{"API address: "}{(*api_location).clone()}</p>
                    </div>
                </div>
            </div>

        </div>
    }
}
