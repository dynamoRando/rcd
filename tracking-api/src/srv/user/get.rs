use chrono::{DateTime, Utc};
use log::{debug, error};
use rocket::{http::Status, post, serde::json::Json};
use tracking_model::user::{Token, User};

use crate::{
    error::TrackingApiError,
    srv::{
        get_client,
        shark_event::get::DB_NAME,
        util::{create_jwt, has_any_rows},
    },
};

#[post("/user/auth", format = "application/json", data = "<request>")]
pub async fn auth_for_token(request: Json<User>) -> (Status, Json<Token>) {
    debug!("{request:?}");

    let un = &request.un;

    let delete_tokens_result = delete_expired_tokens().await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let delete_tokens_result = delete_existing_tokens_for_user(&un).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete existing tokens for user {}", un);
    }

    let has_login_result = has_login(&un).await;
    if let Ok(has_login) = has_login_result {
        if has_login {
            let token = create_jwt("tracking-api", un);
            let save_token_result = save_token(&un, &token.0, token.1).await;

            if let Ok(is_token_saved) = save_token_result {
                if is_token_saved {
                    let token = Token {
                        jwt: token.0.clone(),
                        jwt_exp: token.1.to_string(),
                        addr: "shark.home".to_string(),
                        is_logged_in: true,
                        id: None,
                    };

                    return (Status::Ok, Json(token));
                }
            } else {
                error!("Unable to save token for user: {}", un);
            }
        }
    }

    let token = Token {
        jwt: "".to_string(),
        jwt_exp: "".to_string(),
        addr: "shark.home".to_string(),
        is_logged_in: false,
        id: None,
    };

    return (Status::Ok, Json(token));
}

async fn delete_existing_tokens_for_user(un: &str) -> Result<(), TrackingApiError> {
    let mut cmd = String::from("DELETE FROM user_auth WHERE user_name < ':un'");
    cmd = cmd.replace(":un", &un);

    let mut client = get_client().await;

    let delete_expired_tokens_result = client.execute_write_at_host(DB_NAME, &cmd, 1, "").await;

    match delete_expired_tokens_result {
        Ok(_) => Ok(()),
        Err(_) => Err(TrackingApiError::ExpiredTokens),
    }
}

async fn delete_expired_tokens() -> Result<(), TrackingApiError> {
    let now = Utc::now().to_rfc3339();
    let mut cmd = String::from("DELETE FROM user_auth WHERE expiration_utc < ':now'");
    cmd = cmd.replace(":now", &now);

    let mut client = get_client().await;

    let delete_expired_tokens_result = client.execute_write_at_host(DB_NAME, &cmd, 1, "").await;

    match delete_expired_tokens_result {
        Ok(_) => Ok(()),
        Err(_) => Err(TrackingApiError::ExpiredTokens),
    }
}

async fn has_login(un: &str) -> Result<bool, TrackingApiError> {
    let sql = "SELECT COUNT(*) cnt FROM user_to_participant WHERE user_name = ':un'";
    let sql = sql.replace(":un", un);
    has_any_rows(&sql).await
}

async fn save_token(
    un: &str,
    token: &str,
    expiration: DateTime<Utc>,
) -> Result<bool, TrackingApiError> {
    todo!()
}
