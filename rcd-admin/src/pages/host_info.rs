use yew::{function_component, Html, html};

#[function_component]
pub fn HostInfo() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"View Host Info"} </h1></p>
                    <p>{"This would normally show host information, but we don't have a service call for it yet. We can generate host info though, if we want, which requires us
                    to send a host name"}</p>
                </div>
            </div>
        </div>
    }
}