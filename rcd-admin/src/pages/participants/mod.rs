use yew::{function_component, Html, html};

use crate::pages::participants::view::ViewParticipants;

pub mod view; 
pub mod add;

#[function_component]
pub fn Participants() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                    < ViewParticipants />
                </div>
            </div>
        </div>
    }
}