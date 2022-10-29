/*

Also known as "core".

This is the Rcd "core" business layer. What was previously defined in the rcd-grpc is intended to be 
slowly moved over to a communication ambivalent layer, which is this module. 

This 'core' will handle most business actions by way of the defined proto types.

*/


use chrono::Utc;
use rcd_common::defaults;
use rcdproto::rcdp::{
    AuthResult, DatabaseSchema, GetDatabasesReply, GetDatabasesRequest, TestReply, TestRequest,
};

use crate::comm::RcdRemoteDbClient;
use crate::dbi::Dbi;

mod contract;

#[derive(Debug, Clone)]
pub struct Rcd {
    pub db_interface: Option<Dbi>,
    pub remote_client: Option<RcdRemoteDbClient>,
}

impl Rcd {
    pub fn is_online(&self, request: TestRequest) -> TestReply {
        let item = request.request_echo_message;

        let response = TestReply {
            reply_time_utc: String::from(Utc::now().to_rfc2822()),
            reply_echo_message: String::from(item),
            rcdx_version: defaults::VERSION.to_string(),
        };
        return response;
    }

    pub fn get_databases(&self, request: GetDatabasesRequest) -> GetDatabasesReply {
        let mut db_result: Vec<DatabaseSchema> = Vec::new();

        let auth = request.authentication.as_ref().unwrap().clone();
        let is_authenticated = self.dbi().verify_login(&auth.user_name, &auth.pw);

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        if is_authenticated {
            let db_names = self.dbi().get_database_names();
            for name in &db_names {
                let db_schema = self.dbi().get_database_schema(&name);
                db_result.push(db_schema);
            }
        }

        let result = GetDatabasesReply {
            authentication_result: Some(auth_response),
            databases: db_result,
        };

        return result;
    }

    fn dbi(self: &Self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }

    fn remote(&self) -> RcdRemoteDbClient {
        return self.remote_client.as_ref().unwrap().clone();
    }
}
