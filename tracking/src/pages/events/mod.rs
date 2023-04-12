use yew::prelude::*;

use crate::pages::events::{
    add::{AddAssociatedEvent, AddMainEvent},
    view::ViewEvents, stats::Stats,
};

pub mod add;
pub mod stats;
pub mod view;

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentEventPage {
    Default,
    AddMainEvent,
    AddAssociatedEvent,
    ViewEvents,
    Stats,
}

#[function_component]
pub fn Events() -> Html {
    // let events = Repo::get_events().await;

    let page_state = use_state_eq(|| CurrentEventPage::Default);

    match *page_state {
        CurrentEventPage::Default => html! {<div></div>},
        CurrentEventPage::AddMainEvent => html! {<div><AddMainEvent/></div>},
        CurrentEventPage::AddAssociatedEvent => html! {<div><AddAssociatedEvent /></div>},
        CurrentEventPage::ViewEvents => html! {<div><ViewEvents /></div>},
        CurrentEventPage::Stats => html! {<div><Stats /></div>},
    };

    html! {
        <div>
            <div class="container">
                <div class="box">

                    <div class="has-text-centered">
                        <p><h1 class="title is-1">{ "Events" }</h1></p>

                        <aside class="menu">
                             <p class="menu-label">
                              {"Add Events"}
                             </p>
                             <ul class="menu-list">
                                 <li><a>{"Main Events"}</a></li>
                                 <li><a>{"Associated Events"}</a></li>
                             </ul>
                             <p class="menu-label">
                              {"View Events"}
                             </p>
                             <ul class="menu-list">
                                 <li><a>{"Events"}</a></li>
                             </ul>
                             <p class="menu-label">
                             {"Stats"}
                            </p>
                            <ul class="menu-list">
                                 <li><a>{"View Statistics"}</a></li>
                             </ul>
                        </aside>
                    </div>


                <div>
                {
                    match *page_state {
                        CurrentEventPage::Default => html!{<div></div>},
                        CurrentEventPage::AddMainEvent => html!{<div><AddMainEvent/></div>},
                        CurrentEventPage::AddAssociatedEvent => html!{<div><AddAssociatedEvent /></div>},
                        CurrentEventPage::ViewEvents => html!{<div><ViewEvents /></div>},
                        CurrentEventPage::Stats => html!{<div><Stats /></div>},
                    }
                }
                </div>

                </div>
            </div>
        </div>
    }
}
