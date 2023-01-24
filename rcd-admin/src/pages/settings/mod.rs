use rcd_http_common::url::client::GET_SETTINGS;
use rcd_messages::client::{GetSettingsRequest, GetSettingsReply};
use yew::{function_component, Html, use_state_eq, Callback, AttrValue, html};

use crate::{request::{get_token, update_token_login_status, set_status, self}, log::log_to_console};

#[function_component]
pub fn Settings() -> Html {

    let settings = use_state_eq(move || {
        String::from("")
    });

    let onclick = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let settings = settings.clone();
            let token = get_token();
            let request = GetSettingsRequest {
                authentication: Some(token.auth())
            };

            let url = format!("{}{}", token.addr, GET_SETTINGS);
            let body = serde_json::to_string(&request).unwrap();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if let Ok(ref x) = response {
                    log_to_console(x.to_string());

                    let reply: GetSettingsReply = serde_json::from_str(x).unwrap();

                    let is_authenticated = reply.authentication_result.unwrap().is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        settings.set(reply.settings_json);
                    }
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(url, body, cb);
        })
    };

    html!(
        <div>
            <div class="container">
                <div class="box">
                    <h1 class="subtitle">{"Settings"}</h1>
                    <button type="button" class="button is-primary" id="view_settings" value="View" onclick={onclick}>
                        <span class="mdi mdi-magnify">{" View Settings"}</span>
                    </button>
                    <p>
                    <textarea class="textarea" rows="5" cols="60"  
                    id ="settings" placeholder="Settings Will Appear Here As Json" value={(*settings).clone()}/>
                    </p>
                </div>
            </div>
        </div>
    )
}