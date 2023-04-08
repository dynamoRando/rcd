use chrono::{TimeZone, Utc};
use guid_create::GUID;
use log::trace;
use rcd_common::{
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::CoopDatabaseParticipant, db::DbiConfigSqlite, defaults,
};

use rcd_enum::{
    logical_storage_policy::LogicalStoragePolicy,
    rcd_generate_contract_error::RcdGenerateContractError,
    remote_delete_behavior::RemoteDeleteBehavior,
};
use rcdproto::rcdp::Participant;
use rusqlite::{named_params, Connection, Result};

use crate::sqlite::{execute_read, execute_write, get_db_conn, has_any_rows};

use super::logical_storage_policy::get_logical_storage_policy_for_all_user_tables;

/// Attempts to generate a contract for the user database. This will first validate if all user
/// tables have a logical storage policy set. If not it will return a generate contract error.
/// If there is no existing contract, it will generate one. If there is an already existing active contract,
/// it will retire that contract and generate a new one.
pub fn generate_contract(
    db_name: &str,
    desc: &str,
    remote_delete_behavior: RemoteDeleteBehavior,
    config: DbiConfigSqlite,
) -> Result<bool, RcdGenerateContractError> {
    /*
       First, we should check to see if there is a logical storage policy
       defined on all user tables. If any are not set, then this should return
       a RcdGenerateContractError.

       We need to see if there are other database contracts.
       If there are none, then this is the first contract ever.

       If there are existing contracts, we need to find the active one
       and retire it, then generate the current one.
    */

    // trace!("generate contract: start for {}", db_name);

    let conn = &get_db_conn(&config, db_name);
    let policies = get_logical_storage_policy_for_all_user_tables(db_name, config);

    // check to see if all user tables have a logical storage policy set
    // if any don't, return an error.
    if policies.iter().any(|p| p.1 == LogicalStoragePolicy::None) {
        let mut missing_policies = String::from("policy not set for ");

        for p in policies {
            if p.1 == LogicalStoragePolicy::None {
                let message = format!("table {}, ", p.0);
                missing_policies.push_str(&message);
            }
        }

        let error = RcdGenerateContractError::NotAllTablesSet(missing_policies);
        return Err(error);
    }

    let cmd = String::from("SELECT COUNT(*) TOTALCONTRACTS FROM COOP_DATABASE_CONTRACT");
    if !has_any_rows(cmd, conn) {
        // this is the first contract
        // trace!("generate contract: first_contract");
        let contract = CoopDatabaseContract {
            contract_id: GUID::rand(),
            generated_date: Utc::now(),
            description: desc.to_string(),
            retired_date: None,
            version_id: GUID::rand(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };
        save_contract_at_connection(contract, conn);
    } else {
        // there are other contracts, we need to find the active one and retire it
        // then generate a new contract
        let contracts = get_all_database_contracts(conn);
        // trace!("generate contract: retire contracts");
        trace!(
            "generate contract: retire contracts count: {}",
            contracts.len()
        );
        for con in contracts {
            if !con.is_retired() {
                trace!(
                    "generate contract: retire contract {}",
                    &con.contract_id.to_string()
                );
                retire_contract(con.version_id, conn);
                // trace!(
                //     "generate contract: save retired contract {}",
                //     &con.contract_id.to_string()
                // );
                // save_contract_at_connection(con, conn);
            }
        }

        trace!("generate contract: retired. create new contract");
        let new_contract = CoopDatabaseContract {
            contract_id: GUID::rand(),
            generated_date: Utc::now(),
            description: desc.to_string(),
            retired_date: None,
            version_id: GUID::rand(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        };
        save_contract_at_connection(new_contract, conn);
    }
    Ok(true)
}

pub fn save_contract_at_connection(contract: CoopDatabaseContract, conn: &Connection) {
    let mut cmd = String::from(
        "SELECT COUNT(*) TOTALCOUNT FROM COOP_DATABASE_CONTRACT WHERE VERSION_ID = ':vid'",
    );
    cmd = cmd.replace(":vid", &contract.version_id.to_string());
    if has_any_rows(cmd, conn) {
        // this is an update
        if contract.is_retired() {
            let mut cmd = String::from(
                "
            UPDATE COOP_DATABASE_CONTRACT 
            SET 
                CONTRACT_ID = ':cid',
                GENERATED_DATE_UTC = ':gen_date',
                DESCRIPTION = ':desc',
                RETIRED_DATE_UTC = ':ret_date',
                REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
            WHERE
                VERSION_ID = ':vid'",
            );
            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            let ret = &contract.retired_date.unwrap().to_string();
            cmd = cmd.replace(":ret_date", ret);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(conn, &cmd);
        } else {
            let mut cmd = String::from(
                "
            UPDATE COOP_DATABASE_CONTRACT 
            SET 
                CONTRACT_ID = ':cid',
                GENERATED_DATE_UTC = ':gen_date',
                DESCRIPTION = ':desc',
                REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
            WHERE
                VERSION_ID = ':vid'",
            );
            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(conn, &cmd);
        }
    } else {
        // this is an insert
        if contract.is_retired() {
            let mut cmd = String::from(
                "
            INSERT INTO COOP_DATABASE_CONTRACT
            (
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                RETIRED_DATE_UTC,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR
            )
            VALUES
            (
                ':cid',
                ':gen_date',
                ':desc',
                ':ret_date',
                ':vid',
                ':remote_behavior'
            );
            ",
            );
            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            let ret = &contract.retired_date.unwrap().to_string();
            cmd = cmd.replace(":ret_date", ret);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(conn, &cmd);
        } else {
            let mut cmd = String::from(
                "
            INSERT INTO COOP_DATABASE_CONTRACT
            (
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR
            )
            VALUES
            (
                ':cid',
                ':gen_date',
                ':desc',
                ':vid',
                ':remote_behavior'
            );
            ",
            );

            cmd = cmd.replace(":cid", &contract.contract_id.to_string());
            trace!("{}", &contract.generated_date);
            cmd = cmd.replace(":gen_date", &contract.generated_date.to_string());
            cmd = cmd.replace(":desc", &contract.description);
            cmd = cmd.replace(":vid", &contract.version_id.to_string());
            cmd = cmd.replace(
                ":remote_behavior",
                &contract.remote_delete_behavior.to_string(),
            );
            execute_write(conn, &cmd);
        }
    }
}

pub fn get_all_database_contracts(conn: &Connection) -> Vec<CoopDatabaseContract> {
    let mut result: Vec<CoopDatabaseContract> = Vec::new();

    /*
        "CREATE TABLE IF NOT EXISTS COOP_DATABASE_CONTRACT
        (
            CONTRACT_ID CHAR(36) NOT NULL,
            GENERATED_DATE_UTC DATETIME NOT NULL,
            DESCRIPTION VARCHAR(255),
            RETIRED_DATE_UTC DATETIME,
            VERSION_ID CHAR(36) NOT NULL,
            REMOTE_DELETE_BEHAVIOR INT
        );",
    */

    let cmd = String::from(
        "SELECT 
        CONTRACT_ID,
        GENERATED_DATE_UTC,
        DESCRIPTION,
        RETIRED_DATE_UTC,
        VERSION_ID,
        REMOTE_DELETE_BEHAVIOR
    FROM
        COOP_DATABASE_CONTRACT
        ;
        ",
    );

    let table = execute_read(&cmd, conn).unwrap();

    for row in table.rows {
        for val in &row.vals {
            let mut cid = GUID::rand();
            let mut gen_date = Utc::now();
            let mut desc = String::from("");
            let mut is_retired = false;
            let mut ret_date = Utc::now();
            let mut vid = GUID::rand();
            let mut delete_behavior: u32 = 0;

            if val.col.name == "CONTRACT_ID" {
                let vcid = val.data.as_ref().unwrap().data_string.clone();
                let tcid = GUID::parse(&vcid).unwrap();
                cid = tcid;
            }

            if val.col.name == "GENERATED_DATE_UTC" {
                let vgen_date = val.data.as_ref().unwrap().data_string.clone();
                // trace!("{}", vgen_date);
                let tgen_date =
                    Utc::datetime_from_str(&Utc, &vgen_date, defaults::DATETIME_STRING_FORMAT);
                gen_date = tgen_date.unwrap();
            }

            if val.col.name == "DESCRIPTION" {
                let vdesc = val.data.as_ref().unwrap().data_string.clone();
                desc = vdesc.clone();
            }

            if val.col.name == "RETIRED_DATE_UTC" {
                if val.is_null() {
                    is_retired = false;
                } else {
                    let vret_date = val.data.as_ref().unwrap().data_string.clone();
                    if !vret_date.is_empty() {
                        // trace!("{}", vret_date);
                        let tret_date = Utc::datetime_from_str(
                            &Utc,
                            &vret_date,
                            defaults::DATETIME_STRING_FORMAT,
                        );
                        ret_date = tret_date.unwrap();
                        is_retired = true;
                    } else {
                        is_retired = false;
                    }
                }
            }

            if val.col.name == "VERSION_ID" {
                let vvid = val.data.as_ref().unwrap().data_string.clone();
                let tvid = GUID::parse(&vvid).unwrap();
                vid = tvid;
            }

            if val.col.name == "REMOTE_DELETE_BEHAVIOR" {
                let vbehavior = val.data.as_ref().unwrap().data_string.clone();
                delete_behavior = vbehavior.parse().unwrap();
            }

            let item = CoopDatabaseContract {
                contract_id: cid,
                generated_date: gen_date,
                description: desc,
                retired_date: if is_retired { Some(ret_date) } else { None },
                version_id: vid,
                remote_delete_behavior: delete_behavior,
            };

            result.push(item);
        }
    }

    result
}

pub fn update_participant_accepts_contract(
    db_name: &str,
    participant: CoopDatabaseParticipant,
    participant_message: Participant,
    accepted_contract_version_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);

    let internal_id = participant.internal_id;
    let participant_id = participant_message.participant_guid.clone();
    let token = participant_message.token;

    let cmd = String::from(
        "
    UPDATE 
        COOP_PARTICIPANT
    SET 
        CONTRACT_STATUS = 3, 
        ACCEPTED_CONTRACT_VERSION_ID = :cid,
        PARTICIPANT_ID = :pid,
        TOKEN = :token
    WHERE 
        INTERNAL_PARTICIPANT_ID = :iid
    ;
    ",
    );

    let mut statement = conn.prepare(&cmd).unwrap();

    let rows_affected = statement
        .execute(named_params! {
            ":cid" : accepted_contract_version_id.to_string(),
            ":pid" : participant_id,
            ":token" : token,
            ":iid" : internal_id.to_string(),
        })
        .unwrap();

    rows_affected > 0
}


pub fn get_active_contract(db_name: &str, config: DbiConfigSqlite) -> CoopDatabaseContract {

    let conn = &get_db_conn(&config, db_name);

    let cmd = String::from(
        "
        SELECT 
            CONTRACT_ID,
            GENERATED_DATE_UTC,
            DESCRIPTION,
            VERSION_ID,
            REMOTE_DELETE_BEHAVIOR 
        FROM 
            COOP_DATABASE_CONTRACT 
        WHERE 
            RETIRED_DATE_UTC IS NULL
        ;",
    );

    let row_to_active_contract = |contract_id: String,
                                  generated_date_utc: String,
                                  description: String,
                                  version_id: String,
                                  remote_delete_behavior: u32|
     -> Result<CoopDatabaseContract> {
        let contract = CoopDatabaseContract {
            contract_id: GUID::parse(&contract_id).unwrap(),
            generated_date: Utc::datetime_from_str(
                &Utc,
                &generated_date_utc,
                defaults::DATETIME_STRING_FORMAT,
            )
            .unwrap(),
            description,
            retired_date: None,
            version_id: GUID::parse(&version_id).unwrap(),
            remote_delete_behavior,
        };

        Ok(contract)
    };

    let mut results: Vec<CoopDatabaseContract> = Vec::new();

    let mut statement = conn.prepare(&cmd).unwrap();
    let contracts = statement
        .query_and_then([], |row| {
            row_to_active_contract(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
            )
        })
        .unwrap();

    for contract in contracts {
        results.push(contract.unwrap());
    }

    return results.first().unwrap().clone();
}

/// Marks this contract as retired in the database with today's UTC date
pub fn retire_contract(version_id: GUID, conn: &Connection) {
    let mut cmd = String::from("UPDATE COOP_DATABASE_CONTRACT SET RETIRED_DATE_UTC = ':retire_date' WHERE VERSION_ID = ':vid'");
    cmd = cmd.replace(":retire_date", &Utc::now().to_string());
    cmd = cmd.replace(":vid", &version_id.to_string());
    execute_write(conn, &cmd);
}
