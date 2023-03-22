use crate::{app::Route, request::proxy::has_proxy_token};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct RcdStatusProps {
    pub is_logged_in: UseStateHandle<bool>,
    pub status_message: UseStateHandle<String>,
}


#[function_component]
pub fn RcdNav(props: &RcdStatusProps) -> Html {
    
    /* 
        let navigator = use_navigator().unwrap();

        if !has_proxy_token() {
            navigator.push(&Route::Login);

            html! {
                <div>
                    <p>{"You are not logged in, redirecting to login page."}</p>
                </div>
            }
        } else {
            html! {
                <div>
                    <p>{"my rcd placeholder"}</p>
                </div>
            }
        }
    */

    let navbar_active = use_state_eq(|| false);
    navbar_active.set(*props.is_logged_in);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };
    let active_class = if !*navbar_active { "is-invisible" } else { "" };
    html!(
        <div>
            <nav class="navbar is-light">
                <div class="navbar-brand">
                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={toggle_navbar}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdDb}>
                            { "Databases" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdSql}>
                        { "Sql" }
                        </Link<Route>>
                    </div>
                </div>
                    </nav>
        </div>
    )
}
