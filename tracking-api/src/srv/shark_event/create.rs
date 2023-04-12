use rocket::{http::Status, post, serde::json::Json, State};
use tracking_model::event::{SharkAssociatedEvent, SharkEvent};

use crate::ApiSettings;

#[post("/events/add", format = "application/json", data = "<request>")]
pub async fn add_event(request: Json<SharkEvent>, settings: &State<ApiSettings>) -> Status {
    todo!()
}

#[post(
    "/events/add/associated",
    format = "application/json",
    data = "<request>"
)]
pub async fn add_associated_event(request: Json<SharkAssociatedEvent>, settings: &State<ApiSettings>) -> Status {
    todo!()
}
