use yew::prelude::*;


#[function_component]
pub fn AddMainEvent() -> Html { 

    // before we save the event, we're going to need to look up the user id if we don't have it already

    html!(
        <div>
            <p>{"Main Event Add Placeholder"}</p>
        </div>
    )
}

#[function_component]
pub fn AddAssociatedEvent() -> Html { 
    html!(
        <div>
            <p>{"Associated Event Add Placeholder"}</p>
        </div>
    )
}