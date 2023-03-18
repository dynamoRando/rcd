use log::debug;
use rcd_messages::proxy::server_messages::{RegisterLoginReply, RegisterLoginRequest};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::RcdProxy;

#[post("/account/register", format = "application/json", data = "<request>")]
pub async fn register(
    request: Json<RegisterLoginRequest>,
    state: &State<RcdProxy>,
) -> (Status, Json<RegisterLoginReply>) {
    debug!("{request:?}");

    let request = request.into_inner();
    let result_register = state.register_user(&request.login, &request.pw);

    let response = match result_register {
        Ok(_) => {
            let result_host_id = state.create_rcd_instance(&request.login, false);

            match result_host_id {
                Ok(id) => RegisterLoginReply {
                    is_successful: false,
                    host_id: Some(id),
                    error: None,
                },
                Err(e) => RegisterLoginReply {
                    is_successful: false,
                    host_id: None,
                    error: Some(e.to_string()),
                },
            }
        }
        Err(e) => RegisterLoginReply {
            is_successful: true,
            host_id: None,
            error: Some(e.to_string()),
        },
    };

    (Status::Ok, Json(response))
}
