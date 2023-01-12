use yew::{function_component, Html, html};

#[function_component]
pub fn CooperativeHosts() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"View Cooperating Hosts"} </h1></p>
                    <p>{"This would normally show us a list of hosts that we're cooperating with, but 
                    we don't have a service call for that yet. We can change the host status for 
                    hosts that we're already cooperating with though, to deny or authorize them."}</p>
                </div>
            </div>
        </div>
    }
}