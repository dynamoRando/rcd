use crate::components::nav::Nav;
use crate::repo::Repo;
use futures_util::StreamExt;
use futures_util::future::ready;
use gloo::timers::future::IntervalStream;
use pages::events::Events;
use pages::home::Home;
use pages::login::Login;
use pages::page_not_found::NotFound;
use pages::register::Register;
use storage::get_token;
use tracking_model::event::SharkEvent;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod pages;

pub mod components;
pub mod enter_event;
pub mod event_props;
pub mod logging;
pub mod repo;
pub mod storage;
pub mod view_events;

const DEFAULT_REPO_ADDR: &str = "localhost:8020";

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/events")]
    Events,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn App() -> Html {
    // let app_state = use_state_eq(move || {
    //     let x: Vec<SharkEvent> = Vec::new();
    //     x
    // });

    // {
    //     let app_state = app_state.clone();
    //     spawn_local(async move {
    //         let events = Repo::get_events().await;
    //         if let Ok(events) = events {
    //             app_state.set(events);
    //         }
    //     });
    // }

    let is_logged_in_state = use_state(move || false);
    
    {
        let is_logged_in_state = is_logged_in_state.clone();
        spawn_local(async move {
            IntervalStream::new(1_000)
                .for_each(|_| {
                    check_and_set_login_status(is_logged_in_state.clone());
                    ready(())
                })
                .await;
        });
    }

    html!(
        <BrowserRouter>
        <Nav is_logged_in={is_logged_in_state.clone()}/>
        <main>
            <Switch<Route> render={switch} />
        </main>
        <footer class="footer">
            <div class="content has-text-centered">
                { "Powered by " }
                <a href="https://yew.rs">{ "Yew" }</a>
                { " using " }
                <a href="https://bulma.io">{ "Bulma" }</a>
            </div>
        </footer>
    </BrowserRouter>
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::Events => html! { <Events /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

fn check_and_set_login_status(is_logged_in_state: UseStateHandle<bool>) {
    let is_logged_in = get_token().is_logged_in;
    is_logged_in_state.set(is_logged_in);
}
fn main() {
    yew::Renderer::<App>::new().render();
}



