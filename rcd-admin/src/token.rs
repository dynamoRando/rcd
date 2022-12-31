use rcd_messages::client::AuthRequest;
use yew::Properties;

/// Represents a JWT granted from an RCD instance
#[derive(Clone, Debug, Eq, PartialEq, Properties, serde::Serialize, serde::Deserialize)]
pub struct Token {
    /// The JWT 
    pub jwt: String,
    /// The UTC expiration time of the token
    pub jwt_exp: String,
    /// The HTTP addr of the RCD instance the token is for
    pub addr: String,
}

impl Token {
    pub fn new() -> Token {
        return Token {
            jwt: "".to_string(),
            jwt_exp: "".to_string(),
            addr: "".to_string()
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
        };

        return serde_json::to_string(&request).unwrap();
    }

    /// Returns an AuthRequest with this JWT as the authentication method
    pub fn auth(&self) -> AuthRequest {
        return AuthRequest {
            user_name: "".to_string(),
            pw: "".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: self.jwt.clone(),
        };
    }
}
