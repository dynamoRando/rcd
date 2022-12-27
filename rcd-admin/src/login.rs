use rcd_messages::client::AuthRequest;
use std::fmt;
use std::str::FromStr;
use web_sys::console;
use yew::Properties;

#[derive(Clone, Debug, Eq, PartialEq, Properties, serde::Serialize, serde::Deserialize)]
pub struct Login {
    pub addr: String,
    pub port: u32,
    pub un: String,
    pub pw: String,
}

impl Login {
    pub fn new() -> Login {
        return Login {
            addr: String::new(),
            port: 0,
            un: String::new(),
            pw: String::new(),
        };
    }

    pub fn auth_json(&self) -> String {
        let request = AuthRequest {
            user_name: self.un.to_string(),
            pw: self.pw.to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: String::from(""),
        };

        return serde_json::to_string(&request).unwrap();
    }

    pub fn addr(&self) -> String {
        return format!("{}{}{}{}", "http://", self.addr, ":", self.port);
    }
}

impl fmt::Display for Login {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = serde_json::to_string(&self).unwrap();
        write!(f, "{}", data)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLoginError;

impl FromStr for Login {
    type Err = ParseLoginError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        console::log_1(&s.into());

        let deserializer = &mut serde_json::Deserializer::from_str(s);
        let result: Result<Login, _> = serde_path_to_error::deserialize(deserializer);

        match result {
            Ok(item) => return Ok(item),
            Err(_) => return Err(ParseLoginError),
        }
    }
}
