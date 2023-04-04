use rocket::{http::Status, serde::json::Json, State, put};
use tracking_model::event::SharkEvent;


#[put("/events/update")]
pub async fn update_event() -> (Status, Json<SharkEvent>) { 
    todo!()
}


#[put("/events/update/associated")]
pub async fn update_associated_event() -> (Status, Json<SharkEvent>) { 
    todo!()
}
