use guid_create::GUID;
use rcd_common::{
    coop_database_participant::{CoopDatabaseParticipant, CoopDatabaseParticipantData},
    db::{get_metadata_table_name, DbiConfigSqlite},
    defaults,
};
use rcd_enum::contract_status::ContractStatus;
use rcd_error::rcd_db_error::RcdDbError;
use rcdproto::rcdp::{Participant, ParticipantStatus};
use rusqlite::{named_params, Connection, Result};

use crate::sqlite::{execute_write, get_db_conn, has_any_rows, has_table, sql_text};

/// Creates the COOP_PARTICIPANT table if it does not exist. This holds
/// the participant information that are cooperating with this database.
pub fn create_participant_table(conn: &Connection) {
    let cmd = String::from(
        "CREATE TABLE IF NOT EXISTS COOP_PARTICIPANT
    (
        INTERNAL_PARTICIPANT_ID CHAR(36) NOT NULL,
        ALIAS VARCHAR(50) NOT NULL,
        IP4ADDRESS VARCHAR(25),
        IP6ADDRESS VARCHAR(25),
        PORT INT,
        CONTRACT_STATUS INT,
        ACCEPTED_CONTRACT_VERSION_ID CHAR(36),
        TOKEN BLOB NOT NULL,
        PARTICIPANT_ID CHAR(36),
        HTTP_ADDR VARCHAR(50),
        HTTP_PORT INT
    );",
    );

    conn.execute(&cmd, []).unwrap();
}

pub fn save_participant(participant: CoopDatabaseParticipant, conn: Connection) {
    if has_participant_at_conn(&participant.alias, &conn) {
        // this is an update
        let cmd = String::from(
            "
        UPDATE COOP_PARTICIPANT
        SET
            IP4ADDRESS = ':ip4addr',
            IP6ADDRESS = ':ip6addr',
            PORT = ':db_port',
            CONTRACT_STATUS = ':contract_status',
            ACCEPTED_CONTRACT_VERSION_ID = ':accepted_contract_version',
            TOKEN = ':token',
            PARTICIPANT_ID = ':id',
            HTTP_ADDR = ':http_addr',
            HTTP_PORT = ':http_port',
        WHERE
            ALIAS = ':alias'
        ;
        ",
        );

        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {
                    ":ip4addr": participant.ip4addr,
                    ":ip6addr": participant.ip6addr,
                    ":db_port": participant.db_port.to_string(),
                    ":contract_status": &ContractStatus::to_u32(participant.contract_status),
                    ":accepted_contract_version": &participant.accepted_contract_version.to_string(),
                    ":token": &participant.token,
                    ":id": &participant.id.to_string(),
                    ":alias": &participant.alias,
                    ":http_addr": &participant.http_addr,
                    ":http_port": &participant.http_port,
            })
            .unwrap();
    } else {
        // this is an insert

        // println!("{:?}", &self);

        let cmd = String::from(
            "
        INSERT INTO COOP_PARTICIPANT
        (
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            CONTRACT_STATUS,
            ACCEPTED_CONTRACT_VERSION_ID,
            TOKEN,
            PARTICIPANT_ID,
            HTTP_ADDR,
            HTTP_PORT
        )
        VALUES
        (
            :internal_id,
            :alias,
            :ip4addr,
            :ip6addr,
            :db_port,
            :contract_status,
            :accepted_contract_version,
            :token,
            :id,
            :http_addr,
            :http_port
        );
        ",
        );

        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {
                    ":internal_id": &participant.internal_id.to_string(),
                    ":alias": &participant.alias,
                    ":ip4addr": &participant.ip4addr,
                    ":ip6addr": &participant.ip6addr,
                    ":db_port": &participant.db_port.to_string(),
                    ":contract_status": &ContractStatus::to_u32(participant.contract_status),
                    ":accepted_contract_version": &participant.accepted_contract_version.to_string(),
                    ":token": &participant.token,
                    ":id": &participant.id.to_string(),
                    ":http_addr": &participant.http_addr,
                    ":http_port": &participant.http_port
            })
            .unwrap();
    }
}

pub fn add_participant(
    db_name: &str,
    alias: &str,
    ip4addr: &str,
    db_port: u32,
    config: DbiConfigSqlite,
    http_addr: String,
    http_port: u16,
) -> bool {
    let conn = get_db_conn(&config, db_name);

    let is_added: bool = if has_participant(db_name, alias, config) {
        false
    } else {
        let participant = CoopDatabaseParticipant {
            internal_id: GUID::rand(),
            alias: alias.to_string(),
            ip4addr: ip4addr.to_string(),
            ip6addr: String::from(""),
            db_port,
            contract_status: ContractStatus::NotSent,
            accepted_contract_version: GUID::parse(defaults::EMPTY_GUID).unwrap(),
            id: GUID::parse(defaults::EMPTY_GUID).unwrap(),
            token: Vec::new(),
            http_addr,
            http_port,
        };
        save_participant(participant, conn);
        true
    };

    is_added
}

pub fn get_participant_by_internal_id(
    db_name: &str,
    internal_id: &str,
    config: &DbiConfigSqlite,
) -> CoopDatabaseParticipant {
    let conn = get_db_conn(config, db_name);
    let cmd = String::from(
        "
        SELECT 
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            CONTRACT_STATUS,
            ACCEPTED_CONTRACT_VERSION_ID,
            TOKEN,
            PARTICIPANT_ID,
            HTTP_ADDR,
            HTTP_PORT
        FROM
            COOP_PARTICIPANT
        WHERE
            INTERNAL_PARTICIPANT_ID = :pid
        ;
        ",
    );
    // cmd = cmd.replace(":alias", &alias);

    let row_to_participant = |internal_id: String,
                              alias: String,
                              ip4addr: String,
                              ip6addr: String,
                              port: u32,
                              contract_status: u32,
                              accepted_contract_version_id: String,
                              token: Vec<u8>,
                              id: String,
                              http_addr: String,
                              http_port: u16|
     -> Result<CoopDatabaseParticipant> {
        let participant = CoopDatabaseParticipant {
            internal_id: GUID::parse(&internal_id).unwrap(),
            alias,
            ip4addr,
            ip6addr,
            db_port: port,
            contract_status: ContractStatus::from_i64(contract_status as i64),
            accepted_contract_version: GUID::parse(&accepted_contract_version_id).unwrap(),
            token,
            id: GUID::parse(&id).unwrap(),
            http_addr,
            http_port,
        };

        Ok(participant)
    };

    let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

    let mut statement = conn.prepare(&cmd).unwrap();
    let participants = statement
        .query_and_then(&[(":pid", &internal_id)], |row| {
            row_to_participant(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
                row.get(8).unwrap(),
                row.get(9).unwrap(),
                row.get(10).unwrap(),
            )
        })
        .unwrap();

    for participant in participants {
        results.push(participant.unwrap());
    }

    return results.first().unwrap().clone();
}

pub fn get_participant_by_alias(
    db_name: &str,
    alias: &str,
    config: DbiConfigSqlite,
) -> Option<CoopDatabaseParticipant> {
    let conn = get_db_conn(&config, db_name);
    let cmd = String::from(
        "
        SELECT 
            INTERNAL_PARTICIPANT_ID,
            ALIAS,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            CONTRACT_STATUS,
            ACCEPTED_CONTRACT_VERSION_ID,
            TOKEN,
            PARTICIPANT_ID,
            HTTP_ADDR,
            HTTP_PORT
        FROM
            COOP_PARTICIPANT
        WHERE
            ALIAS = :alias
        ;
        ",
    );
    // cmd = cmd.replace(":alias", &alias);

    // println!("{:?}", cmd);
    // println!("{}", alias);

    let row_to_participant = |internal_id: String,
                              alias: String,
                              ip4addr: String,
                              ip6addr: String,
                              port: u32,
                              contract_status: u32,
                              accepted_contract_version_id: String,
                              token: Vec<u8>,
                              id: String,
                              http_addr: String,
                              http_port: u16|
     -> Result<CoopDatabaseParticipant> {
        let participant = CoopDatabaseParticipant {
            internal_id: GUID::parse(&internal_id).unwrap(),
            alias,
            ip4addr,
            ip6addr,
            db_port: port,
            contract_status: ContractStatus::from_i64(contract_status as i64),
            accepted_contract_version: GUID::parse(&accepted_contract_version_id).unwrap(),
            token,
            id: GUID::parse(&id).unwrap(),
            http_addr,
            http_port,
        };

        Ok(participant)
    };

    let mut results: Vec<CoopDatabaseParticipant> = Vec::new();

    let mut statement = conn.prepare(&cmd).unwrap();
    let participants = statement
        .query_and_then(&[(":alias", &alias)], |row| {
            row_to_participant(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
                row.get(8).unwrap(),
                row.get(9).unwrap(),
                row.get(10).unwrap(),
            )
        })
        .unwrap();

    for participant in participants {
        results.push(participant.unwrap());
    }

    if !results.is_empty() {
        Some(results.first().unwrap().clone())
    } else {
        None
    }
}

pub fn get_participants_for_database(
    db_name: &str,
    config: &DbiConfigSqlite,
) -> core::result::Result<Vec<ParticipantStatus>, RcdDbError> {
    let mut result: Vec<ParticipantStatus> = Vec::new();

    let conn = get_db_conn(config, db_name);

    // if the table doesn't exist, we should return an error here
    if !has_table("COOP_PARTICIPANT", &conn) {
        return Err(RcdDbError::TableNotFoundInDatabase(
            "COOP_PARTICIPANT".to_string(),
            db_name.to_string(),
        ));
    }

    let cmd = "
    SELECT 
        INTERNAL_PARTICIPANT_ID,
        ALIAS,
        IP4ADDRESS,
        IP6ADDRESS,
        PORT,
        CONTRACT_STATUS,
        PARTICIPANT_ID,
        HTTP_ADDR,
        HTTP_PORT
    FROM
        COOP_PARTICIPANT
    ";

    let mut statement = conn.prepare(cmd).unwrap();

    let row_to_participant = |internal_participant_id: String,
                              alias: String,
                              ip4: String,
                              ip6: String,
                              port: u32,
                              contract_status: u32,
                              participant_id: String,
                              http_addr: String,
                              http_port: u32|
     -> Result<ParticipantStatus> {
        let p = Participant {
            participant_guid: participant_id,
            alias,
            ip4_address: ip4,
            ip6_address: ip6,
            database_port_number: port,
            token: Vec::new(),
            internal_participant_guid: internal_participant_id,
            http_addr,
            http_port,
        };

        let ps = ParticipantStatus {
            participant: Some(p),
            contract_status,
        };

        Ok(ps)
    };

    let participants = statement
        .query_and_then([], |row| {
            row_to_participant(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
                row.get(8).unwrap(),
            )
        })
        .unwrap();

    for p in participants {
        result.push(p.unwrap());
    }

    Ok(result)
}

pub fn get_participants_for_table(
    db_name: &str,
    table_name: &str,
    config: DbiConfigSqlite,
) -> Vec<CoopDatabaseParticipantData> {
    // note - we will need another table to track the remote row id
    // let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);

    let conn = get_db_conn(&config, db_name);
    let metadata_table_name = get_metadata_table_name(table_name);

    if !has_table(&metadata_table_name, &conn) {
        //  need to create table
        let mut cmd = sql_text::Coop::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        execute_write(&conn, &cmd);
    }

    let conn = get_db_conn(&config, db_name);

    let mut result: Vec<CoopDatabaseParticipantData> = Vec::new();

    let mut cmd = String::from(
        "
        SELECT DISTINCT
            INTERNAL_PARTICIPANT_ID
        FROM
            :table_name
        ;",
    );
    cmd = cmd.replace(":table_name", &metadata_table_name);

    let mut statement = conn.prepare(&cmd).unwrap();
    let mut participant_ids: Vec<String> = Vec::new();
    let mut db_participants: Vec<CoopDatabaseParticipant> = Vec::new();

    let row_to_id = |participant_id: String| -> Result<String> { Ok(participant_id) };

    let participants = statement
        .query_and_then([], |row| row_to_id(row.get(0).unwrap()))
        .unwrap();

    for p in participants {
        participant_ids.push(p.unwrap());
    }

    for pid in &participant_ids {
        let participant = get_participant_by_internal_id(db_name, pid, &config);
        db_participants.push(participant);
    }

    let row_to_data = |row_id: u32, hash: Vec<u8>| -> Result<(u32, Vec<u8>)> { Ok((row_id, hash)) };

    for p in &db_participants {
        cmd = String::from(
            "
            SELECT
                ROW_ID,
                HASH
            FROM
                :table_name
            WHERE
                INTERNAL_PARTICIPANT_ID = ':pid'
            ;",
        );
        cmd = cmd.replace(":table_name", &metadata_table_name);
        cmd = cmd.replace(":pid", &p.internal_id.to_string());

        statement = conn.prepare(&cmd).unwrap();

        let row_data = statement
            .query_and_then([], |row| {
                row_to_data(row.get(0).unwrap(), row.get(1).unwrap())
            })
            .unwrap();

        let mut row_data_results: Vec<(u32, Vec<u8>)> = Vec::new();

        for data in row_data {
            row_data_results.push(data.unwrap());
        }

        let participant_data = CoopDatabaseParticipantData {
            participant: p.clone(),
            db_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_data: row_data_results,
        };

        result.push(participant_data);
    }

    result
}

pub fn has_participant_at_conn(alias: &str, conn: &Connection) -> bool {
    let mut cmd =
        String::from("SELECT COUNT(*) TOTALCOUNT FROM COOP_PARTICIPANT WHERE ALIAS = ':alias'");
    cmd = cmd.replace(":alias", alias);
    has_any_rows(cmd, conn)
}

pub fn has_participant(db_name: &str, alias: &str, config: DbiConfigSqlite) -> bool {
    let conn = &get_db_conn(&config, db_name);
    let mut cmd =
        String::from("SELECT COUNT(*) TOTALCOUNT FROM COOP_PARTICIPANT WHERE ALIAS = ':alias'");
    cmd = cmd.replace(":alias", alias);
    has_any_rows(cmd, conn)
}
