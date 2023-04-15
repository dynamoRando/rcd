use crate::{
    srv::{
        get_client,
        user::get::{delete_expired_tokens, verify_token},
        ApiToken,
    },
    ApiSettings,
};
use log::{debug, error, trace};
use rocket::{get, http::Status, post, serde::json::Json, State};
use tracking_model::{
    event::{SharkAssociatedEvent, SharkEvent},
    user::Auth,
};

pub const SQL_GET_EVENTS: &str = "SELECT * FROM event;";
pub const SQL_GET_ASSOCIATED_EVENTS: &str = "SELECT * FROM associated_event;";
pub const DB_NAME: &str = "shark.db";

#[get("/events/get")]
pub async fn get_events(
    token: ApiToken<'_>,
    settings: &State<ApiSettings>,
) -> (Status, Json<Option<Vec<SharkEvent>>>) {
    debug!("{token:?}");
    debug!("token: '{}'", &token.jwt());

    let mut request_status: Status = Status::Unauthorized;
    let mut response: Option<Vec<SharkEvent>> = None;

    let delete_tokens_result = delete_expired_tokens(settings).await;
    if let Err(_) = delete_tokens_result {
        error!("Unable to delete expired tokens");
    }

    let is_authenticated_result = verify_token(&token.jwt(), settings).await;

    if let Ok(authenticated) = is_authenticated_result {
        if authenticated {
            let mut associated_events: Vec<SharkAssociatedEvent> = Vec::new();
            let mut shark_events: Vec<SharkEvent> = Vec::new();

            let mut client = get_client(settings).await;
            let result = client
                .execute_read_at_host(DB_NAME, SQL_GET_ASSOCIATED_EVENTS, 1)
                .await
                .unwrap();

            trace!("get_events: {result:?}");

            if !result.is_error {
                let rows = result.clone().rows;
                for row in &rows {
                    let mut event_id: u32 = 0;
                    let mut event_type: u32 = 0;
                    let mut event_date: String = "".to_string();
                    let mut notes: String = "".to_string();
                    let mut user_id: Option<u32> = None;

                    for value in &row.values {
                        if let Some(column) = &value.column {
                            if column.column_name == "event_id" {
                                let result_event_id = value.string_value.parse::<u32>();
                                if let Ok(eid) = result_event_id {
                                    event_id = eid;
                                } else {
                                    event_id = 0;
                                }
                            }

                            if column.column_name == "event_type" {
                                let result_event_type = value.string_value.parse::<u32>();
                                if let Ok(et) = result_event_type {
                                    event_type = et;
                                } else {
                                    event_type = 0;
                                }
                            }

                            if column.column_name == "event_date" {
                                event_date = value.string_value.clone();
                            }

                            if column.column_name == "notes" {
                                notes = value.string_value.clone();
                            }

                            if column.column_name == "user_id" {
                                let result_user_id = value.string_value.parse::<u32>();
                                if let Ok(id) = result_user_id {
                                    user_id = Some(id);
                                }
                            }
                        }
                    }

                    let ae = SharkAssociatedEvent {
                        event_id: event_id,
                        event_type: num::FromPrimitive::from_u32(event_type).unwrap(),
                        date: event_date,
                        notes: Some(notes),
                        user_id: user_id,
                    };

                    associated_events.push(ae);
                }
            } else {
                error!("{}", result.result_message);
                return (Status::InternalServerError, Json(response));
            }

            let result = client
                .execute_read_at_host(DB_NAME, SQL_GET_EVENTS, 1)
                .await
                .unwrap();

            if !result.is_error {
                let rows = result.clone().rows;
                for row in &rows {
                    let mut event_id: u32 = 0;
                    let mut event_date: String = "".to_string();
                    let mut notes: String = "".to_string();
                    let mut user_id: Option<u32> = None;

                    for value in &row.values {
                        if let Some(column) = &value.column {
                            if column.column_name == "id" {
                                let result_event_id = value.string_value.parse::<u32>();
                                if let Ok(eid) = result_event_id {
                                    event_id = eid;
                                } else {
                                    event_id = 0;
                                }
                            }

                            if column.column_name == "event_date" {
                                event_date = value.string_value.clone();
                            }

                            if column.column_name == "notes" {
                                notes = value.string_value.clone();
                            }

                            if column.column_name == "user_id" {
                                let result_user_id = value.string_value.parse::<u32>();
                                if let Ok(id) = result_user_id {
                                    user_id = Some(id);
                                }
                            }
                        }
                    }

                    let e = SharkEvent {
                        id: event_id,
                        date: event_date,
                        notes: Some(notes),
                        associated_events: None,
                        user_id: user_id,
                    };

                    shark_events.push(e);
                }
            } else {
                error!("{}", result.result_message);
                return (Status::InternalServerError, Json(response));
            }

            for event in shark_events.iter_mut() {
                for ae in &associated_events {
                    if ae.event_id == event.id {
                        if event.associated_events.is_none() {
                            let x: Vec<SharkAssociatedEvent> = Vec::new();
                            event.associated_events = Some(x);
                        }

                        event.associated_events.as_mut().unwrap().push(ae.clone());
                    }
                }
            }

            response = Some(shark_events);
            request_status = Status::Ok;
        }
    }

    return (request_status, Json(response));
}
