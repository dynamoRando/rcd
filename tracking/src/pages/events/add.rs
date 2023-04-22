use tracking_model::event::{EventType, SharkAssociatedEvent, SharkEvent};
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    logging::log_to_console,
    repo::Repo,
    storage::{get_last_x_events, get_uid},
};

#[function_component]
pub fn AddMainEvent() -> Html {
    let ui_event_date = use_node_ref();
    let ui_event_date_notes = use_node_ref();

    let add_event_result = use_state_eq(|| "".to_string());

    let onclick = {
        let ui_event_date = ui_event_date.clone();
        let ui_event_date_notes = ui_event_date_notes.clone();

        let add_event_result = add_event_result.clone();

        Callback::from(move |_| {
            let event_date = ui_event_date.clone();
            let event_date_notes = ui_event_date_notes.clone();

            let add_event_result = add_event_result.clone();

            let event_date_val = event_date.cast::<HtmlInputElement>().unwrap().value();
            let event_date_notes = event_date_notes.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let add_event_result = add_event_result.clone();

                let event = SharkEvent {
                    id: 0,
                    date: event_date_val,
                    notes: Some(event_date_notes),
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
                    <p><label for="main_event">{"Date " }</label>
                    <input type="date" id="main_event" name="main_event" ref={ui_event_date} /></p>

                    <p><label for="notes">{ "Enter Any Notes" }</label></p>
                    <p>
                        <textarea class="textarea" rows="5" cols="60"  id ="notes" placeholder="Enter any notes here" ref={&ui_event_date_notes}/>
                    </p>


                    <div class="buttons">
                            <button type="button" class="button is-primary" id="Add" value="Add" {onclick}>
                                <span class="mdi mdi-calendar">{" Add"}</span>
                            </button>
                    </div>
                    <p>{"Add Event Result: "}{(*add_event_result).clone()}</p>
                </div>
            </div>
        </div>
    )
}

#[function_component]
pub fn AddAssociatedEvent() -> Html {
    let ui_associated_event_date = use_node_ref();
    let ui_associated_event_date_notes = use_node_ref();
    let ui_event_type = use_node_ref();

    let add_event_result = use_state_eq(|| "".to_string());

    let previous_events = get_last_x_events(3);
    log_to_console(&previous_events.len().to_string());

    let ui_previous_events = use_node_ref();

    let selected_main_event = use_state_eq(|| "".to_string());
    let selected_event_type = use_state_eq(|| "".to_string());

    let onclick = {
        let selected_main_event = selected_main_event.clone();
        let selected_event_type = selected_event_type.clone();

        let ui_associated_event_date = ui_associated_event_date.clone();
        let ui_associated_event_date_notes = ui_associated_event_date_notes.clone();
        let add_event_result = add_event_result.clone();

        Callback::from(move |_| {
            let selected_main_event = selected_main_event.clone();
            let selected_event_type = selected_event_type.clone();
            let add_event_result = add_event_result.clone();

            let ui_associated_event_date = ui_associated_event_date.clone();
            let ui_associated_event_date_notes = ui_associated_event_date_notes.clone();

            if (*selected_main_event).len() > 0 {
                let selected_main_event_date = (*selected_main_event).clone();

                let previous_events = get_last_x_events(3);
                let main_event = previous_events
                    .iter()
                    .find(|e| e.date == selected_main_event_date);

                if let Some(main_event) = main_event {
                    log_to_console("main event found");
                    let main_event = main_event.clone();
                    let associated_event_date_val = ui_associated_event_date
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value();
                    let associated_event_date_notes_val = ui_associated_event_date_notes
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value();
                    let event_type = (*selected_event_type).clone();
                    let event_type = EventType::try_parse_from_string(&event_type);

                    let associated_event = SharkAssociatedEvent {
                        event_id: main_event.id,
                        event_type: event_type,
                        date: associated_event_date_val,
                        notes: Some(associated_event_date_notes_val),
                        user_id: Some(get_uid()),
                        uuid: None,
                    };

                    spawn_local(async move {
                        let add_event_result = add_event_result.clone();
                        let associated_event = associated_event.clone();
                        let add_event_result = add_event_result.clone();
                        let result = Repo::add_associated_event(&associated_event).await;

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
                    })
                }
            }
        })
    };

    let onchange_main_event = {
        log_to_console("onchange_main_event");

        let selected_main_event = selected_main_event.clone();
        let ui_selected_main_event = ui_previous_events.clone();

        Callback::from(move |_| {
            let ui_selected_main_event = ui_selected_main_event.clone();
            let selected_main_event = selected_main_event.clone();

            let selected_main_event_val = ui_selected_main_event.cast::<HtmlInputElement>();

            if selected_main_event_val.is_some() {
                let selected_main_event_val = ui_selected_main_event
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();

                log_to_console(&selected_main_event_val);

                selected_main_event.set(selected_main_event_val);
            }
        })
    };

    let onchange_event_type = {
        log_to_console("onchange_event_type");

        let selected_event_type = selected_event_type.clone();
        let ui_event_type = ui_event_type.clone();

        Callback::from(move |_| {
            let selected_event_type = selected_event_type.clone();
            let ui_event_type = ui_event_type.clone();

            let selected_event_type_val = ui_event_type.cast::<HtmlInputElement>();

            if selected_event_type_val.is_some() {
                let selected_event_type_val =
                    ui_event_type.cast::<HtmlInputElement>().unwrap().value();
                log_to_console(&selected_event_type_val);
                selected_event_type.set(selected_event_type_val);
            }
        })
    };

    html!(
        <div>
            <p><h1 class="subtitle"> {"Associated Event"} </h1></p>
            <p>{"This is for logging a related entry to your period.
            Please select the date of the start of your period. If it's not in the list,
            please go to View Events and click View, which will reload the latest 3 events."}</p>

            <p>{" Period Start Date:" }</p>
            <div class="select is-multiple">
                    <select
                        name="select_previous_event"
                        id="select_previous_event"
                        ref={&ui_previous_events}
                        onchange={&onchange_main_event}
                    >
                    <option value="SELECT EVENT">{"SELECT EVENT"}</option>
                    {
                        previous_events.clone().into_iter().map(|event| {
                            html!{
                            <option value={event.date.clone()}>{event.date.clone()}</option>}
                        }).collect::<Html>()
                    }
                    </select>
            </div>
            <p><label for="main_event">{" Today's Date " }</label>
            <input type="date" id="main_event" name="main_event" ref={ui_associated_event_date} /></p>

            <p><label for="notes">{ "Enter Any Notes" }</label></p>
            <p>
                <textarea class="textarea" rows="5" cols="60"  id ="notes" placeholder="Enter any notes here" ref={&ui_associated_event_date_notes}/>
            </p>

            <p>{" Event Type:" }</p>
            <div class="select is-multiple">
                    <select
                        name="select_event_type"
                        id="select_event_type"
                        ref={&ui_event_type}
                        onchange={&onchange_event_type}
                    >
                        <option value="SELECT EVENT">{"SELECT EVENT TYPE"}</option>
                        <option value={"Spotting"}>{"Spotting"}</option>
                        <option value={"Other"}>{"Other"}</option>
                        <option value={"End"}>{"End Of Period"}</option>
                    </select>
            </div>

            <div class="buttons">
                    <button type="button" class="button is-primary" id="Add" value="Add" {onclick}>
                        <span class="mdi mdi-calendar">{" Add"}</span>
                    </button>
            </div>
            <p>{"Add Event Result: "}{(*add_event_result).clone()}</p>
        </div>
    )
}
