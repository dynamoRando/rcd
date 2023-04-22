use rocket::{delete, http::Status, serde::json::Json, State};
use tracing::{debug, error};
use tracking_model::event::SharkEvent;

use crate::{
    srv::{
        get_client,
        shark_event::get::DB_NAME,
        user::get::{delete_expired_tokens, get_user_name_for_token, verify_token},
        ApiToken,
    },
    ApiSettings,
};

#[delete("/events/delete/<id>")]
pub async fn delete_event(token: ApiToken<'_>, id: usize, settings: &State<ApiSettings>) -> Status {
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

            let cmd = r#"DELETE FROM event WHERE id = :eid"#;
            let cmd = &cmd.replace(":eid", &id.to_string());

            let where_clause = format!("id = {}", id);

            let mut client = get_client(settings).await;
            let delete_event_result = client
                .execute_cooperative_write_at_host(DB_NAME, &cmd, &user_name, &where_clause)
                .await;

            match delete_event_result {
                Ok(is_deleted) => {
                    if is_deleted {
                        let cmd = r#"DELETE FROM associated_event WHERE event_id = :eid"#;
                        let cmd = &cmd.replace(":eid", &id.to_string());

                        let where_clause = format!("event_id = {}", id);

                        let mut client = get_client(settings).await;
                        let delete_event_result = client
                            .execute_cooperative_write_at_host(
                                DB_NAME,
                                &cmd,
                                &user_name,
                                &where_clause,
                            )
                            .await;

                        match delete_event_result {
                            Ok(_) => return Status::Ok,
                            Err(_) => return Status::InternalServerError,
                        }
                    } else {
                        return Status::InternalServerError;
                    }
                }
                Err(_) => {
                    error!(
                        "unable to delete event for token: {} id: {id:?}",
                        &token.jwt()
                    );
                }
            }
        }
    }

    Status::Unauthorized
}

#[delete("/events/delete/associated/<id>")]
pub async fn delete_associated_event(
    token: ApiToken<'_>,
    id: String,
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

            let cmd = r#"DELETE FROM associated_event WHERE uuid = ':eid'"#;
            let cmd = &cmd.replace(":eid", &id.to_string());

            let where_clause = format!("uuid = '{}'", id);

            let mut client = get_client(settings).await;
            let delete_event_result = client
                .execute_cooperative_write_at_host(DB_NAME, &cmd, &user_name, &where_clause)
                .await;

            match delete_event_result {
                Ok(is_deleted) => {
                    if is_deleted {
                        return Status::Ok;
                    } else {
                        return Status::InternalServerError;
                    }
                }
                Err(_) => {
                    error!(
                        "unable to delete event for token: {} id: {id:?}",
                        &token.jwt()
                    );
                }
            }
        }
    }

    Status::Unauthorized
}
