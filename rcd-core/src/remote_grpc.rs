/*

Represents a gRPC client talking to a remote Data Service endpoint.

*/

use std::time::Duration;

use chrono::Utc;
use endianness::{read_i32, ByteOrder};
use guid_create::GUID;
use log::info;
use rcd_common::{
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::{CoopDatabaseParticipant, CoopDatabaseParticipantData},
    host_info::HostInfo,
};
use rcd_enum::contract_status::ContractStatus;
use rcdproto::rcdp::{
    data_service_client::DataServiceClient, AuthRequest, Contract, DatabaseSchema,
    DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
    GetRowFromPartialDatabaseResult, Host, InsertDataRequest, InsertDataResult, MessageInfo,
    NotifyHostOfRemovedRowRequest, Participant, ParticipantAcceptsContractRequest,
    RowParticipantAddress, SaveContractRequest, TryAuthRequest, UpdateDataRequest,
    UpdateDataResult, UpdateRowDataHashForHostRequest,
};
use tonic::transport::Channel;

use rcd_common::db::CdsHosts;

#[derive(Debug, Clone)]
pub struct RemoteGrpc {
    pub db_addr_port: String,
    pub timeout_in_seconds: u32,
}

impl RemoteGrpc {
    pub async fn try_auth_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
    ) -> bool {
        let auth = get_auth_request(own_host_info);
        let request = TryAuthRequest {
            authentication: Some(auth),
        };

        let client = get_client(participant, self.timeout_in_seconds);
        let response = client.await.try_auth(request).await;
        let result = response.unwrap().into_inner();
        return result.authentication_result.unwrap().is_authenticated;
    }

    pub async fn send_participant_contract(
        &self,
        participant: CoopDatabaseParticipant,
        host_info: HostInfo,
        contract: CoopDatabaseContract,
        db_schema: DatabaseSchema,
    ) -> (bool, String) {
        let message_info = get_message_info(&host_info, self.db_addr_port.clone());

        let contract = contract.to_cdata_contract(
            &host_info,
            self.db_addr_port.as_str().clone(),
            "",
            0,
            ContractStatus::Pending,
            db_schema,
            "",
            0
        );

        let request = tonic::Request::new(SaveContractRequest {
            contract: Some(contract),
            message_info: Some(message_info),
        });

        let addr_port = format!("{}{}", participant.ip4addr, participant.db_port.to_string());

        info!("sending request to rcd at: {}", addr_port);

        let client = get_client(participant, self.timeout_in_seconds);
        let response = client.await.save_contract(request).await.unwrap();

        let is_saved = response.get_ref().is_saved;
        let error_message = response.get_ref().error_message.clone();

        return (is_saved, error_message);
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
        let message_info = get_message_info(own_host_info, self.db_addr_port.clone());

        let chost = Host {
            host_guid: own_host_info.id.clone(),
            host_name: own_host_info.name.clone(),
            ip4_address: String::from(""),
            ip6_address: String::from(""),
            database_port_number: 0,
            token: own_host_info.token.clone(),
            http_addr: "".to_string(),
            http_port: 0
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

        let client = get_client_from_cds_host(host);
        let response = client.await.notify_host_of_removed_row(request).await;
        let result = response.unwrap().into_inner();
        return result.is_successful;
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

        let client = get_client(participant, self.timeout_in_seconds);
        let response = client
            .await
            .delete_command_into_table(request)
            .await
            .unwrap();

        return response.into_inner();
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

        let client = get_client(participant, self.timeout_in_seconds);
        let response = client
            .await
            .update_command_into_table(request)
            .await
            .unwrap();

        // println!("{:?}", response);

        return response.into_inner();
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

        let client = get_client(participant, self.timeout_in_seconds);
        let response = client
            .await
            .insert_command_into_table(request)
            .await
            .unwrap();

        return response.into_inner();
    }

    pub async fn get_row_from_participant(
        &self,
        participant: CoopDatabaseParticipantData,
        own_host_info: HostInfo,
    ) -> GetRowFromPartialDatabaseResult {
        let message_info = get_message_info(&own_host_info, self.db_addr_port.clone());
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

        let participant_info = participant.participant.clone();

        let client = get_client(participant_info, self.timeout_in_seconds);
        let response = client
            .await
            .get_row_from_partial_database(request)
            .await
            .unwrap();

        return response.into_inner();
    }

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
            http_addr: "".to_string(),
            http_port: 0
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
            internal_participant_guid: "".to_string(),
            http_addr: "".to_string(),
            http_port: 0,
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

        let client =
            get_client_with_addr_port(host_info.ip4_address.clone(), self.timeout_in_seconds);
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
        jwt: String::from("")
    };

    return auth;
}

async fn get_client_with_addr_port(
    addr_port: String,
    timeout_in_seconds: u32,
) -> DataServiceClient<Channel> {
    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    let message = format!("configuring to connect to rcd at: {}", addr_port);
    info!("{}", message);

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap())
        .timeout(Duration::from_secs(timeout_in_seconds.into()));
    let channel = endpoint.connect().await.unwrap();

    return DataServiceClient::new(channel);
}

#[allow(dead_code)]
async fn get_client(
    participant: CoopDatabaseParticipant,
    timeout_in_seconds: u32,
) -> DataServiceClient<Channel> {
    let addr_port = format!("{}{}", participant.ip4addr, participant.db_port.to_string());
    let http_addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!("configuring to connect to rcd at: {}", addr_port);

    println!("{}", http_addr_port);

    let endpoint = tonic::transport::Channel::builder(http_addr_port.parse().unwrap())
        .timeout(Duration::from_secs(timeout_in_seconds.into()));
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
