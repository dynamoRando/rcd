use web_sys::HtmlInputElement;
use yew::{prelude::*, platform::spawn_local};

use crate::{repo::Repo, logging::log_to_console};


#[function_component]
pub fn AddMainEvent() -> Html { 

    let event_date = use_node_ref();
    let event_date_notes = use_node_ref();
    let add_event_result = use_state_eq(|| "".to_string());


    html!(
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"Main Event"} </h1></p>
                    <p>{"This is the start of your period."}</p>
                    <p><label for="main_event">{"Date" }</label>
                    <input type="date" id="main_event" name="main_event" /></p>

                    <div class="buttons">
                            <button type="button" class="button is-primary" id="login" value="Login">
                                <span class="mdi mdi-calendar">{" Add"}</span>
                            </button>
                            
                        </div>
                </div>
            </div>
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