use rcd_common::{
    coop_database_contract::CoopDatabaseContract,
    coop_database_participant::{CoopDatabaseParticipant, CoopDatabaseParticipantData},
    data_info::DataInfo,
    db::CdsHosts,
    host_info::HostInfo,
    save_contract_result::RcdSaveContractResult,
};
use rcdproto::rcdp::{
    Contract, DatabaseSchema, DeleteDataResult, GetRowFromPartialDatabaseResult, InsertDataResult,
    UpdateDataResult,
};
use serde::{Deserialize, Serialize};

use crate::{remote_grpc::RemoteGrpc, remote_http::RemoteHttp};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RcdCommunication {
    Unknown = 0,
    Grpc = 1,
    Http = 2,
}

impl RcdCommunication {
    pub fn from_u32(value: u32) -> RcdCommunication {
        match value {
            0 => RcdCommunication::Unknown,
            1 => RcdCommunication::Grpc,
            2 => RcdCommunication::Http,
            _ => panic!("Unknown value: {value}"),
        }
    }

    pub fn to_u32(comm: RcdCommunication) -> u32 {
        match comm {
            RcdCommunication::Unknown => 0,
            RcdCommunication::Grpc => 1,
            RcdCommunication::Http => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RcdRemoteDbClient {
    pub comm_type: RcdCommunication,
    pub grpc: Option<RemoteGrpc>,
    pub http: Option<RemoteHttp>,
}

impl RcdRemoteDbClient {
    pub async fn try_auth_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
    ) -> bool {
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .try_auth_at_participant(participant, own_host_info)
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .try_auth_at_participant(participant, own_host_info)
                    .await;
            }
        };
    }

    pub async fn send_participant_contract(
        &self,
        participant: CoopDatabaseParticipant,
        host_info: HostInfo,
        contract: CoopDatabaseContract,
        db_schema: DatabaseSchema,
    ) -> RcdSaveContractResult {
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .send_participant_contract(participant, host_info, contract, db_schema)
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .send_participant_contract(participant, host_info, contract, db_schema)
                    .await;
            }
        };
    }

    pub async fn notify_host_of_removed_row(
        &self,
        host: &CdsHosts,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> bool {
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .notify_host_of_removed_row(host, own_host_info, db_name, table_name, row_id)
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .notify_host_of_removed_row(host, own_host_info, db_name, table_name, row_id)
                    .await;
            }
        };
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
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .remove_row_at_participant(
                        participant,
                        own_host_info,
                        db_name,
                        table_name,
                        sql,
                        where_clause,
                    )
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .remove_row_at_participant(
                        participant,
                        own_host_info,
                        db_name,
                        table_name,
                        sql,
                        where_clause,
                    )
                    .await;
            }
        };
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
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .update_row_at_participant(
                        participant,
                        own_host_info,
                        db_name,
                        table_name,
                        sql,
                        where_clause,
                    )
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .update_row_at_participant(
                        participant,
                        own_host_info,
                        db_name,
                        table_name,
                        sql,
                        where_clause,
                    )
                    .await;
            }
        };
    }

    pub async fn insert_row_at_participant(
        &self,
        participant: CoopDatabaseParticipant,
        own_host_info: &HostInfo,
        db_name: &str,
        table_name: &str,
        sql: &str,
    ) -> InsertDataResult {
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .insert_row_at_participant(participant, own_host_info, db_name, table_name, sql)
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .insert_row_at_participant(participant, own_host_info, db_name, table_name, sql)
                    .await;
            }
        };
    }

    pub async fn get_row_from_participant(
        &self,
        participant: CoopDatabaseParticipantData,
        own_host_info: HostInfo,
        row_id: u32,
    ) -> GetRowFromPartialDatabaseResult {
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .get_row_from_participant(participant, own_host_info, row_id)
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .get_row_from_participant(participant, own_host_info, row_id)
                    .await;
            }
        };
    }

    pub async fn notify_host_of_updated_hash(
        &self,
        host: &CdsHosts,
        own_host_info: &HostInfo,
        data_info: &DataInfo,
    ) -> bool {
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .notify_host_of_updated_hash(host, own_host_info, data_info)
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .notify_host_of_updated_hash(host, own_host_info, data_info)
                    .await;
            }
        };
    }

    pub async fn notify_host_of_acceptance_of_contract(
        &self,
        accepted_contract: &Contract,
        own_host_info: &HostInfo,
    ) -> bool {
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .notify_host_of_acceptance_of_contract(accepted_contract, own_host_info)
                    .await;
            }
            RcdCommunication::Http => {
                return self
                    .http()
                    .notify_host_of_acceptance_of_contract(accepted_contract, own_host_info)
                    .await;
            }
        };
    }

    fn grpc(&self) -> RemoteGrpc {
        return self.grpc.as_ref().unwrap().clone();
    }

    fn http(&self) -> RemoteHttp {
        return self.http.as_ref().unwrap().clone();
    }
}
