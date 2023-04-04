use rocket::{http::Status, serde::json::Json, State, delete};
use crate::shark::{SharkAssociatedEvent, SharkEvent};

#[delete("/events/delete")]
pub async fn delete_event() -> (Status, Json<SharkEvent>) { 
    todo!()
}


#[delete("/events/delete/associated")]
pub async fn delete_associated_event() -> (Status, Json<SharkEvent>) { 
    todo!()
}
