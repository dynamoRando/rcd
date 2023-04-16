use tracing::{debug, error};
use rocket::{http::Status, post, serde::json::Json, State};
use tracking_model::event::{SharkAssociatedEvent, SharkEvent};

use crate::{
    srv::{
        get_client,
        shark_event::get::{get_events, DB_NAME},
        user::get::{delete_expired_tokens, get_user_name_for_token, verify_token},
        ApiToken,
    },
    ApiSettings,
};

#[post("/events/add", format = "application/json", data = "<request>")]
pub async fn add_event(
    token: ApiToken<'_>,
    request: Json<SharkEvent>,
    settings: &State<ApiSettings>,
) -> Status {
    debug!("{token:?}");
    debug!("token: '{}'", &token.jwt());

    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let is_authenticated_result = verify_token(&token.jwt(), settings).await;
    if let Ok(authenticated) = is_authenticated_result {
        if authenticated {
            let user_name = get_user_name_for_token(&token.jwt(), settings)
                .await
                .expect("could not get user name for token");

            let get_events_result = get_events(token.clone(), settings).await;
            if get_events_result.0 == Status::Ok {
                let mut max_id = 0;

                if get_events_result.1.is_some() {
                    let events = get_events_result.1.as_ref().unwrap().clone();

                    for event in &events {
                        if event.id > max_id {
                            max_id = event.id;
                        }
                    }
                }

                let mut request = request.into_inner();
                request.id = max_id + 1;

                match request.notes {
                    None => {
                        let cmd = r#"
INSERT INTO event
(
    id,
    event_date,
    notes,
    user_id
)
VALUES
(
    :id,
    ':event_date',
    '',
    :uid
)
;"#;

                        let id = request.id.to_string();
                        let uid = request.user_id.unwrap().to_string();

                        let cmd = cmd
                            .replace(":id", &id)
                            .replace(":event_date", &request.date)
                            .replace(":uid", &uid);

                        let mut client = get_client(settings).await;
                        let add_event_result = client
                            .execute_cooperative_write_at_host(DB_NAME, &cmd, &user_name, "")
                            .await;

                        match add_event_result {
                            Ok(is_added) => {
                                if is_added {
                                    return Status::Ok;
                                } else {
                                    return Status::InternalServerError;
                                }
                            }
                            Err(_) => {
                                error!(
                                    "unable to add event for token: {} event: {request:?}",
                                    &token.jwt()
                                );
                            }
                        }
                    }
                    Some(notes) => todo!(),
                }
            }

            return Status::InternalServerError;
        }
    };

    Status::Unauthorized
}

#[post(
    "/events/add/associated",
    format = "application/json",
    data = "<request>"
)]
pub async fn add_associated_event(
    request: Json<SharkAssociatedEvent>,
    settings: &State<ApiSettings>,
) -> Status {
    todo!()
}
