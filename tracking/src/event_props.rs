use yew::{UseStateHandle, Properties};

use crate::event::SharkEvent;

#[derive(Properties, PartialEq)]
pub struct SharkEventProps{
    pub events: UseStateHandle<Vec<SharkEvent>>,
}