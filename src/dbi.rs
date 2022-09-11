use rusqlite::{Connection, Error};

use crate::{
    cdata::{ColumnSchema, Contract, DatabaseSchema, Participant},
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::CoopDatabaseParticipant,
    host_info::HostInfo,
    rcd_enum::{
        DatabaseType, LogicalStoragePolicy, RcdDbError, RcdGenerateContractError,
        RemoteDeleteBehavior,
    },
    table::Table,
};

mod sqlite;

pub struct InsertPartialDataResult {
    pub is_successful: bool,
    pub row_id: u32,
    pub data_hash: u64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DbiConfigSqlite {
    pub root_folder: String,
    pub rcd_db_name: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DbiConfigMySql {
    pub user_name: String,
    pub pw: String,
    pub connection_string: String,
    pub host: String,
    pub connect_options: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DbiConfigPostgres {
    pub user_name: String,
    pub pw: String,
    pub connection_string: String,
    pub host: String,
    pub connect_options: String,
}

impl Dbi {
    pub fn db_type(self: &Self) -> DatabaseType {
        return self.db_type;
    }

    pub fn insert_metadata_into_host_db(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: u64,
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::insert_metadata_into_host_db(
                    db_name, table_name, row_id, hash, settings,
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
    ) -> InsertPartialDataResult {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db_part::insert_data_into_partial_db(
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

    #[allow(dead_code, unused_assignments, unused_variables)]
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
                return sqlite::db::update_participant_accepts_contract(
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

    #[allow(dead_code, unused_assignments, unused_variables)]
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
                return sqlite::rcd_db::accept_pending_contract(host_name, &settings);
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
                return sqlite::rcd_db::get_pending_contracts(&settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn save_contract(self: &Self, contract: Contract) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::save_contract(contract, &settings);
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
                return sqlite::rcd_db::has_role_name(role_name, &settings).unwrap();
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn add_login_to_role(self: &Self, login: &str, role_name: &str) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                sqlite::rcd_db::add_login_to_role(login, role_name, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn login_is_in_role(self: &Self, login: &str, role_name: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::rcd_db::login_is_in_role(login, role_name, &settings).unwrap();
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

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
    ) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::add_participant(db_name, alias, ip4addr, db_port, settings);
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
    ) -> CoopDatabaseParticipant {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::get_participant_by_alias(db_name, participant_alias, settings);
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
                return sqlite::db::has_participant(db_name, participant_alias, settings);
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
                return sqlite::db::get_active_contract(db_name, settings);
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
                return sqlite::db::get_logical_storage_policy(db_name, table_name, &settings);
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
                return sqlite::db::set_logical_storage_policy(
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

    pub fn execute_write(self: &Self, db_name: &str, cmd: &str) -> usize {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::execute_write_on_connection(db_name, cmd, &settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn execute_read(self: &Self, db_name: &str, cmd: &str) -> rusqlite::Result<Table> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::execute_read(db_name, cmd, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    #[allow(unused_variables)]
    pub fn has_cooperative_tables_mock(self: &Self, db_name: &str, cmd: &str) -> bool {
        /*
        - we want to call query_parser and get a list of tables that are in this query for this database
        - once we have that list, we will check against dbi to see if any of those tables have a LSP that is cooperative
        - for every table that is cooperative, we need to aggregate the command against all participants
        */
        return false;
    }

    pub fn get_participants_for_table(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> Vec<CoopDatabaseParticipant> {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return sqlite::db::get_participants_for_table(db_name, table_name, settings);
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

                return sqlite::db::generate_contract(
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
        if !HostInfo::exists(self) {
            HostInfo::generate(host_name, self);
        }

        return HostInfo::get(self);
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

    #[allow(dead_code, unused_variables)]
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
