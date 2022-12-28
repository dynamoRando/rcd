use rcd_messages::client::AuthRequest;
use yew::Properties;

#[derive(Clone, Debug, Eq, PartialEq, Properties, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub jwt: String,
    pub jwt_exp: String,
    pub addr: String,
}

impl Token {
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
