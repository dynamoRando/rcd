use rcd_http_common::url::client::VIEW_PENDING_CONTRACTS;
use rcd_messages::client::{ViewPendingContractsRequest, ViewPendingContractsReply, Contract};
use yew::{function_component, Html, html, Callback, AttrValue, use_state_eq};

use crate::{request::{get_token, update_token_login_status, self}, log::log_to_console};

#[function_component]
pub fn Pending() -> Html {

    let pending_contracts = use_state_eq(move || {
        let x: Vec<Contract> = Vec::new();
        return x;
    });

    let onclick = {
        let pending_contracts = pending_contracts.clone();
        Callback::from(move |_| {
            let token = get_token();
            let pending_contracts = pending_contracts.clone();

            let request = ViewPendingContractsRequest {
                authentication: Some(token.auth().clone())
            };

            let request_json = serde_json::to_string(&request).unwrap();
            let url = format!("{}{}", token.addr, VIEW_PENDING_CONTRACTS);
            let cb =
                Callback::from(move |response: AttrValue| {
                    log_to_console(response.to_string());

                    let reply: ViewPendingContractsReply = serde_json::from_str(&response.to_string()).unwrap();
                    let is_authenticated = reply.authentication_result.as_ref().unwrap().is_authenticated;
                    update_token_login_status(is_authenticated);

                    if is_authenticated {
                        let contracts = reply.contracts.clone();
                        pending_contracts.set(contracts);
                    }
                });

            request::get_data(url, request_json, cb);
        })
    };

    html! {
        <div>
            <h1 class="subtitle">{"View Pending Contracts"}</h1>
            <p>
                <button class="button is-primary" {onclick}>
                    <span class="mdi mdi-timer">{" View Pending Contracts"}</span>
                </button>
            </p>
            <p><h2 class="subtitle">{"Pending Contracts"}</h2></p>
            <div class="table-container">
            <table class="table is-narrow">
                <thead>
                    <tr>
                        <th>{"Host Name"}</th>
                        <th>{"Database Name"}</th>
                        <th>{"Description"}</th>
                        <th>{"Accept/Reject?"}</th>
                    </tr>
                </thead>
                {
                    (*pending_contracts).clone().into_iter().map(|c|{
                        let host_name = c.host_info.unwrap().host_name.clone();
                        let database_name = c.schema.unwrap().database_name.clone();
                        let description = c.description.clone();

                        html!{
                            <tr>
                                <td>{host_name}</td>
                                <td>{database_name}</td>
                                <td>{description}</td>
                                <td>{"placeholder for button"}</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </table>
            </div>
        </div>
    }
}