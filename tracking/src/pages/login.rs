use tracking_model::user::Token;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    logging::log_to_console,
    repo::Repo,
    storage::{save_api_addr, save_token, save_uid, save_un},
};

#[function_component]
pub fn Login() -> Html {
    let ui_un = use_node_ref();
    let ui_api = use_node_ref();

    let login_result = use_state_eq(|| "".to_string());
    let api_location = use_state_eq(|| "".to_string());

    let onclick = {
        let ui_un = ui_un.clone();
        let ui_api = ui_api.clone();

        let login_result = login_result.clone();

        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let ui_api = ui_api.clone();

            let login_result = login_result.clone();

            let un = &ui_un;
            let un_val = un.cast::<HtmlInputElement>().unwrap().value();

            let api_val = ui_api.cast::<HtmlInputElement>().unwrap().value();
            log_to_console(&api_val);
            save_api_addr(&api_val);

            spawn_local(async move {
                let login_result = login_result.clone();
                let result = Repo::login(&un_val).await;
                let message = format!("login response: {:?}", result);
                log_to_console(&message);

                match result {
                    Ok(token) => {
                        if token.is_logged_in {
                            login_result.set("Logged In!".to_string());
                            save_token(token);
                            save_un(&un_val);
                            spawn_local(async move {
                                get_and_save_uid(&un_val).await;
                            });
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
                        save_uid(0);
                        login_result.set("Logged Out!".to_string());
                    }
                    Err(_) => {
                        login_result.set("Failed to logout!".to_string());
                    }
                }
            });
        })
    };

    let onchange = {
        log_to_console("onchange");

        let ui_api = ui_api.clone();
        let api_location = api_location.clone();

        Callback::from(move |_| {
            let ui_api = ui_api.clone();
            let api_location = api_location.clone();

            let api_val = ui_api.cast::<HtmlInputElement>();

            if api_val.is_some() {
                let api_val = ui_api.cast::<HtmlInputElement>().unwrap().value();
                log_to_console(&api_val);
                api_location.set(api_val);
            }
        })
    };

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Login" }</h1></p>

                        <label for="username">{ "User Name" }</label>
                        <input type="text" class="input" id ="username" placeholder="username" ref={&ui_un}/>

                        <label for="api_address">{ "API " }</label>
                        // <input type="text" class="input" id ="api_address" placeholder="http://localhost:8020/" ref={&ui_api}/>
                        <div class="select is-multiple">
                                <select
                                    name="select_api"
                                    id="select_api"
                                    ref={&ui_api}
                                    onchange={&onchange}
                                >
                                    <option value="SELECT EVENT">{"SELECT API LOCATION"}</option>
                                    <option value={"http://localhost:8020/"}>{"localhost"}</option>
                                    <option value={"http://shark.home:8020/"}>{"shark"}</option>
                            </select>
                        </div>

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

async fn get_and_save_uid(un: &str) {
    let result = Repo::get_uid_for_un(un).await;
    match result {
        Ok(uid) => {
            log_to_console(&uid.to_string());
            save_uid(uid);
        }
        Err(_) => {
            log_to_console("unable to get uid for un");
        }
    }
}
