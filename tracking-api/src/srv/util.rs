use crate::{
    error::TrackingApiError,
    srv::{get_client, shark_event::get::DB_NAME},
    ApiSettings,
};

use chrono::{DateTime, Duration, Utc};

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha384;
use std::collections::BTreeMap;
use stdext::function_name;
use tracing::{debug, trace};

/// returns a count of rows where the expected column is "cnt"
pub async fn has_any_rows(sql: &str, settings: &ApiSettings) -> Result<bool, TrackingApiError> {
    let count = get_count(sql, settings).await?;
    trace!("[{}]: has any rows: {}", function_name!(), count);
    Ok(count > 0)
}

/// returns a count of rows where the expected column is "cnt"
pub async fn get_count(sql: &str, settings: &ApiSettings) -> Result<u32, TrackingApiError> {
    let mut client = get_client(settings).await;

    let result = client.execute_read_at_host(DB_NAME, &sql, 1).await.unwrap();

    if !result.is_error {
        return Ok(result.clone().rows.len() as u32);
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
