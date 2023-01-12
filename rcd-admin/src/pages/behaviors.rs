use yew::{function_component, Html, html};

#[function_component]
pub fn Behaviors() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"Behaviors"} </h1></p>
                    <p>{"We can configure what we want to do if a host sends us update or delete
                    requests, such as ignoring them, logging them for later review, and so on."}</p>
                    <p>{"We can also configure what we want to do if we change or delete data, 
                    if we should notify the host or not."}</p>
                </div>
            </div>
        </div>
    }
}