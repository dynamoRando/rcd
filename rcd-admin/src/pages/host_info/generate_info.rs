use rcd_http_common::url::client::GENERATE_HOST_INFO;
use rcd_messages::client::{GenerateHostInfoReply, GenerateHostInfoRequest};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, AttrValue, Callback, Html};

use crate::{
    log::log_to_console,
    request::{self, clear_status, get_token, set_status, update_token_login_status},
};

#[function_component]
pub fn GenerateInfo() -> Html {
    let last_gen_result = use_state_eq(move || String::from(""));

    let ui_host_name = use_node_ref();

    let generate_onclick = {
        let ui_host_name = ui_host_name.clone();
        let last_gen_result = last_gen_result.clone();

        Callback::from(move |_| {
            let last_gen_result = last_gen_result.clone();
            let host_name = ui_host_name.cast::<HtmlInputElement>().unwrap().value();

            let token = get_token();

            let request = GenerateHostInfoRequest {
                authentication: Some(token.auth()),
                host_name: host_name.clone(),
            };

            let json_request = serde_json::to_string(&request).unwrap();
            let url = format!("{}{}", token.addr, GENERATE_HOST_INFO);

            let cb = {
                let last_gen_result = last_gen_result.clone();
                Callback::from(move |response: Result<AttrValue, String>| {
                    if response.is_ok() {
                        let response = response.unwrap();
                        log_to_console(response.to_string());
                        clear_status();

                        let reply: GenerateHostInfoReply =
                            serde_json::from_str(&response.to_string()).unwrap();

                        let is_authenticated = reply
                            .authentication_result
                            .as_ref()
                            .unwrap()
                            .is_authenticated;
                        update_token_login_status(is_authenticated);

                        if is_authenticated {
                            let message = format!(
                                "{}{}",
                                "Last gen result was: ",
                                reply.is_successful.to_string()
                            );
                            last_gen_result.set(message);
                        }
                    } else {
                        set_status(response.err().unwrap());
                    }
                })
            };

            request::post(url, json_request, cb);
        })
    };

    html!(
        <div>
          <p><h1 class="subtitle"> {"Generate Host Info"} </h1></p>

                    <p> <label for="host_name">{ "Enter Host Name" }</label>
                    <input type="text" class="input"  id="host_name" placeholder="Enter Host Name" ref={&ui_host_name} /></p>

                    <p>
                    <button class="button is-primary" onclick={generate_onclick}>
                        <span class="mdi mdi-autorenew">{" Generate Host Info"}</span>
                    </button>
                    </p>

                    <p><h3 class="subtitle"> {"Last Generate Result"} </h3></p>
                    <p>{(*last_gen_result).clone()}</p>
        </div>
    )
}
