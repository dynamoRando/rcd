use std::collections::HashMap;

use futures_util::{future::ready, stream::StreamExt};
use gloo::timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::components::rcd_nav::RcdNav;
use crate::components::status::Status;

use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::rcd_admin::databases::db::RcdDb;
use crate::pages::rcd_admin::sql::sqlx::Sql;
use crate::pages::register::Register;
use crate::pages::site_admin::SiteAdmin;
use crate::request::proxy::get_proxy_token;
use crate::request::rcd::{get_status};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/info/register")]
    Register,
    #[at("/info/login")]
    Login,
    #[at("/info/site-admin")]
    SiteAdmin,
    #[at("/my/db")]
    MyRcdDb,
    #[at("/my/sql")]
    MyRcdSql,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    let is_proxy_logged_in_state = use_state(move || false);
    let login_state = is_proxy_logged_in_state.clone();

    let status_state = use_state(move || String::from(""));

    spawn_local(async move {
        IntervalStream::new(1_000)
            .for_each(|_| {
                check_and_set_login_status(login_state.clone());
                ready(())
            })
            .await;
    });

    {
        let status_state = status_state.clone();
        spawn_local(async move {
            IntervalStream::new(1_000)
                .for_each(|_| {
                    check_and_set_status(status_state.clone());
                    ready(())
                })
                .await;
        });
    }

    html! {
        <BrowserRouter>
            <Nav />
            <RcdNav is_logged_in={is_proxy_logged_in_state.clone()} status_message={status_state.clone()}/>
            <Status is_logged_in={is_proxy_logged_in_state.clone()} status_message={status_state.clone()}/>
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
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
        Route::Login => {
            html! { <Login /> }
        }
        Route::SiteAdmin => {
            html! { <SiteAdmin /> }
        }
        Route::MyRcdDb => {
            html! { <RcdDb /> }
        }
        Route::MyRcdSql => {
            html! { <Sql /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn check_and_set_login_status(is_logged_in_state: UseStateHandle<bool>) {
    let is_logged_in = get_proxy_token().is_logged_in;
    is_logged_in_state.set(is_logged_in);
}

fn check_and_set_status(status: UseStateHandle<String>) {
    let status_string = get_status();
    status.set(status_string);
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            // <Nav />

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
        </Router>
    }
}
