
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct RegisterLoginRequest {
    pub login: String,
    pub pw: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct RegisterLoginReply {
    pub is_successful: bool,
    pub error: Option<String>,
}
