use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::{logging::log_to_console, repo::Repo};

#[function_component]
pub fn Register() -> Html {
    let register_result = use_state_eq(|| "".to_string());

    let ui_un = use_node_ref();
    let ui_pw = use_node_ref();
    let ui_host_id = use_node_ref();

    let onclick = {
        let ui_un = ui_un.clone();
        let ui_pw = ui_pw.clone();
        let ui_host_id = ui_host_id.clone();
        let register_result = register_result.clone();

        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let ui_pw = ui_pw.clone();
            let ui_host_id = ui_host_id.clone();
            
            let register_result = register_result.clone();

            let un = &ui_un;
            let pw = &ui_pw;
            let host_id = &ui_host_id;

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
            let hid_val = host_id.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let result = Repo::register_user(&un_val, &pw_val, &hid_val).await;
                let message = format!("{:?}", result);
                log_to_console(&message);

                match result {
                    Ok(_) => {
                        register_result.set("Account created! Please accept the pending data contract at your info account before using this app".to_string());
                    },
                    Err(e) => {
                        register_result.set("Failed to create account".to_string());
                    },
                }
            });
        })
    };

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Register" }</h1></p>
                        <p>{"Please register for an account"}</p>

                        <label for="ip_address">{ "User Name" }</label>
                        <input type="text" class="input" id ="username" placeholder="username" ref={&ui_un}/>

                        <label for="pw">{ "Password" }</label>
                        <input type="text" class="input"  id="pw" placeholder="pw" ref={&ui_pw} />

                        <label for="hid">{ "Host Id" }</label>
                        <input type="text" class="input"  id="hid" placeholder="host id" ref={&ui_host_id} />

                        <div class="buttons">
                        <button type="button" class="button is-primary" id="register" value="Register" {onclick}>
                            <span class="mdi mdi-account-plus">{" Register"}</span>
                        </button>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    }
}
