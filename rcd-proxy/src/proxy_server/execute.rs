use log::debug;
use rcd_messages::proxy::server_messages::{ExecuteReply, ExecuteRequest};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::{proxy_server::process::process_request, RcdProxy};

#[post("/execute", format = "application/json", data = "<request>")]
pub async fn execute_request(
    request: Json<ExecuteRequest>,
    state: &State<RcdProxy>,
) -> (Status, Json<ExecuteReply>) {
    debug!("{request:?}");

    let request = request.into_inner();
    let result_id = state.get_host_id_for_user(&request.login);
    let response = match result_id {
        Ok(id) => match id {
            Some(id) => {
                let result_core = state.get_rcd_core_for_existing_host(&id);
                match result_core {
                    Ok(core) => {
                        let result_json_reply = process_request(request, &core).await;
                        match result_json_reply {
                            Ok(reply) => ExecuteReply {
                                login_success: true,
                                execute_success: true,
                                reply: Some(reply),
                            },
                            Err(e) => ExecuteReply {
                                login_success: true,
                                execute_success: false,
                                reply: Some(e.to_string()),
                            },
                        }
                    }
                    Err(e) => ExecuteReply {
                        login_success: false,
                        execute_success: false,
                        reply: Some(e.to_string()),
                    },
                }
            }
            None => ExecuteReply {
                login_success: false,
                execute_success: false,
                reply: None,
            },
        },
        Err(e) => ExecuteReply {
            login_success: false,
            execute_success: false,
            reply: Some(e.to_string()),
        },
    };

    (Status::Ok, Json(response))
}
