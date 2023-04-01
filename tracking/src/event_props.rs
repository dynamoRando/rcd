use yew::{Properties, UseStateHandle};

use crate::event::SharkEvent;


#[derive(Properties, PartialEq)]
pub struct SharkEventProps {
    pub events: UseStateHandle<Vec<SharkEvent>>,
}
