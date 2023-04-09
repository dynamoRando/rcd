use crate::{
    error::TrackingApiError,
    srv::{get_client, shark_event::get::DB_NAME},
};

use chrono::{DateTime, Duration, Utc};

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha384;
use std::collections::BTreeMap;

/// returns a count of rows where the expected column is "cnt"
pub async fn has_any_rows(sql: &str) -> Result<bool, TrackingApiError> {
    let mut client = get_client().await;

    let result = client.execute_read_at_host(DB_NAME, &sql, 1).await.unwrap();

    if !result.is_error {
        let rows = result.clone().rows;
        for row in &rows {
            for value in &row.values {
                if let Some(column) = &value.column {
                    if column.column_name == "cnt" {
                        let rv = value.string_value.parse::<u32>();
                        if let Ok(v) = rv {
                            return Ok(v > 0);
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


pub fn create_jwt(host_name: &str, login: &str) -> (String, DateTime<Utc>) {
    // this duration should be a config item
    let expiration = Utc::now() + Duration::minutes(20);
    let exp_string = expiration.to_rfc3339();

    // this secret should be stored in a config file: "rcd_item"
    let key: Hmac<Sha384> = Hmac::new_from_slice(b"rcd_item").unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let mut claims = BTreeMap::new();

    claims.insert("sub", login);
    claims.insert("iss", host_name);
    claims.insert("exp", &exp_string);

    let token = Token::new(header, claims).sign_with_key(&key).unwrap();
    let token_str = token.as_str();

    (token_str.to_string(), expiration)
}
