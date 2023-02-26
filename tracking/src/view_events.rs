use yew::{function_component, html, Html};

use crate::event_props::SharkEventProps;

#[function_component]
pub fn ViewEvents(SharkEventProps { events }: &SharkEventProps) -> Html {
    let events = events.clone();

    html!(
        <div>
            <h1 class="subtitle">{"Previous Events"}</h1>
            <div class="table-container">
                    <table class="table is-narrow">
                        <thead>
                            <tr>
                                <th>{"Date"}</th>
                                <th>{"Type"}</th>
                                <th>{"Notes"}</th>
                            </tr>
                        </thead>
                        {
                            (*events).clone().into_iter().map(|e|{
                                let event_date = e.event_date;
                                let event_type = e.event_type.as_string();
                                let event_notes = e.notes;

                                html!{
                                    <tr>
                                        <td>{event_date}</td>
                                        <td>{event_type}</td>
                                        <td>{event_notes}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </table>
                    </div>
        </div>
    )
}
