use rocket::{delete, http::Status, serde::json::Json, State};
use tracking_model::event::SharkEvent;

use crate::srv::ApiToken;

#[delete("/events/delete/<id>")]
pub async fn delete_event(token: ApiToken<'_>, id: usize) -> Status {
    todo!()
}

#[delete("/events/delete/associated/<id>")]
pub async fn delete_associated_event(token: ApiToken<'_>, id: String) -> Status {
    todo!()
}
