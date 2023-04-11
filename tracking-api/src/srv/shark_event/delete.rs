use rocket::{delete, http::Status, serde::json::Json, State};
use tracking_model::event::SharkEvent;

#[delete("/events/delete")]
pub async fn delete_event() -> (Status, Json<SharkEvent>) {
    todo!()
}

#[delete("/events/delete/associated")]
pub async fn delete_associated_event() -> (Status, Json<SharkEvent>) {
    todo!()
}
