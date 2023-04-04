use tracking_model::event::SharkEvent;
use yew::{Properties, UseStateHandle};


#[derive(Properties, PartialEq)]
pub struct SharkEventProps {
    pub events: UseStateHandle<Vec<SharkEvent>>,
}
