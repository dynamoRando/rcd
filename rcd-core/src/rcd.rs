/*

Also known as "core".

This is the Rcd "core" business layer. What was previously defined in the rcd-grpc is intended to be
slowly moved over to a communication ambivalent layer, which is this module.

This 'core' will handle most business actions by way of the defined proto types.

*/

use chrono::Utc;
use rcd_common::defaults;

use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, AuthRequest, AuthResult,
    ChangeHostStatusReply, ChangeHostStatusRequest, DatabaseSchema, GetDatabasesReply,
    GetDatabasesRequest, GetPendingActionsReply, GetPendingActionsRequest, TestReply, TestRequest,
};

use crate::comm::RcdRemoteDbClient;
use crate::dbi::Dbi;

mod contract;
mod db;

#[derive(Debug, Clone)]
pub struct Rcd {
    pub db_interface: Option<Dbi>,
    pub remote_client: Option<RcdRemoteDbClient>,
}

impl Rcd {
    pub async fn change_host_status(
        &self,
        request: ChangeHostStatusRequest,
    ) -> ChangeHostStatusReply {
        return db::change_host_status(&self, request).await;
    }

    pub async fn get_pending_actions_at_participant(
        &self,
        request: GetPendingActionsRequest,
    ) -> GetPendingActionsReply {
        return db::get_pending_updates_at_participant(self, request).await;
    }

    pub async fn accept_pending_action_at_participant(
        &self,
        request: AcceptPendingActionRequest,
    ) -> AcceptPendingActionReply {
        return db::accept_pending_action_at_participant(&self, request).await;
    }

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

        let auth_result = self.verify_login(request.authentication.unwrap());

        if auth_result.0 {
            let db_names = self.dbi().get_database_names();
            for name in &db_names {
                let db_schema = self.dbi().get_database_schema(&name);
                db_result.push(db_schema);
            }
        }

        let result = GetDatabasesReply {
            authentication_result: Some(auth_result.1),
            databases: db_result,
        };

        return result;
    }

    fn verify_login(&self, request: AuthRequest) -> (bool, AuthResult) {
        let is_authenticated = self.dbi().verify_login(&request.user_name, &request.pw);

        let auth_response = AuthResult {
            is_authenticated: is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        return (is_authenticated, auth_response);
    }

    fn dbi(&self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }

    fn remote(&self) -> RcdRemoteDbClient {
        return self.remote_client.as_ref().unwrap().clone();
    }
}
