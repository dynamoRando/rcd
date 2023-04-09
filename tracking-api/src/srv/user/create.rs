use log::debug;
use rocket::{http::Status, post, serde::json::Json};
use tracking_model::user::{CreateUserResult, User};

use crate::{
    error::TrackingApiError,
    srv::{
        get_client,
        shark_event::get::DB_NAME,
        util::{get_count, has_any_rows},
    },
};

#[post("/user/create", format = "application/json", data = "<request>")]
pub async fn create_account(request: Json<User>) -> (Status, Json<CreateUserResult>) {
    debug!("{request:?}");

    let mut is_successful: bool = false;
    let mut result_message: Option<String> = None;

    let u = request.clone().into_inner();
    let has_account = has_account_with_name(&u.un).await;

    if !has_account {
        let create_account_result = create_new_account(&request).await;
        match create_account_result {
            Ok(()) => {
                is_successful = true;
            }
            Err(e) => {
                is_successful = false;
                result_message = Some(e.to_string());
            }
        }
    }

    if has_account {
        result_message = Some("account already exists".to_string());
    }

    let result = CreateUserResult {
        is_successful: is_successful,
        message: result_message,
    };

    return (Status::Ok, Json(result));
}

/// Attempts to create a new account with the specified un/pw
async fn create_new_account(request: &Json<User>) -> Result<(), TrackingApiError> {
    if request.id.is_none() {
        return Err(TrackingApiError::HostIdMissing(request.un.clone()));
    }

    // we want to create a new account with the un/pw
    // then we want to add a participant with the same un for the alias
    // and then we want to let the UI know that the user should accept the pending contract
    let sql = "SELECT COUNT(*) cnt FROM user_to_participant";
    let total_users = get_count(sql).await?;
    let id = total_users + 1;

    let sql = "INSERT INTO user_to_participant 
    (
        user_id,
        user_name,
        participant_alias,
        participant_id
    )
    VALUES
    (
        :uid,
        ':un',
        ':alias',
        ':id'
    )";

    let sql = sql
        .replace(":uid", &id.to_string())
        .replace(":un", &request.un)
        .replace(":alias", &request.alias.as_ref().unwrap())
        .replace(":id", &request.id.as_ref().unwrap());

    let mut client = get_client().await;
    let add_user_result = client.execute_write_at_host(DB_NAME, &sql, 1, "").await;

    if let Ok(added_user) = add_user_result {
        if added_user {
            let add_participant_result = client
                .add_participant(
                    DB_NAME,
                    &request.un,
                    "proxy.home",
                    50052,
                    "proxy.home",
                    50040,
                    Some(request.id.as_ref().unwrap().clone()),
                )
                .await;

            if let Ok(added_participant) = add_participant_result {
                if added_participant {
                    let send_contract_result =
                        client.send_participant_contract(DB_NAME, &request.un).await;
                    if let Ok(send_contract) = send_contract_result {
                        if send_contract {
                            return Ok(());
                        } else {
                            return Err(TrackingApiError::SendContract(request.un.to_string()));
                        }
                    }
                } else {
                    return Err(TrackingApiError::AddParticipant(request.un.to_string()));
                }
            }
        } else {
            return Err(TrackingApiError::AddUser(request.un.to_string()));
        }
    }

    Err(TrackingApiError::CreateAccountFailed(
        request.un.to_string(),
    ))
}

async fn has_account_with_name(un: &str) -> bool {
    let sql = "SELECT COUNT(*) cnt FROM user_to_participant WHERE user_name = ':un'";
    let sql = sql.replace(":un", un);

    let has_account_result = has_any_rows(&sql).await;
    match has_account_result {
        Ok(has_account) => has_account,
        Err(_) => true,
    }
}
