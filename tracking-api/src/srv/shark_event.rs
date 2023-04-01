use rcd_client::RcdClient;
use rocket::{http::Status, post, serde::json::Json, State};
use crate::shark::SharkEvent;

#[post("/events/get", format = "application/json")]
pub async fn register(
    state: &State<RcdClient>,
) -> (Status, Json<Vec<SharkEvent>>) {
    

    todo!()
}