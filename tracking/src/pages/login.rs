use tracking_model::user::Token;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::{logging::log_to_console, repo::Repo, storage::{save_token, save_un}};

#[function_component]
pub fn Login() -> Html {
    let login_result = use_state_eq(|| "".to_string());
    let ui_un = use_node_ref();

    let onclick = {
        let ui_un = ui_un.clone();
        let login_result = login_result.clone();

        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let login_result = login_result.clone();

            let un = &ui_un;

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let login_result = login_result.clone();
                let result = Repo::login(&un_val).await;
                let message = format!("{:?}", result);
                log_to_console(&message);

                match result {
                    Ok(token) => {
                        if token.is_logged_in {
                            login_result.set("Logged In!".to_string());
                            save_token(token);
                            save_un(&un_val);
                        }
                    }
                    Err(_) => {
                        login_result.set("Failed to login!".to_string());
                    }
                }
            });
        })
    };

    let onclick_logout = {
        let ui_un = ui_un.clone();
        let login_result = login_result.clone();

        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let login_result = login_result.clone();

            let un = &ui_un;

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let login_result = login_result.clone();
                let result = Repo::logout(&un_val).await;
                let message = format!("{:?}", result);
                log_to_console(&message);

                match result {
                    Ok(_) => {
                        save_token(Token::default());
                        save_un("");
                        login_result.set("Logged Out!".to_string());
                    }
                    Err(_) => {
                        login_result.set("Failed to logout!".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Login" }</h1></p>

                        <label for="ip_address">{ "User Name" }</label>
                        <input type="text" class="input" id ="username" placeholder="username" ref={&ui_un}/>

                        <div class="buttons">
                            <button type="button" class="button is-primary" id="login" value="Login" {onclick}>
                                <span class="mdi mdi-login">{" Login"}</span>
                            </button>
                            <button type="button" class="button is-info" id="logout" value="Logout" onclick={onclick_logout}>
                                <span class="mdi mdi-logout">{" Logout"}</span>
                            </button>
                        </div>

                        <p>{"Login Status: "}{(*login_result).clone()}</p>
                    </div>
                </div>
            </div>

        </div>
    }
}
