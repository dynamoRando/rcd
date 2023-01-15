use ::rcd_enum::rcd_database_type::RcdDatabaseType;
use chrono::{DateTime, Utc};
use rcd_common::{
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::{CoopDatabaseParticipant, CoopDatabaseParticipantData},
    db::{CdsHosts, DbiConfigMySql, DbiConfigPostgres, DbiConfigSqlite, PartialDataResult},
    host_info::HostInfo,
    rcd_enum::{
        ContractStatus, DatabaseType, DeletesFromHostBehavior, LogicalStoragePolicy, RcdDbError,
        RcdGenerateContractError, RemoteDeleteBehavior, UpdatesFromHostBehavior,
        
    },
    table::Table,
};
use rcd_enum::updates_to_host_behavior::UpdatesToHostBehavior;
use rcd_enum::deletes_to_host_behavior::DeletesToHostBehavior;
use rcd_sqlite::sqlite::{self};
use rcdproto::rcdp::{
    ColumnSchema, Contract, DatabaseSchema, Participant, ParticipantStatus, PendingStatement, Row,
    TokenReply,
};
use rusqlite::{Connection, Error};

use crate::auth;

#[derive(Debug, Clone)]
/// Database Interface: an abstraction over the underlying database layer. Supports:
/// - Sqlite
/// - MySQL
/// - Postgres
/// - SQL Server
pub struct Dbi {
    pub db_type: DatabaseType,
    pub mysql_config: Option<DbiConfigMySql>,
    pub postgres_config: Option<DbiConfigPostgres>,
    pub sqlite_config: Option<DbiConfigSqlite>,
}

impl Dbi {
    pub fn auth_for_token(self: &Self, login: &str, pw: &str) -> TokenReply {
        let mut is_authorized = false;
        let mut jwt = String::from("");
        let mut expiration_utc = String::from("");

        if self.verify_login(login, pw) {
            is_authorized = true;

            if !self.login_has_token(login) {
                let token_data = self.create_token_for_login(login);
                jwt = token_data.0;
                expiration_utc = token_data.1.to_string();
            }
        }

        let reply = TokenReply {
            is_successful: is_authorized,
            expiration_utc,
            jwt,
        };

        return reply;
    }

    pub fn login_has_token(&self, login: &str) -> bool {
        self.delete_expired_tokens();
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return rcd_sqlite::sqlite::rcd_db::login_has_token(login, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn revoke_token(&self, jwt: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return rcd_sqlite::sqlite::rcd_db::revoke_token(&jwt, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn create_token_for_login(&self, login: &str) -> (String, DateTime<Utc>) {
        let host_info = self.rcd_get_host_info();
        let token_data = auth::create_jwt(&host_info.name, login);
        self.save_token(login, &token_data.0.clone(), token_data.1);
        return token_data;
    }

    pub fn get_cooperative_hosts(&self) -> Vec<CdsHosts> {
        self.delete_expired_tokens();
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return rcd_sqlite::sqlite::rcd_db::get_cooperative_hosts(&settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn verify_token(&self, token: String) -> bool {
        self.delete_expired_tokens();
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return rcd_sqlite::sqlite::rcd_db::verify_token(&token, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn delete_expired_tokens(&self) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                rcd_sqlite::sqlite::rcd_db::delete_expired_tokens(&settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn save_token(&self, login: &str, token: &str, expiration: DateTime<Utc>) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                rcd_sqlite::sqlite::rcd_db::save_token(&login, &token, expiration, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn accept_pending_action_at_participant(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> PartialDataResult {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return rcd_sqlite::sqlite::db_part::accept_pending_action_at_participant(
                    db_name, table_name, row_id, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_pending_actions(
        self: &Self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Vec<PendingStatement> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::get_pending_actions(
                    db_name, table_name, action, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_data_hash_at_host(self: &Self, db_name: &str, table_name: &str, row_id: u32) -> u64 {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::metadata::get_data_hash_at_host(
                    db_name, table_name, row_id, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_data_hash_at_participant(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> u64 {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::get_data_hash_at_participant(
                    db_name, table_name, row_id, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn read_row_id_from_part_db(
        self: &Self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> u32 {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::read_row_id_from_part_db(
                    db_name,
                    table_name,
                    where_clause,
                    &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn remove_remote_row_reference_from_host(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::metadata::remove_remote_row_reference_from_host(
                    db_name, table_name, row_id, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_cds_host_for_part_db(self: &Self, db_name: &str) -> Option<CdsHosts> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_cds_host_for_part_db(db_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_rcd_db_type(self: &Self, db_name: &str) -> RcdDatabaseType {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_rcd_db_type(db_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn db_type(self: &Self) -> DatabaseType {
        return self.db_type;
    }

    pub fn get_updates_to_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> UpdatesToHostBehavior {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_updates_to_host_behavior(
                    db_name, table_name, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_deletes_to_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> DeletesToHostBehavior {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_deletes_to_host_behavior(
                    db_name, table_name, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_deletes_from_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> DeletesFromHostBehavior {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_deletes_from_host_behavior(
                    db_name, table_name, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_updates_from_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> UpdatesFromHostBehavior {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_updates_from_host_behavior(
                    db_name, table_name, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn change_updates_from_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::change_updates_from_host_behavior(
                    db_name, table_name, behavior, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn change_deletes_from_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::change_deletes_from_host_behavior(
                    db_name, table_name, behavior, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn change_updates_to_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::change_updates_to_host_behavior(
                    db_name, table_name, behavior, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn change_deletes_to_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: u32,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::change_deletes_to_host_behavior(
                    db_name, table_name, behavior, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_row_from_partial_database(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Row {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::get_row_from_partial_database(
                    db_name, table_name, row_id, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn change_host_status_by_id(self: &Self, host_id: &str, status: u32) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::change_host_status_by_id(host_id, status, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn change_host_status_by_name(self: &Self, host_name: &str, status: u32) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::change_host_status_by_name(host_name, status, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn verify_host_by_id(self: &Self, host_id: &str, token: Vec<u8>) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::verify_host_by_id(host_id, token, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn verify_host_by_name(self: &Self, host_name: &str, token: Vec<u8>) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::verify_host_by_name(host_name, token, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn delete_metadata_in_host_db(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        internal_participant_id: &str,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::metadata::delete_metadata_in_host_db(
                    db_name,
                    table_name,
                    row_id,
                    internal_participant_id,
                    settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn update_metadata_in_host_db(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::metadata::update_metadata_in_host_db(
                    db_name,
                    table_name,
                    row_id,
                    hash,
                    internal_participant_id,
                    settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn insert_metadata_into_host_db(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
        internal_participant_id: &str,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::metadata::insert_metadata_into_host_db(
                    db_name,
                    table_name,
                    row_id,
                    hash,
                    internal_participant_id,
                    settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn delete_data_in_partial_db(
        self: &Self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host_id: &str,
    ) -> PartialDataResult {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::delete::delete_data_in_partial_db(
                    part_db_name,
                    table_name,
                    cmd,
                    where_clause,
                    host_id,
                    &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn update_data_into_partial_db_queue(
        self: &Self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        where_clause: &str,
        host: &CdsHosts,
    ) -> PartialDataResult {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::update::update_data_into_partial_db_queue(
                    part_db_name,
                    table_name,
                    cmd,
                    where_clause,
                    &host.host_id,
                    &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn update_data_into_partial_db(
        self: &Self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
        host_id: &str,
        where_clause: &str,
    ) -> PartialDataResult {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::update::update_data_into_partial_db(
                    part_db_name,
                    table_name,
                    cmd,
                    where_clause,
                    host_id,
                    &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn insert_data_into_partial_db(
        self: &Self,
        part_db_name: &str,
        table_name: &str,
        cmd: &str,
    ) -> PartialDataResult {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::insert::insert_data_into_partial_db(
                    part_db_name,
                    table_name,
                    cmd,
                    &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn update_participant_accepts_contract(
        self: &Self,
        db_name: &str,
        participant: CoopDatabaseParticipant,
        participant_message: Participant,
        accepted_contract_id: &str,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::contract::update_participant_accepts_contract(
                    db_name,
                    participant,
                    participant_message,
                    accepted_contract_id,
                    settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn create_partial_database_from_contract(self: &Self, contract: &Contract) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::create_partial_database_from_contract(contract, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn accept_pending_contract(self: &Self, host_name: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::contract::accept_pending_contract(host_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_pending_contracts(self: &Self) -> Vec<Contract> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::contract::get_pending_contracts(&settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn save_contract(self: &Self, contract: Contract) -> (bool, String) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                let result = sqlite::rcd_db::contract::save_contract(contract, &settings);
                return result;
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_table_id(self: &Self, db_name: &str, table_name: &str) -> String {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::get_table_id(db_name, table_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn create_table_in_partial_database(
        self: &Self,
        db_name: &str,
        table_name: &str,
        schema: Vec<ColumnSchema>,
    ) -> rusqlite::Result<bool> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::create_table_in_partial_database(
                    db_name, table_name, schema, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_db_id(self: &Self, db_name: &str) -> String {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::get_db_id(db_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn create_partial_database(
        self: &Self,
        db_name: &str,
    ) -> Result<Connection, rusqlite::Error> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                sqlite::db_part::create_partial_database(db_name, &settings)
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn has_role_name(self: &Self, role_name: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::role::has_role_name(role_name, &settings).unwrap();
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    /// Associates the specified login to the specified role
    pub fn add_login_to_role(self: &Self, login: &str, role_name: &str) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                sqlite::rcd_db::role::add_login_to_role(login, role_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    /// Checks if the specified login is in the specified role
    pub fn login_is_in_role(self: &Self, login: &str, role_name: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::role::login_is_in_role(login, role_name, &settings)
                    .unwrap();
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    /// Creates a login with the specific values
    pub fn create_login(self: &Self, login: &str, pw: &str) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                sqlite::rcd_db::create_login(login, pw, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    /// Returns the names of all the database at this instance
    pub fn get_database_names(self: &Self) -> Vec<String> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_database_names(&settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn has_login(self: &Self, login: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::has_login_via_config(login, settings).unwrap();
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn add_participant(
        self: &Self,
        db_name: &str,
        alias: &str,
        ip4addr: &str,
        db_port: u32,
        http_addr: String,
        http_port: u16,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::participant::add_participant(
                    db_name, alias, ip4addr, db_port, settings, http_addr, http_port,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_database_schema(self: &Self, db_name: &str) -> DatabaseSchema {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::get_db_schema(db_name, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_participant_by_alias(
        self: &Self,
        db_name: &str,
        participant_alias: &str,
    ) -> Option<CoopDatabaseParticipant> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::participant::get_participant_by_alias(
                    db_name,
                    participant_alias,
                    settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn has_participant(self: &Self, db_name: &str, participant_alias: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::participant::has_participant(
                    db_name,
                    participant_alias,
                    settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_active_contract(self: &Self, db_name: &str) -> CoopDatabaseContract {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::contract::get_active_contract(db_name, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_logical_storage_policy(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, RcdDbError> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::logical_storage_policy::get_logical_storage_policy(
                    db_name, table_name, &settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn set_logical_storage_policy(
        self: &Self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, RcdDbError> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::logical_storage_policy::set_logical_storage_policy(
                    db_name, table_name, policy, settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn has_table(self: &Self, db_name: &str, table_name: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::has_table_client_service(db_name, table_name, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn execute_write_at_host(self: &Self, db_name: &str, cmd: &str) -> Result<usize, String> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::execute_write_on_connection_at_host(db_name, cmd, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn execute_write_at_partipant(self: &Self, db_name: &str, cmd: &str) -> usize {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::execute_write_on_connection_at_participant(db_name, cmd, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn execute_read_at_participant(
        self: &Self,
        db_name: &str,
        cmd: &str,
    ) -> rusqlite::Result<Table> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::execute_read_at_participant(db_name, cmd, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn execute_read_at_host(self: &Self, db_name: &str, cmd: &str) -> rusqlite::Result<Table> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::execute_read_at_host(db_name, cmd, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn has_cooperative_tables(self: &Self, db_name: &str, cmd: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::has_cooperative_tables(db_name, cmd, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_participants_for_table(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> Vec<CoopDatabaseParticipantData> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::participant::get_participants_for_table(
                    db_name, table_name, settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_active_contract_proto(self: &Self, db_name: &str) -> Contract {
        let active_contract = self.get_active_contract(&db_name);
        let db_schema = self.get_database_schema(&db_name);
        let host_info = self.rcd_get_host_info();
        return active_contract.to_cdata_contract(
            &host_info,
            "",
            "",
            0,
            ContractStatus::Unknown,
            db_schema,
            "",
            0,
        );
    }

    pub fn get_participants_for_database(self: &Self, db_name: &str) -> Vec<ParticipantStatus> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::participant::get_participants_for_database(db_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn get_cooperative_tables(self: &Self, db_name: &str, cmd: &str) -> Vec<String> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::get_cooperative_tables(db_name, cmd, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn create_database(self: &Self, db_name: &str) -> Result<Connection, Error> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::create_database(db_name, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn enable_coooperative_features(self: &Self, db_name: &str) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();

                sqlite::db::enable_coooperative_features(db_name, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn generate_contract(
        self: &Self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, RcdGenerateContractError> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();

                let _ = self.generate_and_get_host_info(host_name);

                return sqlite::db::contract::generate_contract(
                    db_name,
                    desc,
                    remote_delete_behavior,
                    settings,
                );
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn rcd_get_host_info(self: &Self) -> HostInfo {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::get_host_info(settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn rcd_generate_host_info(self: &Self, host_name: &str) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                sqlite::rcd_db::generate_host_info(host_name, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn if_rcd_host_info_exists(self: &Self) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::if_host_info_exists(settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    /// Generates the host info and saves it to our rcd_db if it has not alraedy been generated.
    /// Will always return the current `HostInfo`
    pub fn generate_and_get_host_info(self: &Self, host_name: &str) -> HostInfo {
        if !self.if_rcd_host_info_exists() {
            self.rcd_generate_host_info(host_name);
        }

        return self.rcd_get_host_info();
    }

    pub fn configure_admin(self: &Self, login: &str, pw: &str) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::configure_admin(login, pw, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn verify_login(self: &Self, login: &str, pw: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::verify_login(login, pw, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn configure_rcd_db(self: &Self) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                sqlite::rcd_db::configure_rcd_db(&settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    fn get_sqlite_settings(self: &Self) -> DbiConfigSqlite {
        return self.sqlite_config.as_ref().unwrap().clone();
    }
}
