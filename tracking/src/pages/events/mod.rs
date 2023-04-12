use yew::prelude::*;

use crate::pages::events::{
    add::{AddAssociatedEvent, AddMainEvent},
    stats::Stats,
    view::ViewEvents,
};

pub mod add;
pub mod stats;
pub mod view;

#[derive(Debug, Clone, PartialEq, Copy)]
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

    let onclick_add_main = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            let page_state = page_state.clone();
            page_state.set(CurrentEventPage::AddMainEvent)
        })
    };

    let onclick_add_associated = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            let page_state = page_state.clone();
            page_state.set(CurrentEventPage::AddAssociatedEvent)
        })
    };

    let onclick_view = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            let page_state = page_state.clone();
            page_state.set(CurrentEventPage::ViewEvents)
        })
    };

    let onclick_stats = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            let page_state = page_state.clone();
            page_state.set(CurrentEventPage::Stats)
        })
    };

    let onclick_default = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            let page_state = page_state.clone();
            page_state.set(CurrentEventPage::Default)
        })
    };

    html! {
        <div>
            <div class="container">
                <div class="box">

                    <div class="has-text-centered">
                        <p><h1 class="title is-1" onclick={onclick_default}>{ "Events" }</h1></p>

                        <aside class="menu">
                             <p class="menu-label">
                              {"Add Events"}
                             </p>
                             <ul class="menu-list">
                                 <li onclick={onclick_add_main}><a>{"Main Events"}</a></li>
                                 <li onclick={onclick_add_associated}><a>{"Associated Events"}</a></li>
                             </ul>
                             <p class="menu-label">
                              {"View Events"}
                             </p>
                             <ul class="menu-list">
                                 <li onclick={onclick_view}><a>{"Events"}</a></li>
                             </ul>
                             <p class="menu-label">
                             {"Stats"}
                            </p>
                            <ul class="menu-list">
                                 <li onclick={onclick_stats}><a>{"View Statistics"}</a></li>
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
