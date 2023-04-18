use crate::{
    srv::{
        get_client,
        user::get::{delete_expired_tokens, get_user_id_for_token, verify_token},
        util::has_any_rows,
        ApiToken,
    },
    ApiSettings,
};
use log::warn;
use rocket::{get, http::Status, post, serde::json::Json, State};
use stdext::function_name;
use tracing::{debug, error, info, trace};
use tracking_model::{
    event::{SharkAssociatedEvent, SharkEvent, EventType},
    user::Auth,
};

pub const SQL_GET_EVENTS: &str =
    "SELECT id, event_date, notes, user_id FROM event WHERE user_id = :uid ;";
pub const SQL_GET_ASSOCIATED_EVENTS: &str =
    "SELECT event_id, event_type, event_date, user_id FROM associated_event WHERE user_id = :uid ;";
pub const DB_NAME: &str = "shark.db";

#[get("/events/get")]
pub async fn get_events(
    token: ApiToken<'_>,
    settings: &State<ApiSettings>,
) -> (Status, Json<Option<Vec<SharkEvent>>>) {
    trace!("[{}]: token: '{}'", function_name!(), &token.jwt());

    let mut request_status: Status = Status::Unauthorized;
    let mut response: Option<Vec<SharkEvent>> = None;

    let delete_tokens_result = delete_expired_tokens(settings).await;
    match delete_tokens_result {
        Ok(_) => trace!("[{}]: expired tokens removed", function_name!()),
        Err(_) => error!("[{}]: Unable to delete expired tokens", function_name!()),
    }

    let is_authenticated_result = verify_token(&token.jwt(), settings).await;

    if let Ok(authenticated) = is_authenticated_result {
        if authenticated {
            trace!("[{}]: is authenticated", function_name!());

            let uid_result = get_user_id_for_token(&token.jwt(), settings).await;

            match uid_result {
                Ok(uid) => {
                    let uid = uid.to_string();

                    let sql =
                        "SELECT COUNT(*) cnt from event WHERE user_id = :uid".replace(":uid", &uid);

                    let has_any_events_result = has_any_rows(&sql, settings).await;

                    if let Ok(has_events) = has_any_events_result {
                        if has_events {
                            let mut associated_events: Vec<SharkAssociatedEvent> = Vec::new();
                            let mut shark_events: Vec<SharkEvent> = Vec::new();

                            let sql = SQL_GET_ASSOCIATED_EVENTS.replace(":uid", &uid);

                            let mut client = get_client(settings).await;
                            let result =
                                client.execute_read_at_host(DB_NAME, &sql, 1).await.unwrap();

                            trace!("[{}]: get_events: {result:?}", function_name!());

                            if !result.is_error {
                                let rows = result.clone().rows;

                                let total_rows = rows.len();
                                trace!("[{}]: total_associated_events: {total_rows:?}", function_name!());

                                for row in &rows {
                                    let mut event_id: u32 = 0;
                                    let mut event_type: u32 = 0;
                                    let mut event_date: String = "".to_string();
                                    let mut notes: String = "".to_string();
                                    let mut user_id: Option<u32> = None;

                                    for value in &row.values {
                                        if let Some(column) = &value.column {
                                            if column.column_name == "event_id" {
                                                let result_event_id =
                                                    value.string_value.parse::<u32>();
                                                if let Ok(eid) = result_event_id {
                                                    event_id = eid;
                                                } else {
                                                    event_id = 0;
                                                }
                                            }

                                            if column.column_name == "event_type" {
                                                let result_event_type =
                                                    value.string_value.parse::<u32>();
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
                                                let result_user_id =
                                                    value.string_value.parse::<u32>();
                                                if let Ok(id) = result_user_id {
                                                    user_id = Some(id);
                                                }
                                            }
                                        }
                                    }

                                    let ae = SharkAssociatedEvent {
                                        event_id: event_id,
                                        event_type: num::FromPrimitive::from_u32(event_type)
                                            .unwrap(),
                                        date: event_date,
                                        notes: Some(notes),
                                        user_id: user_id,
                                    };

                                    associated_events.push(ae);
                                }
                            } else {
                                warn!(
                                    "unable to get associated events, result message was: {}",
                                    result.result_message
                                );
                                // return (Status::InternalServerError, Json(response));
                            }

                            let sql = SQL_GET_EVENTS.replace(":uid", &uid);

                            let result =
                                client.execute_read_at_host(DB_NAME, &sql, 1).await.unwrap();

                            if !result.is_error {
                                let rows = result.clone().rows;

                                let total_rows = rows.len();
                                trace!("[{}]: events: {total_rows:?}", function_name!());

                                for row in &rows {
                                    let mut event_id: u32 = 0;
                                    let mut event_date: String = "".to_string();
                                    let mut notes: String = "".to_string();
                                    let mut user_id: Option<u32> = None;

                                    for value in &row.values {
                                        if let Some(column) = &value.column {
                                            if column.column_name == "id" {
                                                let result_event_id =
                                                    value.string_value.parse::<u32>();
                                                if let Ok(eid) = result_event_id {
                                                    event_id = eid;
                                                } else {
                                                    event_id = 0;
                                                }
                                            }

                                            if column.column_name == "event_date" {
                                                event_date = value.string_value.clone();
                                                trace!("[{}]: added event_date: {event_date:?}", function_name!());
                                            }

                                            if column.column_name == "notes" {
                                                notes = value.string_value.clone();
                                                trace!("[{}]: added notes: {notes:?}", function_name!());
                                            }

                                            if column.column_name == "user_id" {
                                                let result_user_id =
                                                    value.string_value.parse::<u32>();
                                                if let Ok(id) = result_user_id {
                                                    user_id = Some(id);
                                                    trace!("[{}]: added user_id: {user_id:?}", function_name!());
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
                                error!(
                                    "unable to get associated events: {}",
                                    result.result_message
                                );
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
                    } else {
                        info!("[{}]: we don't have any events", function_name!());
                        return (Status::Ok, Json(response));
                    }
                }
                Err(e) => {
                    error!(
                        "[{}]: unable to get user id for token: {}",
                        function_name!(),
                        &token.jwt()
                    );
                    return (Status::InternalServerError, Json(response));
                }
            }
        } else {
            warn!("[{}]: not authenticated", function_name!());
        }
    }

    return (request_status, Json(response));
}


#[get("/events/get/mock")]
pub async fn get_events_mock(
    token: ApiToken<'_>,
    settings: &State<ApiSettings>,
) -> (Status, Json<Option<Vec<SharkEvent>>>) { 

    let mut events: Vec<SharkEvent> = Vec::new();

    let a1 = SharkAssociatedEvent {
        event_id: 1,
        event_type: EventType::Spotting,
        date: "2021-01-01".to_string(),
        notes: Some("test1".to_string()),
        user_id: Some(100),
    };

    let a2 = SharkAssociatedEvent {
        event_id: 1,
        event_type: EventType::Other,
        date: "2021-01-02".to_string(),
        notes: Some("test2".to_string()),
        user_id: Some(100),
    };


    let a3 = SharkAssociatedEvent {
        event_id: 1,
        event_type: EventType::End,
        date: "2021-01-03".to_string(),
        notes: Some("test3".to_string()),
        user_id: Some(100),
    };

    let associated_events1 = vec![a1, a2, a3];
    
    let e = SharkEvent {
        id: 1,
        date: "2020-12-31".to_string(),
        notes: Some("test1".to_string()),
        associated_events: Some(associated_events1),
        user_id: Some(100),
    };

    events.push(e);

    let a1 = SharkAssociatedEvent {
        event_id: 1,
        event_type: EventType::Spotting,
        date: "2021-02-01".to_string(),
        notes: Some("test1".to_string()),
        user_id: Some(100),
    };

    let a2 = SharkAssociatedEvent {
        event_id: 1,
        event_type: EventType::Other,
        date: "2021-02-02".to_string(),
        notes: Some("test2".to_string()),
        user_id: Some(100),
    };


    let a3 = SharkAssociatedEvent {
        event_id: 1,
        event_type: EventType::End,
        date: "2021-02-03".to_string(),
        notes: Some("test3".to_string()),
        user_id: Some(100),
    };

    let associated_events1 = vec![a1, a2, a3];
    
    let e = SharkEvent {
        id: 1,
        date: "2020-01-31".to_string(),
        notes: Some("test1".to_string()),
        associated_events: Some(associated_events1),
        user_id: Some(100),
    };

    events.push(e);

    return (Status::Ok, Json(Some(events)));

}