use regex::Regex;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, use_state_eq, Callback, Html};

use crate::{
    event::{SharkEvent, SharkEventType},
    event_props::SharkEventProps,
    storage::{add_event, get_events},
};

const DATE_FORMAT: &str = r"^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$";

#[function_component]
pub fn EnterEvent(SharkEventProps { events }: &SharkEventProps) -> Html {
    let event_date_ui = use_node_ref();
    let event_type_ui = use_node_ref();
    let event_notes_ui = use_node_ref();
    let events = events.clone();

    let error_messages = use_state_eq(|| String::from(""));
    let form_is_invalid = use_state_eq(move || false);

    let onclick = {
        let error_messages = error_messages.clone();
        let event_date_ui = event_date_ui.clone();
        let event_type_ui = event_type_ui.clone();
        let event_notes_ui = event_notes_ui.clone();
        let events = events;
        let form_is_invalid = form_is_invalid.clone();

        Callback::from(move |_| {
            let error_messages = error_messages.clone();
            let event_date = event_date_ui.cast::<HtmlInputElement>().unwrap().value();
            let event_type = event_type_ui.cast::<HtmlInputElement>().unwrap().value();
            let event_notes = event_notes_ui.cast::<HtmlInputElement>().unwrap().value();
            let events = events.clone();

            let etype = SharkEventType::try_parse(&event_type);

            let re = Regex::new(DATE_FORMAT).unwrap();

            if !re.is_match(&event_date) {
                let em = format!("{}{}", event_date, " is not a date");
                error_messages.set(em);
                form_is_invalid.set(true);
            } else {
                form_is_invalid.set(false);
                let event = SharkEvent {
                    event_date,
                    event_type: etype,
                    notes: event_notes,
                };

                add_event(event);
                let x = get_events();
                events.set(x);
                error_messages.set(String::from(""));
            }
        })
    };

    html! {
        <div>
            <h1 class="title">{"SHARK APP"}</h1>

            <label for="event_date">{ "Date (Please enter in YYYY-MM-DD format)" }</label>
            <input type="text" class="input" id ="event_date" placeholder="Please Enter A Date"
            ref={&event_date_ui}/>

            <div class="select is-multiple">
                    <select name="event_type" id="event_type" ref={&event_type_ui}>
                        <option value="Spotting">{"Spotting"}</option>
                        <option value="StartPeriod">{"Period Start"}</option>
                        <option value="EndPeriod">{"Period End"}</option>
                    </select>
            </div>

            <p><label for="notes">{ "Notes" }</label></p>
            <p>
                <textarea class="textarea" rows="5" cols="60"  id ="notes"
                placeholder="Enter Any Additional Notes"
                ref={&event_notes_ui}/>
            </p>

            {
                if *form_is_invalid {
                    html!(
                        <div>
                            <div class="notification is-warning">
                                <p><label for="errors">{ "Errors:" }</label></p>
                                <p><label for="errors">{ (*error_messages).clone() }</label></p>
                            </div>
                        </div>
                    )
                } else {
                    html!(<div></div>)
                }
            }

            <div class="buttons">
                <button class="button is-primary" type="button" id="save_event" value="Save" {onclick}>
                    <span class="mdi mdi-shark-fin">{" Save Event"}</span>
                </button>
            </div>
        </div>
    }
}
