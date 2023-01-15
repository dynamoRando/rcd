use rcd_http_common::url::client::GET_COOP_HOSTS;
use rcd_messages::client::{GetCooperativeHostsReply, GetCooperativeHostsRequest, HostInfoStatus};
use web_sys::console;
use yew::{function_component, html, use_state_eq, AttrValue, Callback, Html};

use crate::request::{clear_status, get_token, set_status, update_token_login_status, self};

#[function_component]
pub fn CooperativeHosts() -> Html {
    let hosts_state = use_state_eq(move || {
        let x: Vec<HostInfoStatus> = Vec::new();
        return x;
    });

    let onclick = {
        let hosts_state = hosts_state.clone();
        Callback::from(move |_| {
            let token = get_token();
            let url = format!("{}{}", token.addr, GET_COOP_HOSTS);

            let request = GetCooperativeHostsRequest {
                authentication: Some(token.auth()),
            };

            let json_request = serde_json::to_string(&request).unwrap();

            let hosts_state = hosts_state.clone();

            let cb = Callback::from(move |response: Result<AttrValue, String>| {
                if response.is_ok() {
                    let response = response.unwrap();
                    console::log_1(&response.to_string().into());
                    clear_status();

                    let coop_response: GetCooperativeHostsReply =
                        serde_json::from_str(&response).unwrap();

                    let is_authenticated = coop_response
                        .authentication_result
                        .as_ref()
                        .unwrap()
                        .is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        let hosts = coop_response.hosts.clone();
                        hosts_state.set(hosts);
                    }
                } else {
                    set_status(response.err().unwrap());
                }
            });

            request::post(url, json_request, cb)
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"View Cooperating Hosts"} </h1></p>
                    <p>{"This would normally show us a list of hosts that we're cooperating with, but
                    we don't have a service call for that yet. We can change the host status for 
                    hosts that we're already cooperating with though, to deny or authorize them."}</p>
                    <p><button class="button is-primary" {onclick}><span class="mdi mdi-handshake">{" View Hosts"}</span></button></p>
                </div>
                <div class="table-container">
                    <table class="table is-narrow">
                        <thead>
                            <tr>
                                <th>{"Host Id"}</th>
                                <th>{"Host Name"}</th>
                                <th>{"IP4"}</th>
                                <th>{"IP6"}</th>
                                <th>{"DB Port"}</th>
                                <th>{"Last Communication UTC"}</th>
                                <th>{"Status"}</th>
                                <th>{"HTTP Addr"}</th>
                                <th>{"HTTP Port"}</th>
                            </tr>
                        </thead>
                        {
                            (*hosts_state).clone().into_iter().map(|h|{
                        
                                html!{
                                    <tr>
                                        <td>{""}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </table>
                    </div>
            </div>
        </div>
    }
}
