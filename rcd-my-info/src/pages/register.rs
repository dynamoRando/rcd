use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    log::log_to_console,
    request::proxy::{set_proxy, RcdProxy},
};

#[function_component]
pub fn Register() -> Html {
    let ui_un = use_node_ref();
    let ui_pw = use_node_ref();
    let ui_addr_port = use_node_ref();
    let api_location = use_state_eq(|| "".to_string());

    let register_result = use_state_eq(move || String::from(""));

    let onclick = {
        let ui_un = ui_un.clone();
        let ui_pw = ui_pw.clone();
        let api_location = api_location.clone();
        let register_result = register_result.clone();
    

        Callback::from(move |_| {
            let ui_un = ui_un.clone();
            let ui_pw = ui_pw.clone();
            let api_location = api_location.clone();
            let register_result = register_result.clone();

            let un = &ui_un;
            let pw = &ui_pw;
            

            let un_val = un.cast::<HtmlInputElement>().unwrap().value();
            let pw_val = pw.cast::<HtmlInputElement>().unwrap().value();
            let addr_port = &(*api_location);

            let mut proxy = RcdProxy::new(&addr_port);
            set_proxy(&proxy);

            let u = un_val;
            let p = pw_val;

            spawn_local(async move {
                let result = proxy.register_account(&u, &p).await;
                log_to_console("{result:?}");
                match result {
                    Ok(registration) => {
                        if registration.is_successful {
                            let result_message = format!(
                                "result: {} host_id: {}",
                                registration.is_successful,
                                registration.host_id.as_ref().unwrap().clone()
                            );
                            register_result.set(result_message);
                        } else {
                            let result_message = format!(
                                "result: {} message: {}",
                                registration.is_successful,
                                registration.error.as_ref().unwrap().to_string(),
                            );
                            register_result.set(result_message);
                        }
                    }
                    Err(e) => log_to_console(&e),
                };
            })
        })
    };

    let onchange = {
        log_to_console("onchange");

        let ui_api = ui_addr_port.clone();
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
        <div>
            <div class="container">
                <div class="box">
                    <div class="has-text-centered">
                        <h1 class="subtitle"> {"Register For Account"} </h1>
                        <p>
                            <label for="ip_address">{ "API " }</label>
                            // <input type="text" class="input" id ="addr_port" placeholder="127.0.0.1:50040" ref={&ui_addr_port}/>
                            <div class="select is-multiple">
                                    <select
                                        name="select_api"
                                        id="select_api"
                                        ref={&ui_addr_port}
                                        onchange={&onchange}
                                    >
                                        <option value="SELECT EVENT">{"SELECT API LOCATION"}</option>
                                        <option value={"localhost:50040"}>{"localhost"}</option>
                                        <option value={"proxy.home:50040"}>{"proxy"}</option>
                                </select>
                            </div>
                        </p>

                        <label for="ip_address">{ "User Name" }</label>
                        <input type="text" class="input" id ="username" placeholder="username" ref={&ui_un}/>

                        <label for="port">{ "Password" }</label>
                        <input type="text" class="input"  id="pw" placeholder="pw" ref={&ui_pw} />

                        <div class="buttons">
                        <button type="button" class="button is-primary" id="register" value="Register" {onclick}>
                            <span class="mdi mdi-account-plus">{" Register"}</span>
                        </button>
                        </div>
                        <h2 class="subtitle"> {"Register Result"} </h2>
                        <p>{(*register_result).clone()}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
