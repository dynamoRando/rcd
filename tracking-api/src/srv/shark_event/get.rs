use crate::{
    srv::{get_client, user::get::verify_token},
    ApiSettings,
};
use rocket::{http::Status, post, serde::json::Json, State};
use tracking_model::{
    event::{SharkAssociatedEvent, SharkEvent, SharkEventReply},
    user::Auth,
};

pub const SQL_GET_EVENTS: &str = "SELECT * FROM event;";
pub const SQL_GET_ASSOCIATED_EVENTS: &str = "SELECT * FROM associated_event;";
pub const DB_NAME: &str = "shark.db";

#[post("/events/get", format = "application/json", data = "<request>")]
pub async fn get_events(
    request: Json<Auth>,
    settings: &State<ApiSettings>,
) -> (Status, Json<SharkEventReply>) {
    let is_logged_in_result = verify_token(&request.jwt, settings).await;

    if let Ok(is_logged_in) = is_logged_in_result {
        if is_logged_in {
            let mut associated_events: Vec<SharkAssociatedEvent> = Vec::new();
            let mut shark_events: Vec<SharkEvent> = Vec::new();

            let mut client = get_client(settings).await;
            let result = client
                .execute_read_at_host(DB_NAME, SQL_GET_ASSOCIATED_EVENTS, 1)
                .await
                .unwrap();

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

            let response = SharkEventReply {
                is_logged_in: true,
                events: shark_events,
            };

            return (Status::Ok, Json(response));
        }
    }

    let response = SharkEventReply {
        is_logged_in: false,
        events: Vec::new(),
    };

    return (Status::Ok, Json(response));
}
