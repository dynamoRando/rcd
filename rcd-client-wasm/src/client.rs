use crate::token::Token;
use rcd_enum::database_type::DatabaseType;
use rcd_http_common::url::client::{
    AUTH_FOR_TOKEN, COOPERATIVE_WRITE_SQL_AT_HOST, GET_PARTICIPANTS, READ_SQL_AT_HOST,
    READ_SQL_AT_PARTICIPANT, WRITE_SQL_AT_HOST, WRITE_SQL_AT_PARTICIPANT,
};
use rcd_messages::client::{
    AuthRequest, ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
    ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest, GetParticipantsReply,
    GetParticipantsRequest, StatementResultset, TokenReply,
};

use serde::{de, Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcdClient {
    addr: String,
}

impl RcdClient {
    pub fn new(ip: &str, port: u32) -> Self {
        let addr = format!("{}{}{}{}", "http://", ip, ":", port);
        Self { addr }
    }

    pub async fn execute_cooperative_write_at_host(
        &mut self,
        authentication: AuthRequest,
        db_name: &str,
        cmd: &str,
        participant_alias: &str,
        where_clause: &str,
    ) -> Result<bool, String> {
        let request = ExecuteCooperativeWriteRequest {
            authentication: Some(authentication),
            database_name: db_name.to_string(),
            sql_statement: cmd.to_string(),
            database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
            alias: participant_alias.to_string(),
            participant_id: String::from(""),
            where_clause: where_clause.to_string(),
        };

        let url = self.get_http_url(COOPERATIVE_WRITE_SQL_AT_HOST);

        let result: Result<ExecuteCooperativeWriteReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.is_successful),
            Err(e) => Err(e),
        }
    }

    pub async fn execute_write_at_participant(
        &mut self,
        authentication: AuthRequest,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, String> {
        let request = ExecuteWriteRequest {
            authentication: Some(authentication),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        };

        let url = self.get_http_url(WRITE_SQL_AT_PARTICIPANT);

        let result: Result<ExecuteWriteReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.is_successful),
            Err(e) => Err(e),
        }
    }

    pub async fn execute_write_at_host(
        &mut self,
        authentication: AuthRequest,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, String> {
        let request = ExecuteWriteRequest {
            authentication: Some(authentication),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        };

        let url = self.get_http_url(WRITE_SQL_AT_HOST);

        let result: Result<ExecuteWriteReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.is_successful),
            Err(e) => Err(e),
        }
    }

    pub async fn execute_read_at_participant(
        &mut self,
        authentication: AuthRequest,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, String> {
        let request = ExecuteReadRequest {
            authentication: Some(authentication),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        };

        let url = self.get_http_url(READ_SQL_AT_PARTICIPANT);

        let result: Result<ExecuteReadReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.results[0].clone()),
            Err(e) => Err(e),
        }
    }

    pub async fn execute_read_at_host(
        &mut self,
        authentication: AuthRequest,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, String> {
        let request = ExecuteReadRequest {
            authentication: Some(authentication),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        };

        let url = self.get_http_url(READ_SQL_AT_HOST);
        let result: Result<ExecuteReadReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r.results[0].clone()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_participants(
        &mut self,
        authentication: AuthRequest,
        database_name: &str,
    ) -> Result<GetParticipantsReply, String> {
        let request = GetParticipantsRequest {
            authentication: Some(authentication),
            database_name: database_name.to_string(),
        };

        let url = self.get_http_url(GET_PARTICIPANTS);
        let result: Result<GetParticipantsReply, String> =
            self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn auth_for_token(&mut self, un: &str, pw: &str) -> Result<Token, String> {
        let request = AuthRequest {
            user_name: un.to_string(),
            pw: pw.to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: String::from(""),
            id: None,
        };

        let url = self.get_http_url(AUTH_FOR_TOKEN);
        let result: Result<TokenReply, String> = self.get_http_result_error(url, request).await;

        match result {
            Ok(r) => Ok(Token {
                jwt: r.jwt,
                jwt_exp: r.expiration_utc,
                addr: self.addr.clone(),
                is_logged_in: true,
                id: None,
            }),
            Err(e) => Err(e),
        }
    }

    async fn get_http_result_error<
        'a,
        'b,
        T: de::DeserializeOwned + std::clone::Clone,
        U: de::DeserializeOwned + serde::Serialize + std::clone::Clone,
    >(
        &mut self,
        url: String,
        request: U,
    ) -> Result<T, String> {
        let body = serde_json::to_string(&request).unwrap();
        let result = post_result(&url, &body).await;

        match result {
            Ok(r) => {
                let value: T = serde_json::from_str(&r).unwrap();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn get_http_url(&self, action_url: &str) -> String {
        let address = &self.addr;
        let url = format!("{address}{action_url}");
        url
    }
}

pub async fn post_result(url: &str, body: &str) -> Result<String, String> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(body)));

    let request = Request::new_with_str_and_init(url, &opts);

    match request {
        Ok(r) => {
            r.headers().set("Content-Type", "application/json").unwrap();

            let window = web_sys::window().unwrap();
            let resp_value_result = JsFuture::from(window.fetch_with_request(&r)).await;
            match resp_value_result {
                Ok(result) => {
                    assert!(result.is_instance_of::<Response>());
                    let resp: Response = result.dyn_into().unwrap();

                    let json = JsFuture::from(resp.text().unwrap()).await.unwrap();

                    Ok(JsValue::as_string(&json).unwrap())
                }
                Err(e) => {
                    // let m = format!("{:?}", e);
                    // log_to_console(m);

                    if JsValue::is_string(&e) {
                        Err(JsValue::as_string(&e).unwrap())
                    } else {
                        Err("Unable to connect".to_string())
                    }
                }
            }
        }
        Err(e) => {
            if JsValue::is_string(&e) {
                Err(JsValue::as_string(&e).unwrap())
            } else {
                Err("Unable to connect".to_string())
            }
        }
    }
}
