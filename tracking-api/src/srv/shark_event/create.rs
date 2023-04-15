use log::{debug, error};
use rocket::{http::Status, post, serde::json::Json, State};
use tracking_model::event::{SharkAssociatedEvent, SharkEvent};

use crate::{ApiSettings, srv::{ApiToken, user::get::{delete_expired_tokens, verify_token}}};

#[post("/events/add", format = "application/json", data = "<request>")]
pub async fn add_event(token: ApiToken<'_>, request: Json<SharkEvent>, settings: &State<ApiSettings>) -> Status {

    debug!("{token:?}");
    debug!("token: '{}'", &token.jwt());

    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let is_authenticated_result = verify_token(&token.jwt(), settings).await;

    if let Ok(authenticated) = is_authenticated_result {
        if authenticated {
            todo!()
        }
    };

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
