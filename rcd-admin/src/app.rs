use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::home::Home;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    // #[at("/db")]
    // Db,
    // #[at("/sql")]
    // Sql,
    // #[at("/contract")]
    // Contract,
    // #[at("/participant")]
    // Participant,
    // #[at("/404")]
    // NotFound,
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
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => 
        {
            html! { <Home /> }
        },
        // Route::Db => todo!(),
        // Route::Sql => todo!(),
        // Route::Contract => todo!(),
        // Route::Participant => todo!(),
        // Route::NotFound => todo!(),

        /*
        Route::Post { id } => {
            html! { <Post seed={id} /> }
        }
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Author { id } => {
            html! { <Author seed={id} /> }
        }
        Route::Authors => {
            html! { <AuthorList /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
         */
    }
}

