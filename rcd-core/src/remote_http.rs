/*

Represents a HTTP client talking to a remote Data Service endpoint.

*/

use chrono::Utc;
use endianness::{read_i32, ByteOrder};
use guid_create::GUID;
use log::info;
use rcd_common::{
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::CoopDatabaseParticipant, host_info::HostInfo,
    rcd_enum::ContractStatus,
};
use rcd_http_common::url::data::{PARTICIPANT_ACCEPTS_CONTRACT, SAVE_CONTRACT};
use rcdproto::rcdp::{
    Contract, DatabaseSchema, MessageInfo, Participant, ParticipantAcceptsContractRequest,
    ParticipantAcceptsContractResult, SaveContractRequest, SaveContractResult,
};

#[derive(Debug, Clone)]
pub struct RemoteHttp {
    pub own_http_addr: String,
    pub own_http_port: u32,
}

impl RemoteHttp {
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
