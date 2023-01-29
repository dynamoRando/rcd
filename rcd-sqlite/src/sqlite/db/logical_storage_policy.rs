use crate::sqlite::{
    execute_write_on_connection_at_host, get_db_conn, get_scalar_as_u32, has_any_rows, has_table,
};
use rcd_common::db::DbiConfigSqlite;
use rcd_enum::{logical_storage_policy::LogicalStoragePolicy};
use rcd_error::rcd_db_error::RcdDbError;

use super::{get_all_user_table_names_in_db, populate_data_host_tables};

/// Returns a vector of tuples representing the name of the user table and the logical storage policy
/// attached to it.
pub fn get_logical_storage_policy_for_all_user_tables(
    db_name: &str,
    config: DbiConfigSqlite,
) -> Vec<(String, LogicalStoragePolicy)> {
    let conn = get_db_conn(&config, db_name);

    let mut result: Vec<(String, LogicalStoragePolicy)> = Vec::new();

    let table_names = get_all_user_table_names_in_db(&conn);

    for table_name in &table_names {
        let l_policy =
            get_logical_storage_policy(db_name, &table_name.to_string(), &config).unwrap();
        let item = (table_name.to_string(), l_policy);
        result.push(item);
    }

    result
}

pub fn set_logical_storage_policy(
    db_name: &str,
    table_name: &str,
    policy: LogicalStoragePolicy,
    config: DbiConfigSqlite,
) -> Result<bool, RcdDbError> {
    let conn = get_db_conn(&config, db_name);
    if has_table(table_name, &conn) {
        // insert or update on the coop tables
        let mut cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
        );
        cmd = cmd.replace(":table_name", table_name);
        if has_any_rows(cmd, &conn) {
            // then this is an update
            let mut cmd = String::from(
                "UPDATE COOP_REMOTES
            SET LOGICAL_STORAGE_POLICY = :policy
            WHERE TABLENAME = ':table_name';
            ",
            );

            cmd = cmd.replace(":table_name", table_name);
            cmd = cmd.replace(":policy", &LogicalStoragePolicy::to_u32(policy).to_string());
            let result = execute_write_on_connection_at_host(db_name, &cmd, &config);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        } else {
            // then this is an insert
            let mut cmd = String::from(
                "INSERT INTO COOP_REMOTES
            (
                TABLENAME,
                LOGICAL_STORAGE_POLICY  
            )
            VALUES
            (
                ':table_name',
                :policy
            );",
            );

            cmd = cmd.replace(":table_name", table_name);
            cmd = cmd.replace(":policy", &LogicalStoragePolicy::to_u32(policy).to_string());
            let result = execute_write_on_connection_at_host(db_name, &cmd, &config);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        populate_data_host_tables(db_name, &conn);
    } else {
        let err = RcdDbError::TableNotFoundInDatabase(table_name.to_string(), db_name.to_string());
        return Err(err);
    }
    Ok(true)
}

/// Returns the logical storage policy for the specified table. If the table does not exist in the database, it will
/// return an error. If the table exist but does not have a logical storage policy defined for it, it will default
/// to `LogicalStoragePolicy::None`
pub fn get_logical_storage_policy(
    db_name: &str,
    table_name: &str,
    config: &DbiConfigSqlite,
) -> Result<LogicalStoragePolicy, RcdDbError> {
    let conn = get_db_conn(config, db_name);
    let policy;

    if has_table(table_name, &conn) {
        // insert or update on the coop tables

        if !has_table("COOP_REMOTES", &conn) {
            return Ok(LogicalStoragePolicy::None); 
        }

        let mut cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
        );
        cmd = cmd.replace(":table_name", table_name);
        if has_any_rows(cmd, &conn) {
            // then we have a record for the policy of the table
            let mut cmd = String::from(
                "SELECT LOGICAL_STORAGE_POLICY FROM COOP_REMOTES WHERE TABLENAME = ':table_name';",
            );

            cmd = cmd.replace(":table_name", table_name);
            let i_policy = get_scalar_as_u32(cmd, &conn);
            policy = LogicalStoragePolicy::from_i64(i_policy as i64);
        } else {
            /*
                let error_message = format!(
                    "logical storage policy not saved in COOP_REMOTES for table {} in db {}",
                    table_name, db_name
                );
                let err = RcdDbError::LogicalStoragePolicyNotSet(error_message);
                return Err(err);
            */
            return Ok(LogicalStoragePolicy::None);
        }
    } else {
        let err = RcdDbError::TableNotFoundInDatabase(table_name.to_string(), db_name.to_string());
        return Err(err);
    }

    Ok(policy)
}
