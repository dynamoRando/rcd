use yew::prelude::*;
use yew_router::prelude::*;

use crate::{storage::get_un, Route};

#[derive(Properties, PartialEq, Debug)]
pub struct NavProps {
    pub is_logged_in: UseStateHandle<bool>,
}

#[function_component]
pub fn Nav(props: &NavProps) -> Html {
    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "is-active" } else { "" };
    let un = get_un();

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">
                        { "Shark! " }
                        <span class="mdi mdi-shark-fin">
                        </span>
                    </h1>

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
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Register}>
                    { "Register" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                    { "Login" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Events}>
                    { "Events" }
                    </Link<Route>>
                </div>
            </div>
            {
                if *props.is_logged_in {
                    html! {
                        <div>
                            <div class="navbar-item">
                                    <div class="buttons">
                                        <button class="button is-warning">
                                        {"User Name: "}{un}
                                        </button>
                                    </div>
                                </div>
                        </div>
                    }
                }
                else {
                    html! {
                        <div>
                            <div class="navbar-item">
                                <button class="button is-light">
                                    <span class="mdi mdi-account-cancel">{" Not Logged In"}</span>
                                </button>
                            </div>
                       </div>
                        }
                }
            }
        </nav>
    }
}
