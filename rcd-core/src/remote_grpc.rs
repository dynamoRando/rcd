/*

Represents a gRPC client talking to a remote Data Service endpoint.

*/

use chrono::Utc;
use endianness::{read_i32, ByteOrder};
use guid_create::GUID;
use log::info;
use rcd_common::{host_info::HostInfo, coop_database_participant::CoopDatabaseParticipant};
use rcdproto::rcdp::{Contract, Participant, ParticipantAcceptsContractRequest, data_service_client::DataServiceClient, AuthRequest, MessageInfo, Host, UpdateRowDataHashForHostRequest, NotifyHostOfRemovedRowRequest};
use tonic::transport::Channel;

use rcd_common::db::CdsHosts;



#[derive(Debug, Clone)]
pub struct RemoteGrpc{
    pub db_addr_port: String,
}

impl RemoteGrpc {


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
    let message_info = get_message_info(own_host_info, self.db_addr_port.clone());

    let chost = Host {
        host_guid: own_host_info.id.clone(),
        host_name: own_host_info.name.clone(),
        ip4_address: String::from(""),
        ip6_address: String::from(""),
        database_port_number: 0,
        token: own_host_info.token.clone(),
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

        let client = get_client_from_cds_host(host);
        let response = client.await.update_row_data_hash_for_host(request).await;
        let result = response.unwrap().into_inner();
        return result.is_successful;
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

        let client = get_client_from_cds_host(host);
        let response = client.await.notify_host_of_removed_row(request).await;
        let result = response.unwrap().into_inner();
        return result.is_successful;
    }
}


    pub async fn notify_host_of_acceptance_of_contract(
        &self,
        accepted_contract: &Contract,
        own_host_info: &HostInfo,
    ) -> bool {
        // rpc AcceptContract(ParticipantAcceptsContractRequest) returns (ParticipantAcceptsContractResult);
    
        let message_info = get_message_info(own_host_info, self.db_addr_port.clone());
        let host_info = accepted_contract.host_info.as_ref().unwrap().clone();
    
        let participant = Participant {
            participant_guid: own_host_info.id.clone(),
            alias: own_host_info.name.clone(),
            ip4_address: self.db_addr_port.clone(),
            ip6_address: String::from(""),
            database_port_number: 0,
            token: own_host_info.token.clone(),
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
    
        let message = format!(
            "sending request to rcd at: {}",
            host_info.ip4_address.clone()
        );
        info!("{}", message);
        println!("{}", message);
    
        let client = get_client_with_addr_port(host_info.ip4_address.clone());
        let response = client.await.accept_contract(request).await.unwrap();
    
        return response.into_inner().contract_acceptance_is_acknowledged;
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

fn get_auth_request(own_host_info: &HostInfo) -> AuthRequest {
    let auth = AuthRequest {
        user_name: own_host_info.name.clone(),
        pw: String::from(""),
        pw_hash: Vec::new(),
        token: own_host_info.token.clone(),
    };

    return auth;
}

async fn get_client_with_addr_port(addr_port: String) -> DataServiceClient<Channel> {
    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    let message = format!("configuring to connect to rcd at: {}", addr_port);
    info!("{}", message);

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap());
    let channel = endpoint.connect().await.unwrap();

    return DataServiceClient::new(channel);
}

#[allow(dead_code)]
async fn get_client(participant: CoopDatabaseParticipant) -> DataServiceClient<Channel> {
    let addr_port = format!("{}{}", participant.ip4addr, participant.db_port.to_string());
    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!("configuring to connect to rcd at: {}", addr_port);

    println!("{}", http_addr_port);

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap());
    let channel = endpoint.connect().await.unwrap();

    return DataServiceClient::new(channel);
}

async fn get_client_from_cds_host(host: &CdsHosts) -> DataServiceClient<Channel> {
    // let addr_port = format!("{}{}", host.ip4, host.port.to_string());
    let addr_port = host.ip4.clone();
    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    println!(
        "configuring to connect to rcd from cds host at: {}",
        addr_port
    );

    println!("{}", http_addr_port);

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap());
    let channel = endpoint.connect().await.unwrap();

    return DataServiceClient::new(channel);
}