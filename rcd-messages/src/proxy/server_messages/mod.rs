
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


#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct ExecuteRequest {
    pub login: String,
    pub pw: String,
    pub request_type: u16,
    pub request_json: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct ExecuteReply {
    pub login_success: bool,
    pub execute_success: bool,
    pub reply: Option<String>,
}
