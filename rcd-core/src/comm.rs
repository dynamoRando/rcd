use rcd_common::{db::CdsHosts, host_info::HostInfo};
use rcdproto::rcdp::Contract;
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
            _ => panic!("Unknown value: {}", value),
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
        match self.comm_type {
            RcdCommunication::Unknown => todo!(),
            RcdCommunication::Grpc => {
                return self
                    .grpc()
                    .notify_host_of_updated_hash(
                        host,
                        own_host_info,
                        db_name,
                        table_name,
                        row_id,
                        hash,
                        is_deleted,
                    )
                    .await;
            }
            RcdCommunication::Http => todo!(),
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
            RcdCommunication::Http => todo!(),
        };
    }

    fn grpc(&self) -> RemoteGrpc {
        return self.grpc.as_ref().unwrap().clone();
    }

    #[allow(dead_code)]
    fn http(&self) -> RemoteHttp {
        return self.http.as_ref().unwrap().clone();
    }
}
