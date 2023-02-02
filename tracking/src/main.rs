use yew::prelude::*;
use crate::{enter_event::EnterEvent, view_events::ViewEvents, event_props::SharkEventProps, event::SharkEvent};

pub mod enter_event;
pub mod event;
pub mod storage;
pub mod view_events;
pub mod logging;
pub mod event_props;

#[function_component]
fn App() -> Html {

    let app_state = use_state_eq(move || {
        let x: Vec<SharkEvent> = Vec::new();
        x
    });

    html!(
        <div>
            < EnterEvent events={app_state.clone()} />
            < ViewEvents events={app_state.clone()}/>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
