use crate::{enter_event::EnterEvent, event::SharkEvent, view_events::ViewEvents};
use yew::prelude::*;

pub mod enter_event;
pub mod event;
pub mod event_props;
pub mod logging;
pub mod storage;
pub mod view_events;

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
