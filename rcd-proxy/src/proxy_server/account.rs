use log::debug;
use rcd_messages::proxy::server_messages::{RegisterLoginRequest, RegisterLoginReply};
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::RcdProxy;


#[post(
    "/account/register",
    format = "application/json",
    data = "<request>"
)]
pub async fn register(
    request: Json<RegisterLoginRequest>,
    state: &State<RcdProxy>,
) -> (Status, Json<RegisterLoginReply>) {

    debug!("{request:?}");

    let request = request.into_inner();
    let result_register = state.register_user(&request.login, &request.pw);

    let response = match result_register {
        Ok(_) => {
            RegisterLoginReply {
                is_successful: true,
                error: None,
            }
        },
        Err(e) => {
            RegisterLoginReply {
                is_successful: true,
                error: Some(e.to_string()),
            }
        },
    };

    (Status::Ok, Json(response))
}
