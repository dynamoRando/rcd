use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    con::PROXY_ADDR,
    log::log_to_console,
    request::proxy::{set_proxy, set_proxy_token, RcdProxy, clear_proxy_token},
};

#[function_component]
pub fn Login() -> Html {
    let ui_un = use_node_ref();
    let ui_pw = use_node_ref();

    let login_result = use_state_eq(move || String::from(""));

    let onclick = {
        let ui_un = ui_un.clone();
        let ui_pw = ui_pw.clone();
        let login_result = login_result.clone();

        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let ui_pw = ui_pw.clone();
            let login_result = login_result.clone();

            let un = &ui_un;
            let pw = &ui_pw;

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();

            let mut proxy = RcdProxy::new(PROXY_ADDR);
            set_proxy(&proxy);

            let u = un_val;
            let p = pw_val;

            spawn_local(async move {
                let result = proxy.auth_for_token(&u, &p).await;
                log_to_console("{result:?}".to_string());
                match result {
                    Ok(token) => {
                        if token.is_logged_in {
                            set_proxy_token(token);
                            login_result.set("Login success! You will be 
                            redirected in a moment to the app".to_string());
                        } else {
                            login_result.set("Login failed.".to_string());
                        }
                    }
                    Err(e) => log_to_console(e),
                };
            })
        })
    };

    let onclick_logout = {
        let ui_un = ui_un.clone();
        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let un = &ui_un;
            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let mut proxy = RcdProxy::new(PROXY_ADDR);
            set_proxy(&proxy);
            clear_proxy_token();
            spawn_local(async move {
                proxy.logout(&un_val).await;
            })
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <div class="has-text-centered">
                        <h1 class="subtitle"> {"Login"} </h1>
                        <label for="ip_address">{ "User Name" }</label>
                        <input type="text" class="input" id ="username" placeholder="username" ref={&ui_un}/>

                        <label for="port">{ "Password" }</label>
                        <input type="text" class="input"  id="pw" placeholder="pw" ref={&ui_pw} />

                        <div class="buttons">
                        <button type="button" class="button is-primary" id="login" value="Login" {onclick}>
                            <span class="mdi mdi-login">{" Login"}</span>
                        </button>
                        <button type="button" class="button is-info" id="logout" value="Logout" onclick={onclick_logout}>
                        <span class="mdi mdi-logout">{" Logout"}</span>
                    </button>
                        </div>
                        <h2 class="subtitle"> {"Login Result"} </h2>
                        <p>{(*login_result).clone()}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
