/*

Represents a HTTP client talking to a remote Data Service endpoint.

*/

use chrono::Utc;
use endianness::{read_i32, ByteOrder};
use guid_create::GUID;
use log::info;
use rcd_common::{
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::{CoopDatabaseParticipant, CoopDatabaseParticipantData},
    db::CdsHosts,
    host_info::HostInfo,
    rcd_enum::ContractStatus,
};
use rcd_http_common::url::data::{
    GET_ROW_AT_PARTICIPANT, INSERT_ROW_AT_PARTICIPANT, NOTIFY_HOST_OF_REMOVED_ROW,
    NOTIFY_HOST_OF_UPDATED_HASH, PARTICIPANT_ACCEPTS_CONTRACT, REMOVE_ROW_AT_PARTICIPANT,
    SAVE_CONTRACT, TRY_AUTH, UPDATE_ROW_AT_PARTICIPANT,
};
use rcdproto::rcdp::{
    AuthRequest, Contract, DatabaseSchema, DeleteDataRequest, DeleteDataResult,
    GetRowFromPartialDatabaseRequest, GetRowFromPartialDatabaseResult, Host, InsertDataRequest,
    InsertDataResult, MessageInfo, NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResponse,
    Participant, ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult,
    RowParticipantAddress, SaveContractRequest, SaveContractResult, TryAuthRequest, TryAuthResult,
    UpdateDataRequest, UpdateDataResult, UpdateRowDataHashForHostRequest,
    UpdateRowDataHashForHostResponse,
};

#[derive(Debug, Clone)]
pub struct RemoteHttp {
    pub own_http_addr: String,
    pub own_http_port: u32,
}

impl RemoteHttp {
    pub async fn notify_host_of_updated_hash(
        &self,
        host: &CdsHosts,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        row_id: u32,
        hash: Option<u64>,
        is_deleted: bool,
    ) -> bool {
        let auth = get_auth_request(own_host_info);
        let message_info = get_message_info(&own_host_info, "".to_string());

        let chost = Host {
            host_guid: own_host_info.id.clone(),
            host_name: own_host_info.name.clone(),
            ip4_address: String::from(""),
            ip6_address: String::from(""),
            database_port_number: 0,
            token: own_host_info.token.clone(),
            http_addr: "".to_string(),
            http_port: 0,
        };

        let hash_val = match hash {
            Some(_) => hash.unwrap(),
            None => 0,
        };

        if !is_deleted {
            let request = UpdateRowDataHashForHostRequest {
                authentication: Some(auth),
                message_info: Some(message_info),
                host_info: Some(chost),
                database_name: db_name.to_string(),
                database_id: String::from(""),
                table_name: table_name.to_string(),
                table_id: 0,
                row_id,
                updated_hash_value: hash_val,
                is_deleted_at_participant: is_deleted,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let addr_port = format!("{}:{}", host.http_addr, host.http_port.to_string());

            info!("sending request to rcd at: {}", addr_port);

            let url = format!("http://{}{}", addr_port, NOTIFY_HOST_OF_UPDATED_HASH);
            let result = send_message(request_json, url).await;
            let reply: UpdateRowDataHashForHostResponse =
                serde_json::from_str(&result.to_string()).unwrap();

            return reply.is_successful;
        } else {
            let request = NotifyHostOfRemovedRowRequest {
                authentication: Some(auth),
                message_info: Some(message_info),
                host_info: Some(chost),
                database_name: db_name.to_string(),
                database_id: String::from(""),
                table_name: table_name.to_string(),
                table_id: 0,
                row_id,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let addr_port = format!("{}:{}", host.http_addr, host.http_port.to_string());

            info!("sending request to rcd at: {}", addr_port);

            let url = format!("http://{}{}", addr_port, NOTIFY_HOST_OF_REMOVED_ROW);
            let result = send_message(request_json, url).await;
            let reply: NotifyHostOfRemovedRowResponse =
                serde_json::from_str(&result.to_string()).unwrap();

            return reply.is_successful;
        }
    }

    pub async fn get_row_from_participant(
        &self,
        participant: CoopDatabaseParticipantData,
        own_host_info: HostInfo,
    ) -> GetRowFromPartialDatabaseResult {
        let message_info = get_message_info(&own_host_info, "".to_string());
        let auth = get_auth_request(&own_host_info);

        let row_address = RowParticipantAddress {
            database_name: participant.db_name.clone(),
            table_name: participant.table_name.clone(),
            row_id: participant.row_data.first().unwrap().0,
        };

        let request = GetRowFromPartialDatabaseRequest {
            authentication: Some(auth),
            row_address: Some(row_address),
            message_info: Some(message_info),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!(
            "{}:{}",
            participant.participant.http_addr,
            participant.participant.http_port.to_string()
        );

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, GET_ROW_AT_PARTICIPANT);
        let result = send_message(request_json, url).await;
        let reply: GetRowFromPartialDatabaseResult =
            serde_json::from_str(&result.to_string()).unwrap();

        return reply;
    }

    pub async fn insert_row_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        sql: &str,
    ) -> InsertDataResult {
        let auth = get_auth_request(own_host_info);

        let request = InsertDataRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!(
            "{}:{}",
            participant.http_addr,
            participant.http_port.to_string()
        );

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, INSERT_ROW_AT_PARTICIPANT);
        let result = send_message(request_json, url).await;
        let reply: InsertDataResult = serde_json::from_str(&result.to_string()).unwrap();

        return reply;
    }

    pub async fn update_row_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        sql: &str,
        where_clause: &str,
    ) -> UpdateDataResult {
        let auth = get_auth_request(own_host_info);

        let request = UpdateDataRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
            where_clause: where_clause.to_string(),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!(
            "{}:{}",
            participant.http_addr,
            participant.http_port.to_string()
        );

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, UPDATE_ROW_AT_PARTICIPANT);
        let result = send_message(request_json, url).await;
        let reply: UpdateDataResult = serde_json::from_str(&result.to_string()).unwrap();

        return reply;
    }

    pub async fn try_auth_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
    ) -> bool {
        let auth = get_auth_request(own_host_info);
        let request = TryAuthRequest {
            authentication: Some(auth),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!(
            "{}:{}",
            participant.http_addr,
            participant.http_port.to_string()
        );

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, TRY_AUTH);
        let result = send_message(request_json, url).await;
        let reply: TryAuthResult = serde_json::from_str(&result.to_string()).unwrap();

        return reply.authentication_result.unwrap().is_authenticated;
    }

    pub async fn notify_host_of_removed_row(
        &self,
        host: &CdsHosts,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> bool {
        let auth = get_auth_request(own_host_info);
        let message_info = get_message_info(&own_host_info, "".to_string());

        let chost = Host {
            host_guid: own_host_info.id.clone(),
            host_name: own_host_info.name.clone(),
            ip4_address: String::from(""),
            ip6_address: String::from(""),
            database_port_number: 0,
            token: own_host_info.token.clone(),
            http_addr: "".to_string(),
            http_port: 0,
        };

        let request = NotifyHostOfRemovedRowRequest {
            authentication: Some(auth),
            message_info: Some(message_info),
            host_info: Some(chost),
            database_name: db_name.to_string(),
            database_id: String::from(""),
            table_name: table_name.to_string(),
            table_id: 0,
            row_id,
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!("{}:{}", host.http_addr, host.http_port.to_string());

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, NOTIFY_HOST_OF_REMOVED_ROW);
        let result = send_message(request_json, url).await;
        let reply: NotifyHostOfRemovedRowResponse =
            serde_json::from_str(&result.to_string()).unwrap();

        return reply.is_successful;
    }

    pub async fn remove_row_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        sql: &str,
        where_clause: &str,
    ) -> DeleteDataResult {
        let auth = get_auth_request(own_host_info);

        let request = DeleteDataRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            cmd: sql.to_string(),
            where_clause: where_clause.to_string(),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!(
            "{}:{}",
            participant.http_addr,
            participant.http_port.to_string()
        );

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, REMOVE_ROW_AT_PARTICIPANT);
        let result = send_message(request_json, url).await;
        let reply: DeleteDataResult = serde_json::from_str(&result.to_string()).unwrap();

        return reply;
    }

    pub async fn notify_host_of_acceptance_of_contract(
        &self,
        accepted_contract: &Contract,
        own_host_info: &HostInfo,
    ) -> bool {
        let message_info = get_message_info(&own_host_info, "".to_string());
        let host_info = accepted_contract.host_info.as_ref().unwrap().clone();

        let participant = Participant {
            participant_guid: own_host_info.id.clone(),
            alias: own_host_info.name.clone(),
            ip4_address: self.own_http_addr.clone(),
            ip6_address: String::from(""),
            database_port_number: 0,
            token: own_host_info.token.clone(),
            internal_participant_guid: "".to_string(),
            http_addr: self.own_http_addr.clone(),
            http_port: self.own_http_port,
        };

        let request = ParticipantAcceptsContractRequest {
            participant: Some(participant),
            contract_guid: accepted_contract.contract_guid.clone(),
            contract_version_guid: accepted_contract.contract_version.clone(),
            database_name: accepted_contract
                .schema
                .as_ref()
                .unwrap()
                .database_name
                .clone(),
            message_info: Some(message_info),
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let addr_port = format!(
            "{}:{}",
            host_info.http_addr,
            host_info.http_port.to_string()
        );

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, PARTICIPANT_ACCEPTS_CONTRACT);
        let result = send_message(request_json, url).await;
        let reply: ParticipantAcceptsContractResult =
            serde_json::from_str(&result.to_string()).unwrap();

        return reply.contract_acceptance_is_acknowledged;
    }

    pub async fn send_participant_contract(
        &self,
        participant: CoopDatabaseParticipant,
        host_info: HostInfo,
        contract: CoopDatabaseContract,
        db_schema: DatabaseSchema,
    ) -> bool {
        let message_info = get_message_info(&host_info, "".to_string());

        let contract = contract.to_cdata_contract(
            &host_info,
            "",
            "",
            0,
            ContractStatus::Pending,
            db_schema,
            &self.own_http_addr,
            self.own_http_port,
        );

        let request = SaveContractRequest {
            contract: Some(contract),
            message_info: Some(message_info),
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let addr_port = format!(
            "{}:{}",
            participant.http_addr,
            participant.http_port.to_string()
        );

        info!("sending request to rcd at: {}", addr_port);

        let url = format!("http://{}{}", addr_port, SAVE_CONTRACT);
        let result = send_message(request_json, url).await;
        let reply: SaveContractResult = serde_json::from_str(&result.to_string()).unwrap();

        /*
        let http_response = reqwest::new(&url)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
            */

        return reply.is_saved;
    }
}

fn get_message_info(host_info: &HostInfo, own_db_addr_port: String) -> MessageInfo {
    let mut addresses: Vec<String> = Vec::new();

    addresses.push(host_info.id.clone());
    addresses.push(host_info.name.clone());
    addresses.push(own_db_addr_port);

    let is_little_endian = is_little_endian();

    let message_info = MessageInfo {
        is_little_endian: is_little_endian,
        message_addresses: addresses,
        message_generated_time_utc: Utc::now().to_string(),
        message_guid: GUID::rand().to_string(),
    };

    return message_info;
}

fn is_little_endian() -> bool {
    let v = vec![0, 128, 128, 0];

    let result = match read_i32(&v, ByteOrder::LittleEndian) {
        Ok(_n) => true,
        Err(_err) => false,
    };

    return result;
}

async fn send_message(json_message: String, url: String) -> String {
    let client = reqwest::Client::new();

    println!("{}", json_message);
    println!("{}", url);

    return client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_message)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}

fn get_auth_request(own_host_info: &HostInfo) -> AuthRequest {
    let auth = AuthRequest {
        user_name: own_host_info.name.clone(),
        pw: String::from(""),
        pw_hash: Vec::new(),
        token: own_host_info.token.clone(),
    };

    return auth;
}
