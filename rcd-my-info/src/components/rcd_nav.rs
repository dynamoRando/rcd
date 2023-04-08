use crate::{app::Route, request::proxy::get_proxy_token};
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
                         <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdContracts}>
                        { "Contracts" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdHostInfo}>
                        { "Host Info" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdPart}>
                        { "Participants" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdBehavior}>
                        { "Behaviors" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdCoopHost}>
                        { "Cooperative Hosts" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdSettings}>
                        { "Settings" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MyRcdLogs}>
                        { "Logs" }
                        </Link<Route>>
                    </div>
                </div>
                <div class="buttons">
                        {
                            if *props.is_logged_in {
                                let token = get_proxy_token();
                                let id = token.id.as_ref().unwrap().clone();
                                html! {
                                    <div class="navbar-item">
                                        <div class="field">
                                        <input type="text" class="input" size=36
                                        id ="account_id" placeholder="Account Id"
                                        value={id} readonly=true />
                                    </div>
                                </div>
                                }
                            }
                            else {
                                html! {
                                    <button class="button is-light">
                                    <span class="mdi mdi-account-cancel">{" Not Logged In"}</span>
                                    </button>
                                    }
                            }
                        }
                        </div>
            </nav>
        </div>
    )
}
