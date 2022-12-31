use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::behaviors::Behaviors;
use crate::pages::contracts::Contracts;
use crate::pages::coop_hosts::CooperativeHosts;
use crate::pages::databases::databases::Databases;
use crate::pages::home::Home;
use crate::pages::hosts::Hosts;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::participants::Participants;
use crate::pages::sql::sql::Sql;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/databases")]
    Databases,
    #[at("/sql")]
    Sql,
    #[at("/contracts")]
    Contracts,
    #[at("/hosts")]
    Hosts,
    #[at("/participants")]
    Participants,
    #[at("/behaviors")]
    Behaviors,
    #[at("/CooperativeHosts")]
    CooperativeHosts,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

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
            <Nav />

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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Databases => {
            html! { <Databases /> }
        },
        Route::Sql => {
            html! { <Sql /> }
        },
        Route::NotFound => {
            html! { <PageNotFound /> }
        },
        Route::Contracts => {
            html! { <Contracts /> }
        },
        Route::Hosts => {
            html! { <Hosts /> }
        },
        Route::Participants => {
            html! { <Participants /> }
        },
        Route::Behaviors => {
            html! { <Behaviors /> }
        },
        Route::CooperativeHosts => {
            html! { <CooperativeHosts /> }
        },
    }
}
