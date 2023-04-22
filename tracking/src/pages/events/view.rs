use tracking_model::event::{SharkAssociatedEvent, SharkEvent};
use yew::{platform::spawn_local, prelude::*};

use crate::{logging::log_to_console, repo::Repo, storage::save_events};

#[function_component]
pub fn ViewEvents() -> Html {
    let events = use_state_eq(move || {
        let x: Vec<SharkEvent> = Vec::new();
        x
    });

    let get_event_result = use_state_eq(|| String::from(""));

    let onclick = {
        let events = events.clone();
        let get_event_result = get_event_result.clone();

        Callback::from(move |_| {
            let events = events.clone();
            let get_event_result = get_event_result.clone();

            spawn_local(async move {
                let events = events.clone();
                let get_event_result = get_event_result.clone();

                let result = Repo::get_events().await;
                let message = format!("get event response: {:?}", result);
                log_to_console(&message);

                match result {
                    Ok(repo_events) => {
                        save_events(&repo_events);
                        log_to_console("events saved");
                        events.set(repo_events);
                    }
                    Err(_) => {
                        get_event_result.set("Failed!".to_string());
                    }
                }
            });
        })
    };

    let onclick_mock = {
        let events = events.clone();
        let get_event_result = get_event_result.clone();

        Callback::from(move |_| {
            let events = events.clone();
            let get_event_result = get_event_result.clone();

            spawn_local(async move {
                let events = events.clone();
                let get_event_result = get_event_result.clone();

                let result = Repo::get_events_mock().await;
                let message = format!("get event response: {:?}", result);
                log_to_console(&message);

                match result {
                    Ok(repo_events) => {
                        events.set(repo_events);
                    }
                    Err(_) => {
                        get_event_result.set("Failed!".to_string());
                    }
                }
            });
        })
    };

    html!(
        <div>
            <div class="container">
                <div class="box">
                    <p><h1 class="subtitle"> {"View Events"} </h1></p>
                    <div class="buttons">
                            <button type="button" class="button is-primary" id="Add" value="View" {onclick}>
                                <span class="mdi mdi-calendar-account">{" View"}</span>
                            </button>
                            <button type="button" class="button is-primary" id="Add" value="View" onclick={&onclick_mock}>
                            <span class="mdi mdi-calendar-account">{" View Mock"}</span>
                        </button>
                    </div>
                    <p>{"Get Events Result: "}{(*get_event_result).clone()}</p>

                    <div class="table-container">
                        <table class="table is-narrow">
                            <thead>
                                <tr>
                                    <th>{"Main Event Date"}</th>
                                    <th>{"Associated Event Date"}</th>
                                    <th>{"Associated Event Type"}</th>
                                    <th>{"Notes"}</th>
                                    <th>{"Delete"}</th>
                                </tr>
                            </thead>
                            {
                                (*events).clone().into_iter().map(|e|{
                                    let event = e.clone();
                                    let main_event_date = event.date.clone();
                                    let main_event_notes = event.notes.clone();
                                    let associated_events = event.associated_events.clone();

                                    html!{
                                        <>
                                        <tr>
                                            <td>{main_event_date}</td>
                                            <td></td>
                                            <td></td>
                                            <td>{main_event_notes}</td>
                                            <td><button class="button is-danger" onclick=
                                            {
                                                let event = event.clone();
                                                move |_| {
                                                let event = event.clone();
                                                spawn_local(async move {
                                                    let event = event.clone();
                                                    let delete_result = Repo::delete_event(event.id as usize).await;
                                                    match delete_result {
                                                        Ok(is_deleted) => {
                                                            if is_deleted {
                                                                log_to_console("event deleted");
                                                            } else
                                                            {
                                                                log_to_console("event NOT deleted");
                                                            }},
                                                        Err(e) => log_to_console(&e.to_string())
                                                        }
                                                });
                                            }}>{"Delete"}</button></td>
                                        </tr>
                                        {get_associated_events_html(associated_events)}
                                        </>
                                    }
                                }).collect::<Html>()
                            }
                        </table>
                    </div>
                </div>
            </div>
        </div>
    )
}

fn get_associated_events_html(associated_events: Option<Vec<SharkAssociatedEvent>>) -> Html {
    match associated_events {
        None => {
            return html!(
                <>
                </>
            )
        }
        Some(events) => {
            return events
                .into_iter()
                .map(|ae| {
                    let date = ae.date.clone();
                    let event_type = ae.event_type.as_string().to_string();
                    let notes_option = ae.notes.clone();

                    let notes = match notes_option {
                        None => "".to_string(),
                        Some(notes) => notes,
                    };

                    html! {
                        <tr>
                            <td></td>
                            <td>{date}</td>
                            <td>{event_type}</td>
                            <td>{notes}</td>
                            <td><button class="button is-danger"
                            onclick=
                                            {
                                                let event = ae.clone();
                                                move |_| {
                                                let event = event.clone();
                                                spawn_local(async move {
                                                    let event = event.clone();
                                                    let delete_result = Repo::delete_associated_event(event.uuid.as_ref().unwrap()).await;
                                                    match delete_result {
                                                        Ok(is_deleted) => {
                                                            if is_deleted {
                                                                log_to_console("event deleted");
                                                            } else
                                                            {
                                                                log_to_console("event NOT deleted");
                                                            }},
                                                        Err(e) => log_to_console(&e.to_string())
                                                        }
                                                });
                                            }}>{"Delete"}</button></td>
                        </tr>
                    }
                })
                .collect::<Html>()
        }
    }
}
