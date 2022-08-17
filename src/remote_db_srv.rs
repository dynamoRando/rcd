use log::info;
use tonic::transport::Channel;
use guid_create::GUID;
use chrono::Utc;

use crate::cdata::data_service_client::DataServiceClient;
use crate::cdata::{SaveContractRequest, MessageInfo};
use crate::{
    cdata::GetRowFromPartialDatabaseResult, database_contract::DatabaseContract,
    database_participant::DatabaseParticipant, host_info::HostInfo,
};

#[allow(dead_code, unused_assignments, unused_variables)]
pub async fn send_participant_contract(
    participant: DatabaseParticipant,
    host_info: HostInfo,
    contract: DatabaseContract,
    own_db_addr_port: String,
) -> bool {
    // need to finish this message

    let message_info = get_message_info(host_info, own_db_addr_port);

    let request = tonic::Request::new(SaveContractRequest {
        contract: None,
        message_info: Some(message_info),
    });

    let addr_port = format!(
        "{}:{}",
        participant.ip4addr,
        participant.db_port.to_string()
    );

    info!("sending request to rcd at: {}", addr_port);

    let client = get_client(participant);
    let response = client.await.save_contract(request).await.unwrap();

    return response.into_inner().is_saved;
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn get_row_from_participant(
    participant: DatabaseParticipant,
    host_info: HostInfo,
    db_name: &str,
    table_name: &str,
) -> GetRowFromPartialDatabaseResult {
    unimplemented!();
}

#[allow(dead_code)]
async fn get_client(participant: DatabaseParticipant) -> DataServiceClient<Channel> {
    let addr_port = format!(
        "{}:{}",
        participant.ip4addr,
        participant.db_port.to_string()
    );
    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!("configuring to connect to rcd at: {}", addr_port);

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap());
    let channel = endpoint.connect().await.unwrap();

    return DataServiceClient::new(channel);
}

fn get_message_info(host_info: HostInfo, own_db_addr_port: String) -> MessageInfo {

    let mut addresses: Vec<String> = Vec::new();

    addresses.push(host_info.id);
    addresses.push(host_info.name);
    addresses.push(own_db_addr_port);


    let message_info = MessageInfo {
        is_little_endian: false,
        message_addresses: addresses,
        message_generated_time_utc: Utc::now().to_string(),
        message_guid: GUID::rand().to_string()
    };

    return message_info
}