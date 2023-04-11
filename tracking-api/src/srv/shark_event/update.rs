use rocket::{http::Status, put, serde::json::Json, State};
use tracking_model::event::SharkEvent;

#[put("/events/update")]
pub async fn update_event() -> (Status, Json<SharkEvent>) {
    todo!()
}

#[put("/events/update/associated")]
pub async fn update_associated_event() -> (Status, Json<SharkEvent>) {
    todo!()
}
