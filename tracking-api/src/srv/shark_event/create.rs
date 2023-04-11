use rocket::{http::Status, post, serde::json::Json, State};
use tracking_model::event::{SharkAssociatedEvent, SharkEvent};

#[post("/events/add", format = "application/json", data = "<request>")]
pub async fn add_event(request: Json<SharkEvent>) -> Status {
    todo!()
}

#[post(
    "/events/add/associated",
    format = "application/json",
    data = "<request>"
)]
pub async fn add_associated_event(request: Json<SharkAssociatedEvent>) -> Status {
    todo!()
}
