use log::debug;
use rocket::{http::Status, post, serde::json::Json};
use tracking_model::user::User;

#[post("/user/create", format = "application/json", data = "<request>")]
pub async fn create_account(request: Json<User>) -> Status {
    debug!("{request:?}");

    todo!()
}
