use chrono::{DateTime, Utc};
use tracing::{debug, error, trace, warn};
use rocket::{get, http::Status, post, serde::json::Json, State};
use tracking_model::user::{Token, User};

use crate::{
    error::TrackingApiError,
    srv::{
        get_client,
        shark_event::get::DB_NAME,
        util::{create_jwt, has_any_rows},
        ApiToken,
    },
    ApiSettings,
};

#[post("/user/logout", format = "application/json", data = "<request>")]
pub async fn logout(request: Json<User>, settings: &State<ApiSettings>) -> Status {
    debug!("{request:?}");

    let un = &request.un;

    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let delete_tokens_result = delete_existing_tokens_for_user(&un, settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete existing tokens for user {}", un);
    }

    return Status::Ok;
}

#[get("/user/get/<name>")]
pub async fn user_id(
    token: ApiToken<'_>,
    name: String,
    settings: &State<ApiSettings>,
) -> (Status, Json<Option<User>>) {
    debug!("{name:?}");
    debug!("{token:?}");
    debug!("token: '{}'", &token.jwt());

    let mut request_status: Status = Status::Unauthorized;
    let mut user: Option<User> = None;

    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let is_authenticated_result = verify_token(&token.jwt(), settings).await;

    if let Ok(authenticated) = is_authenticated_result {
        if authenticated {
            let get_id_result = get_user_id_for_user_name(&name, settings).await;
            match get_id_result {
                Ok(id) => {
                    user = Some(User {
                        un: name.clone(),
                        alias: None,
                        id: Some(id.to_string()),
                    });

                    request_status = Status::Ok;
                }
                Err(e) => {
                    request_status = Status::InternalServerError;
                    error!(
                        "Unable to get id for user: {} error: {}",
                        name,
                        &e.to_string()
                    );
                }
            };
        } else {
            warn!("invalid authentication for: {}", &token.jwt());
        }
    }

    return (request_status, Json(user));
}

#[post("/user/auth", format = "application/json", data = "<request>")]
pub async fn auth_for_token(
    request: Json<User>,
    settings: &State<ApiSettings>,
) -> (Status, Json<Token>) {
    debug!("{request:?}");

    let un = &request.un;

    let delete_tokens_result = delete_expired_tokens(&settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let delete_tokens_result = delete_existing_tokens_for_user(&un, settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete existing tokens for user {}", un);
    }

    let has_login_result = has_login(&un, settings).await;
    if let Ok(has_login) = has_login_result {
        if has_login {
            trace!("auth_for_token - login found for {}", &un);
            let token = create_jwt("tracking-api", un);
            let save_token_result = save_token(&un, &token.0, token.1, settings).await;

            if let Ok(is_token_saved) = save_token_result {
                if is_token_saved {
                    let token = Token {
                        jwt: token.0.clone(),
                        jwt_exp: token.1.to_string(),
                        addr: "shark.home".to_string(),
                        is_logged_in: true,
                        id: None,
                    };

                    trace!("sending token: {token:?}");

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

pub async fn verify_token(jwt: &str, settings: &ApiSettings) -> Result<bool, TrackingApiError> {
    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let sql = "SELECT COUNT(*) cnt from user_auth WHERE token = ':jwt'";
    let sql = sql.replace(":jwt", jwt);

    has_any_rows(&sql, settings).await
}

pub async fn get_user_id_for_user_name(
    un: &str,
    settings: &ApiSettings,
) -> Result<u32, TrackingApiError> {
    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let sql = "SELECT user_id FROM user_to_participant WHERE user_name = ':un'";
    let sql = sql.replace(":un", &un);

    let mut client = get_client(settings).await;

    let result = client.execute_read_at_host(DB_NAME, &sql, 1).await.unwrap();

    if !result.is_error {
        let rows = result.clone().rows;
        for row in &rows {
            for value in &row.values {
                if let Some(column) = &value.column {
                    if column.column_name == "user_id" {
                        let rid = value.string_value.parse::<u32>();
                        if let Ok(id) = rid {
                            return Ok(id);
                        } else {
                            return Err(TrackingApiError::Unknown);
                        }
                    }
                }
            }
        }
    }

    Err(TrackingApiError::Unknown)
}

pub async fn get_user_id_for_token(
    jwt: &str,
    settings: &ApiSettings,
) -> Result<u32, TrackingApiError> {
    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let user_name = get_user_name_for_token(jwt, settings).await?;
    get_user_id_for_user_name(&user_name, settings).await
}

pub async fn get_user_name_for_token(
    jwt: &str,
    settings: &ApiSettings,
) -> Result<String, TrackingApiError> {
    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let sql = "SELECT user_name FROM user_auth WHERE token = ':jwt'";
    let sql = sql.replace(":jwt", jwt);

    let mut client = get_client(settings).await;

    let result = client.execute_read_at_host(DB_NAME, &sql, 1).await.unwrap();

    if !result.is_error {
        let rows = result.clone().rows;
        for row in &rows {
            for value in &row.values {
                if let Some(column) = &value.column {
                    if column.column_name == "user_name" {
                        return Ok(value.string_value.clone());
                    }
                }
            }
        }
    }

    Err(TrackingApiError::Unknown)
}

async fn delete_existing_tokens_for_user(
    un: &str,
    settings: &ApiSettings,
) -> Result<(), TrackingApiError> {
    let mut cmd = String::from("DELETE FROM user_auth WHERE user_name < ':un'");
    cmd = cmd.replace(":un", &un);

    let mut client = get_client(settings).await;

    let delete_expired_tokens_result = client.execute_write_at_host(DB_NAME, &cmd, 1, "").await;

    trace!(
        "delete_existing_tokens_for_user: {} result: {delete_expired_tokens_result:?}",
        &un
    );

    match delete_expired_tokens_result {
        Ok(_) => Ok(()),
        Err(_) => Err(TrackingApiError::ExpiredTokens),
    }
}

pub async fn delete_expired_tokens(settings: &ApiSettings) -> Result<(), TrackingApiError> {
    let now = Utc::now().to_rfc3339();
    let mut cmd = String::from("DELETE FROM user_auth WHERE expiration_utc < ':now'");
    cmd = cmd.replace(":now", &now);

    let mut client = get_client(settings).await;

    trace!("delete_expired_tokens - sending cmd: {cmd:?}");

    let delete_expired_tokens_result = client.execute_write_at_host(DB_NAME, &cmd, 1, "").await;

    trace!("delete_expired_tokens - cmd: {cmd:?} result: {delete_expired_tokens_result:?}");

    match delete_expired_tokens_result {
        Ok(_) => Ok(()),
        Err(_) => Err(TrackingApiError::ExpiredTokens),
    }
}

async fn has_login(un: &str, settings: &ApiSettings) -> Result<bool, TrackingApiError> {
    let sql = "SELECT COUNT(*) cnt FROM user_to_participant WHERE user_name = ':un'";
    let sql = sql.replace(":un", un);
    has_any_rows(&sql, settings).await
}

async fn save_token(
    un: &str,
    token: &str,
    expiration: DateTime<Utc>,
    settings: &ApiSettings,
) -> Result<bool, TrackingApiError> {
    let sql = "INSERT INTO user_auth
    (
        user_name,
        token,
        issued_utc,
        expiration_utc
    )
    VALUES
    (
        ':un',
        ':jwt',
        ':iss',
        ':exp'
    );
    ";

    let sql = sql
        .replace(":un", un)
        .replace(":jwt", token)
        .replace(":exp", &expiration.to_rfc3339().to_string())
        .replace(":iss", &Utc::now().to_rfc3339().to_string());

    trace!("save_token: cmd: {sql:?}");

    let mut client = get_client(settings).await;

    let result = client.execute_write_at_host(DB_NAME, &sql, 1, "").await;

    match result {
        Ok(is_saved) => {
            trace!("saved token: {}", &token);
            return Ok(is_saved);
        }
        Err(_) => Err(TrackingApiError::Unknown),
    }
}
