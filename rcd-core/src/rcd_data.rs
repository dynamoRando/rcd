/*

Also known as "core_data".

This is the Rcd "core data" business layer. What was previously defined in the rcd-grpc::data_srv is intended to be
slowly moved over to a communication ambivalent layer, which is this module.

This 'core' will handle most data actions by way of the defined proto types.

*/

use chrono::Utc;
use rcd_common::defaults;
use rcdproto::rcdp::{
    AuthRequest, AuthResult, CreateDatabaseRequest, CreateDatabaseResult, CreateTableRequest,
    CreateTableResult, DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
    GetRowFromPartialDatabaseResult, InsertDataRequest, InsertDataResult,
    NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResponse,
    ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, SaveContractRequest,
    SaveContractResult, TestReply, TestRequest, TryAuthRequest, TryAuthResult, UpdateDataRequest,
    UpdateDataResult, UpdateRowDataHashForHostRequest, UpdateRowDataHashForHostResponse,
};

use crate::dbi::Dbi;

mod contract;
mod crud;
mod db;
#[derive(Debug, Clone)]
pub struct RcdData {
    pub db_interface: Option<Dbi>,
}

impl RcdData {
    fn dbi(&self) -> &Dbi {
        return self.db_interface.as_ref().unwrap();
    }

    pub async fn notify_host_of_removed_row(
        &self,
        request: NotifyHostOfRemovedRowRequest,
    ) -> NotifyHostOfRemovedRowResponse {
        return crud::notify_host_of_removed_row(self, request).await;
    }

    pub async fn save_contract(&self, request: SaveContractRequest) -> SaveContractResult {
        return contract::save_contract(self, request).await;
    }

    pub async fn get_row_from_partial_database(
        &self,
        request: GetRowFromPartialDatabaseRequest,
    ) -> GetRowFromPartialDatabaseResult {
        return crud::get_row_from_partial_database(self, request).await;
    }

    pub async fn accept_contract(
        &self,
        request: ParticipantAcceptsContractRequest,
    ) -> ParticipantAcceptsContractResult {
        return contract::accept_contract(self, request).await;
    }

    pub async fn update_row_data_hash_for_host(
        &self,
        request: UpdateRowDataHashForHostRequest,
    ) -> UpdateRowDataHashForHostResponse {
        return crud::update_row_data_hash_for_host(self, request).await;
    }

    pub async fn update_command_into_table(&self, request: UpdateDataRequest) -> UpdateDataResult {
        return crud::update_command_into_table(self, request).await;
    }

    pub async fn delete_command_into_table(&self, request: DeleteDataRequest) -> DeleteDataResult {
        return crud::delete_command_into_table(self, request).await;
    }

    pub async fn create_table_in_database(&self, request: CreateTableRequest) -> CreateTableResult {
        return db::create_table_in_database(self, request).await;
    }

    pub async fn is_online(&self, request: TestRequest) -> TestReply {
        let item = request.request_echo_message;

        TestReply {
            reply_time_utc: Utc::now().to_rfc2822(),
            reply_echo_message: item,
            rcdx_version: defaults::VERSION.to_string(),
        }
    }

    pub async fn insert_command_into_table(&self, request: InsertDataRequest) -> InsertDataResult {
        return crud::insert_command_into_table(self, request).await;
    }

    pub async fn create_partial_database(
        &self,
        request: CreateDatabaseRequest,
    ) -> CreateDatabaseResult {
        let auth_result = self.authenticate_host(request.authentication.unwrap());

        let db_name = request.database_name;
        let mut db_id = String::from("");

        if auth_result.0 {
            let result = self.dbi().create_partial_database(&db_name);
            if result.is_ok() {
                db_id = self.dbi().get_db_id(db_name.as_str());
            }
        }

        

        CreateDatabaseResult {
            authentication_result: Some(auth_result.1),
            is_successful: auth_result.0,
            database_name: db_name,
            result_message: String::from(""),
            database_id: db_id,
        }
    }

    fn authenticate_host(&self, authentication: AuthRequest) -> (bool, AuthResult) {
        let mut is_authenticated = false;

        let host_id = authentication.user_name;
        let host_token = authentication.token;

        if self.dbi().verify_host_by_id(&host_id, host_token.to_vec()) {
            is_authenticated = true;
        }

        if self
            .dbi()
            .verify_host_by_name(&host_id, host_token.to_vec())
        {
            is_authenticated = true;
        }

        let auth_response = AuthResult {
            is_authenticated,
            user_name: String::from(""),
            token: String::from(""),
            authentication_message: String::from(""),
        };

        (is_authenticated, auth_response)
    }

    fn authenticate_participant(
        &self,
        authentication: AuthRequest,
        db_name: &str,
    ) -> (bool, AuthResult) {
        let host_id = authentication.user_name;
        let host_token = authentication.token;
        let participant = self.dbi().get_participant_by_alias(db_name, &host_id);

        match participant {
            Some(p) => {
                let is_auth = do_vecs_match(&p.token, &host_token);
                let auth_response = AuthResult {
                    is_authenticated: is_auth,
                    user_name: String::from(""),
                    token: String::from(""),
                    authentication_message: String::from(""),
                };

                (is_auth, auth_response)
            }
            None => {
                let auth_response = AuthResult {
                    is_authenticated: false,
                    user_name: String::from(""),
                    token: String::from(""),
                    authentication_message: String::from(""),
                };

                (false, auth_response)
            }
        }
    }

    pub async fn try_auth(&self, request: TryAuthRequest) -> TryAuthResult {
        let is_authenticated = self.authenticate_host(request.authentication.unwrap());

        

        TryAuthResult {
            authentication_result: Some(is_authenticated.1),
        }
    }
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
