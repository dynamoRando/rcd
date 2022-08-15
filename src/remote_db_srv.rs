use log::info;
use tonic::transport::Channel;

use crate::cdata::data_service_client::DataServiceClient;
use crate::cdata::SaveContractRequest;
use crate::{
    cdata::GetRowFromPartialDatabaseResult, database_contract::DatabaseContract,
    database_participant::DatabaseParticipant, host_info::HostInfo,
};

#[allow(dead_code, unused_assignments, unused_variables)]
pub async fn send_participant_contract(
    participant: DatabaseParticipant,
    host_info: HostInfo,
    contract: DatabaseContract,
) -> bool {
    // need to finish this message
    let request = tonic::Request::new(SaveContractRequest {
        contract: None,
        message_info: None,
    });

    let addr_port = format!(
        "{}:{}",
        participant.ip4addr,
        participant.db_port.to_string()
    );

    info!("sending request to rcd at: {}", addr_port);

    let client = get_client(participant);
    let response = client.await.save_contract(request).await.unwrap();

    unimplemented!();
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
