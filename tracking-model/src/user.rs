use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub un: String, 
    pub pw: String,
    pub alias: Option<String>,
    pub id: Option<String>,
}
