use tracking_model::event::SharkEvent;
use web_sys::HtmlInputElement;
use yew::{prelude::*, platform::spawn_local};

use crate::{repo::Repo, logging::log_to_console, storage::get_uid};


#[function_component]
pub fn AddMainEvent() -> Html { 

    let event_date = use_node_ref();
    let event_date_notes = use_node_ref();
    let add_event_result = use_state_eq(|| "".to_string());

    let onclick = {
        let event_date = event_date.clone();
        let event_date_notes = event_date_notes.clone();
        let add_event_result = add_event_result.clone();

        Callback::from(move |_| {
            let event_date = event_date.clone();
            let event_date_notes = event_date_notes.clone();
            let add_event_result = add_event_result.clone();

            let event_date_val = event_date.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let add_event_result = add_event_result.clone();

                let event = SharkEvent {
                    id: 0,
                    date: event_date_val,
                    notes: None,
                    associated_events: None,
                    user_id: Some(get_uid()),
                };

                let result = Repo::add_event(&event).await;
                let message = format!("add event response: {:?}", result);
                log_to_console(&message);

                match result {
                    Ok(is_added) => {
                        if is_added {
                            add_event_result.set("Is added.".to_string());
                        } else {
                            add_event_result.set("Failed!".to_string());
                        }
                    }
                    Err(_) => {
                        add_event_result.set("Failed!".to_string());
                    }
                }
            });
        })
    };

    html!(
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"Main Event"} </h1></p>
                    <p>{"This is the start of your period."}</p>
                    <p><label for="main_event">{"Date" }</label>
                    <input type="date" id="main_event" name="main_event" ref={event_date} /></p>

                    <div class="buttons">
                            <button type="button" class="button is-primary" id="Add" value="Add" {onclick}>
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