use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, Callback, Html};

use crate::{
    event::{SharkEvent, SharkEventType},
    event_props::SharkEventProps,
    logging::log_to_console,
    storage::{add_event, get_events},
};

#[function_component]
pub fn EnterEvent(SharkEventProps { events }: &SharkEventProps) -> Html {
    let event_date_ui = use_node_ref();
    let event_type_ui = use_node_ref();
    let event_notes_ui = use_node_ref();
    let events = events.clone();

    let onclick = {
        log_to_console("clicked");
        let event_date_ui = event_date_ui.clone();
        let event_type_ui = event_type_ui.clone();
        let event_notes_ui = event_notes_ui.clone();
        let events = events;
        Callback::from(move |_| {
            let event_date = event_date_ui.cast::<HtmlInputElement>().unwrap().value();
            let event_type = event_type_ui.cast::<HtmlInputElement>().unwrap().value();
            let event_notes = event_notes_ui.cast::<HtmlInputElement>().unwrap().value();
            let events = events.clone();

            let etype = SharkEventType::try_parse(&event_type);

            let event = SharkEvent {
                event_date,
                event_type: etype,
                notes: event_notes,
            };

            add_event(event);
            let x = get_events();
            events.set(x);
        })
    };

    html! {
        <div>
            <h1 class="title">{"SHARK APP"}</h1>

            <label for="event_date">{ "Date" }</label>
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

            <div class="buttons">
                <button class="button is-primary" type="button" id="save_event" value="Save" {onclick}>
                    <span class="mdi mdi-shark-fin">{" Save Event"}</span>
                </button>
            </div>
        </div>
    }
}
