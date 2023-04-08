use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub un: String, 
    pub alias: Option<String>,
    pub id: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateUserResult {
    pub is_successful: bool,
    pub message: Option<String>,
}
