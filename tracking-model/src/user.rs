use rcd_messages::client::AuthRequest;
use serde_derive::{Deserialize, Serialize};

/*
CREATE TABLE IF NOT EXISTS user_to_participant
(
    user_id INT,
    user_name VARCHAR(25),
    participant_alias VARCHAR(50),
    participant_id CHAR(36)
);

| cid    | name                 | type           | notnull    | dflt_value    | pk    |
| ------ | -------------------- | -------------- | ---------- | ------------- | ----- |
| 0      | user_id              | INT            | 0          |               | 0     |
| 1      | user_name            | varchar(25)    | 0          |               | 0     |
| 2      | participant_alias    | varchar(50)    | 0          |               | 0     |
| 3      | participant_id       | char(36)       | 0          |               | 0     |
 */

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub un: String,
    pub alias: Option<String>,
    pub id: Option<String>,
}

/*
CREATE TABLE IF NOT EXISTS user_auth
(
    user_name VARCHAR(25) NOT NULL,
    token TEXT NOT NULL,
    issued_utc DATETIME,
    expiration_utc DATETIME
);
*/

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub jwt: String,
    pub jwt_exp: String,
    pub addr: String,
    pub is_logged_in: bool,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Auth {
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateUserResult {
    pub is_successful: bool,
    pub message: Option<String>,
}

impl Default for Token {
    fn default() -> Self {
        Self::new()
    }
}

impl Token {
    pub fn new() -> Token {
        Token {
            jwt: "".to_string(),
            jwt_exp: "".to_string(),
            addr: "".to_string(),
            is_logged_in: false,
            id: None,
        }
    }

    /// Returns an AuthRequest in JSON format with this JWT as the authentication method
    pub fn auth_json(&self) -> String {
        let request = AuthRequest {
            user_name: "".to_string(),
            pw: "".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: self.jwt.clone(),
            id: None,
        };

        serde_json::to_string(&request).unwrap()
    }

    /// Returns an AuthRequest with this JWT as the authentication method
    pub fn auth(&self) -> AuthRequest {
        AuthRequest {
            user_name: "".to_string(),
            pw: "".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: self.jwt.clone(),
            id: None,
        }
    }
}
