use log::debug;
use rocket::{http::Status, post, serde::json::Json};
use tracking_model::user::{CreateUserResult, User};

use crate::srv::{get_client, shark_event::get::DB_NAME};

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
                result_message = Some(e);
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
async fn create_new_account(request: &Json<User>) -> Result<(), String> {
    // we want to create a new account with the un/pw
    // then we want to add a participant with the same un for the alias
    // and then we want to let the UI know that the user should accept the pending contract

    todo!();
}

async fn has_account_with_name(un: &str) -> bool {
    let mut client = get_client().await;

    let sql = "SELECT COUNT(*) cnt FROM user_to_participant WHERE un = ':un'";
    let sql = sql.replace(":un", un);

    let result = client.execute_read_at_host(DB_NAME, &sql, 1).await.unwrap();

    if !result.is_error {
        let rows = result.clone().rows;
        for row in &rows {
            for value in &row.values {
                if let Some(column) = &value.column {
                    if column.column_name == "cnt" {
                        let rv = value.string_value.parse::<u32>();
                        if let Ok(v) = rv {
                            return v > 0;
                        } else {
                            return true;
                        }
                    }
                }
            }
        }
    }

    true
}
