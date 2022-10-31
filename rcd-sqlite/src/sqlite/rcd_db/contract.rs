use rcdproto::rcdp::{ColumnSchema, Contract, DatabaseSchema, Host, TableSchema};

use crate::sqlite::{execute_write, get_scalar_as_string, has_any_rows};

use super::{get_rcd_conn, has_contract};
use chrono::Utc;
use rcd_common::{
    db::{CdsContracts, CdsContractsTables, CdsContractsTablesColumns, CdsHosts, DbiConfigSqlite},
    rcd_enum::ContractStatus,
};
use rusqlite::{named_params, Connection, Result};

pub fn accept_pending_contract(host_name: &str, config: &DbiConfigSqlite) -> bool {
    let conn = get_rcd_conn(config);

    let mut cmd = String::from("SELECT HOST_ID FROM CDS_HOSTS WHERE HOST_NAME = ':hostname'");
    cmd = cmd.replace(":hostname", host_name);

    let db_host_id = get_scalar_as_string(cmd, &conn);
    cmd = String::from(
        "SELECT COUNT(*) TOTALCOUNT FROM CDS_CONTRACTS WHERE HOST_ID = ':hid'
    AND CONTRACT_STATUS = 2",
    );
    cmd = cmd.replace(":hid", &db_host_id);

    let has_pending_contract = has_any_rows(cmd, &conn);

    if has_pending_contract {
        // 1 - we need to update the rcd_db record that we are accepting this contract
        // 2 - then we actually need to create the database with the properties of the
        // contract
        // 3 - we need to notify the host that we have accepted the contract

        cmd = String::from(
            "SELECT CONTRACT_ID FROM CDS_CONTRACTS WHERE HOST_ID = ':hid' AND CONTRACT_STATUS = 2",
        );
        cmd = cmd.replace(":hid", &db_host_id);

        let cid = get_scalar_as_string(cmd, &conn);

        cmd =
            String::from("UPDATE CDS_CONTRACTS SET CONTRACT_STATUS = 3 WHERE CONTRACT_ID = ':cid'");
        cmd = cmd.replace(":cid", &cid);

        let total_count = execute_write(&conn, &cmd);
        return total_count > 0;
    }

    return false;
}

pub fn get_pending_contracts(config: &DbiConfigSqlite) -> Vec<Contract> {
    let conn = get_rcd_conn(config);

    let pending_status = ContractStatus::to_u32(ContractStatus::Pending);

    let cmd = String::from(
        "
        SELECT 
            HOST_ID,
            CONTRACT_ID,
            CONTRACT_VERSION_ID,
            DATABASE_NAME,
            DATABASE_ID,
            DESCRIPTION,
            GENERATED_DATE_UTC,
            CONTRACT_STATUS 
        FROM 
            CDS_CONTRACTS 
        WHERE 
            CONTRACT_STATUS = :pending",
    );

    let mut statement = conn.prepare(&cmd).unwrap();

    let mut pending_contracts: Vec<Contract> = Vec::new();

    let mut cds_contracts: Vec<CdsContracts> = Vec::new();
    let mut cds_tables: Vec<CdsContractsTables> = Vec::new();
    let mut cds_tables_columns: Vec<CdsContractsTablesColumns> = Vec::new();

    let row_to_contract = |host_id: String,
                           contract_id: String,
                           contract_version_id: String,
                           database_name: String,
                           database_id: String,
                           description: String,
                           gen_date: String,
                           status: u32|
     -> Result<CdsContracts> {
        let cds_contract = CdsContracts {
            host_id,
            contract_id,
            contract_version_id,
            database_name,
            database_id,
            description,
            generated_date: gen_date,
            contract_status: ContractStatus::from_u32(status),
        };

        Ok(cds_contract)
    };

    let contract_metadata = statement
        .query_and_then(&[(":pending", &pending_status.to_string())], |row| {
            row_to_contract(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
                row.get(7).unwrap(),
            )
        })
        .unwrap();

    for c in contract_metadata {
        cds_contracts.push(c.unwrap());
    }

    for cdata in &cds_contracts {
        let dbid = cdata.database_id.clone();

        let cmd = String::from(
            "
        SELECT 
            DATABASE_ID,
            DATABASE_NAME,
            TABLE_ID,
            TABLE_NAME,
            LOGICAL_STORAGE_POLICY
        FROM 
            CDS_CONTRACTS_TABLES 
        WHERE 
            DATABASE_ID = :dbid",
        );

        let row_to_table = |database_id: String,
                            database_name: String,
                            table_id: String,
                            table_name: String,
                            logical_storage_policy: u32|
         -> Result<CdsContractsTables> {
            let table = CdsContractsTables {
                database_id: database_id,
                database_name: database_name,
                table_id,
                table_name,
                logical_storage_policy,
            };
            Ok(table)
        };

        statement = conn.prepare(&cmd).unwrap();

        let table_metadata = statement
            .query_and_then(&[(":dbid", &dbid)], |row| {
                row_to_table(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                )
            })
            .unwrap();

        for table in table_metadata {
            cds_tables.push(table.unwrap());
        }
    }

    for table in &cds_tables {
        let tid = table.table_id.clone();

        let cmd = String::from(
            "
        SELECT 
            TABLE_ID,
            COLUMN_ID,
            COLUMN_NAME,
            COLUMN_TYPE,
            COLUMN_LENGTH,
            COLUMN_ORDINAL,
            IS_NULLABLE 
        FROM 
        CDS_CONTRACTS_TABLE_SCHEMAS 
        WHERE 
            TABLE_ID = :tid",
        );

        statement = conn.prepare(&cmd).unwrap();

        let row_to_column = |table_id: String,
                             column_id: String,
                             column_name: String,
                             column_type: u32,
                             column_length: u32,
                             column_ordinal: u32,
                             is_nullable: bool|
         -> Result<CdsContractsTablesColumns> {
            let col = CdsContractsTablesColumns {
                table_id,
                column_id,
                column_name,
                column_type,
                column_length,
                column_ordinal,
                is_nullable,
            };
            Ok(col)
        };

        let table_columns = statement
            .query_and_then(&[(":tid", &tid)], |row| {
                row_to_column(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                )
            })
            .unwrap();

        for column in table_columns {
            cds_tables_columns.push(column.unwrap());
        }
    }

    let mut cds_host_infos: Vec<CdsHosts> = Vec::new();

    let cmd = String::from(
        "
        SELECT 
            HOST_ID,
            HOST_NAME,
            TOKEN,
            IP4ADDRESS,
            IP6ADDRESS,
            PORT,
            LAST_COMMUNICATION_UTC
        FROM
            CDS_HOSTS
        WHERE
            HOST_ID = :hid
    ;",
    );

    statement = conn.prepare(&cmd).unwrap();

    let row_to_host = |host_id: String,
                       host_name: String,
                       token: Vec<u8>,
                       ip4: String,
                       ip6: String,
                       port: u32,
                       last_comm_utc: String|
     -> Result<CdsHosts> {
        let host = CdsHosts {
            host_id,
            host_name,
            token,
            ip4,
            ip6,
            port,
            last_comm_utc,
        };
        Ok(host)
    };

    for c in &cds_contracts {
        let h = c.host_id.clone();

        let table_hosts = statement
            .query_and_then(&[(":hid", &h)], |row| {
                row_to_host(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                )
            })
            .unwrap();

        for h in table_hosts {
            cds_host_infos.push(h.unwrap());
        }
    }

    let mut db_schema: Vec<DatabaseSchema> = Vec::new();

    for contract in &cds_contracts {
        let dbid = contract.database_id.clone();
        let tables = cds_tables
            .iter()
            .enumerate()
            .filter(|&(_, t)| t.database_id == dbid)
            .map(|(_, t)| t);

        let mut table_schema: Vec<TableSchema> = Vec::new();

        for t in tables {
            let mut col_schema: Vec<ColumnSchema> = Vec::new();

            let tid = t.table_id.clone();
            let cols = cds_tables_columns
                .iter()
                .enumerate()
                .filter(|&(_, c)| c.table_id == tid)
                .map(|(_, c)| c);

            for c in cols {
                let cs = ColumnSchema {
                    column_name: c.column_name.clone(),
                    column_type: c.column_type,
                    column_length: c.column_length,
                    is_nullable: c.is_nullable,
                    ordinal: c.column_ordinal,
                    table_id: c.table_id.clone(),
                    column_id: c.column_id.clone(),
                    is_primary_key: false,
                };
                col_schema.push(cs);
            }

            let ts = TableSchema {
                table_name: t.table_name.clone(),
                table_id: t.table_id.clone(),
                database_name: t.database_name.clone(),
                database_id: t.database_id.clone(),
                columns: col_schema,
                logical_storage_policy: t.logical_storage_policy,
            };

            table_schema.push(ts);
        }

        let ds = DatabaseSchema {
            database_name: contract.database_name.clone(),
            database_id: contract.database_id.clone(),
            tables: table_schema,
            database_type: 0,
            rcd_database_type: 0,
        };

        db_schema.push(ds);
    }

    for c in &cds_contracts {
        let dbs = db_schema
            .iter()
            .enumerate()
            .filter(|&(_, s)| s.database_id == c.database_id)
            .map(|(_, s)| s);

        let hi = cds_host_infos
            .iter()
            .enumerate()
            .filter(|&(_, h)| h.host_id == c.host_id)
            .map(|(_, h)| h);

        let h = hi.last().unwrap().clone();

        let i = Host {
            host_guid: h.host_id.clone(),
            host_name: h.host_name.clone(),
            ip4_address: h.ip4.clone(),
            ip6_address: h.ip6.clone(),
            database_port_number: h.port,
            token: h.token.clone(),
        };

        let pc = Contract {
            contract_guid: c.contract_id.clone(),
            description: c.description.clone(),
            schema: Some(dbs.last().unwrap().clone()),
            contract_version: c.contract_version_id.clone(),
            host_info: Some(i.clone()),
            status: ContractStatus::to_u32(c.contract_status),
        };

        pending_contracts.push(pc);
    }

    return pending_contracts;
}

/// Saves a contract sent from a host to our local rcd_db instance. This lets us
/// later review the contract for us to accept or reject it. If we accept it
/// this means that we'll create a partial database with the contract's schema
/// and also notify the host that we are willing to be a participant of the database.
#[allow(dead_code, unused_variables)]
pub fn save_contract(contract: Contract, config: &DbiConfigSqlite) -> bool {
    let conn = get_rcd_conn(config);

    // println!("save_contract called with {:?}", contract);

    if !has_contract(&contract.contract_guid, &conn) {
        save_contract_metadata(&contract, &conn);
        save_contract_table_data(&contract, &conn);
        save_contract_table_schema_data(&contract, &conn);
        save_contract_host_data(&contract, &conn);
        return true;
    }

    return false;
}

/// saves a contract's table information to CDS_CONTRACTS_TABLES
fn save_contract_table_data(contract: &Contract, conn: &Connection) {
    // println!("save_contract_table_data: connection: {:?}", conn);

    let cmd = String::from(
        "INSERT INTO CDS_CONTRACTS_TABLES
    (
        DATABASE_ID,
        DATABASE_NAME,
        TABLE_ID,
        TABLE_NAME,
        LOGICAL_STORAGE_POLICY,
        UPDATES_FROM_HOST_BEHAVIOR,
        DELETES_FROM_HOST_BEHAVIOR,
        UPDATES_TO_HOST_BEHAVIOR,
        DELETES_TO_HOST_BEHAVIOR,
        USE_DATA_LOG_TABLE
    )
    VALUES
    (
        :dbid,
        :dbname,
        :tid,
        :tname,
        :policy,
        1,
        1,
        1,
        1,
        0
    )
    ;
    ",
    );

    let schema = contract.schema.as_ref().unwrap();

    let db_name = schema.database_name.clone();
    let db_id = schema.database_id.clone();

    for t in &schema.tables {
        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {
                ":dbid": &db_id,
                ":dbname" : &db_name,
                ":tid" : &t.table_id,
                ":tname" : &t.table_name,
                ":policy" : &t.logical_storage_policy
            })
            .unwrap();
    }
}

/// saves top level contract data to rcd_db's CDS_CONTRACTS table
fn save_contract_metadata(contract: &Contract, conn: &Connection) {
    let host = contract.host_info.as_ref().clone().unwrap().clone();
    let db = contract.schema.as_ref().clone().unwrap().clone();

    let cmd = String::from(
        "INSERT INTO CDS_CONTRACTS
    (
        HOST_ID,
        CONTRACT_ID,
        CONTRACT_VERSION_ID,
        DATABASE_NAME,
        DATABASE_ID,
        DESCRIPTION,
        GENERATED_DATE_UTC,
        CONTRACT_STATUS
    )
    VALUES
    (
        :hid,
        :cid,
        :cvid,
        :dbname,
        :dbid,
        :desc,
        :gdutc,
        :status
    )
    ;",
    );

    let mut statement = conn.prepare(&cmd).unwrap();
    statement
        .execute(named_params! {
            ":hid": host.host_guid.to_string(),
            ":cid" : contract.contract_guid,
            ":cvid" : contract.contract_version,
            ":dbname" : db.database_name,
            ":dbid" : db.database_id,
            ":desc" : contract.description,
            ":gdutc" : Utc::now().to_string(),
            ":status" : contract.status.to_string()
        })
        .unwrap();
}

/// save's a contract's table schema information to CDS_CONTRACTS_TABLE_SCHEMAS
fn save_contract_table_schema_data(contract: &Contract, conn: &Connection) {
    let tables = contract.schema.as_ref().unwrap().tables.clone();

    for table in &tables {
        let cmd = String::from(
            "INSERT INTO CDS_CONTRACTS_TABLE_SCHEMAS
        (
            TABLE_ID,
            COLUMN_ID,
            COLUMN_NAME,
            COLUMN_TYPE,
            COLUMN_LENGTH,
            COLUMN_ORDINAL,
            IS_NULLABLE
        )
        VALUES
        (
            :tid,
            :cid,
            :cname,
            :ctype,
            :clength,
            :cordinal,
            :is_nullable
        )
        ;",
        );

        let tid = table.table_id.clone();
        for column in &table.columns {
            let cid = column.column_id.clone();
            let cname = column.column_name.clone();
            let ctype = column.column_type;
            let clength = column.column_length;
            let cordinal = column.ordinal;
            let is_nullable = if column.is_nullable { 1 } else { 0 };

            let mut statement = conn.prepare(&cmd).unwrap();
            statement
                .execute(named_params! {
                    ":tid": tid,
                    ":cid" : cid,
                    ":cname" : cname,
                    ":ctype" : ctype,
                    ":clength" : clength,
                    ":cordinal" : cordinal,
                    ":is_nullable" : is_nullable,
                })
                .unwrap();
        }
    }
}

// save a contract's host information to CDS_HOSTS
fn save_contract_host_data(contract: &Contract, conn: &Connection) {
    let cmd = String::from(
        "INSERT INTO CDS_HOSTS
    (
        HOST_ID,
        HOST_NAME,
        TOKEN,
        IP4ADDRESS,
        IP6ADDRESS,
        PORT,
        LAST_COMMUNICATION_UTC,
        HOST_STATUS
    )
    VALUES
    (
        :hid,
        :hname,
        :token,
        :ip4,
        :ip6,
        :port,
        :last_comm,
        1
    )
    ;",
    );

    let host = contract.host_info.as_ref().unwrap().clone();

    let mut statement = conn.prepare(&cmd).unwrap();
    statement
        .execute(named_params! {
            ":hid": &host.host_guid,
            ":hname" : &host.host_name,
            ":token" : &host.token,
            ":ip4" : &host.ip4_address,
            ":ip6" : &host.ip6_address,
            ":port" : &host.database_port_number,
            ":last_comm" : Utc::now().to_string()
        })
        .unwrap();
}
