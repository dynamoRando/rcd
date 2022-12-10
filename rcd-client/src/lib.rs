use client_type::RcdClientType;
use rcd_http_common::url::client::{
    ACCEPT_PENDING_ACTION, ACCEPT_PENDING_CONTRACT, ADD_PARTICIPANT,
    CHANGE_DELETES_FROM_HOST_BEHAVIOR, CHANGE_DELETES_TO_HOST_BEHAVIOR, CHANGE_HOST_STATUS_ID,
    CHANGE_HOST_STATUS_NAME, CHANGE_UPDATES_FROM_HOST_BEHAVIOR, CHANGE_UPDATES_TO_HOST_BEHAVIOR,
    COOPERATIVE_WRITE_SQL_AT_HOST, ENABLE_COOPERATIVE_FEATURES, GENERATE_CONTRACT,
    GENERATE_HOST_INFO, GET_ACTIVE_CONTRACT, GET_DATABSES, GET_DATA_HASH_AT_HOST,
    GET_DATA_HASH_AT_PARTICIPANT, GET_PARTICIPANTS, GET_PENDING_ACTIONS, GET_POLICY,
    GET_ROW_AT_PARTICIPANT, IS_ONLINE, NEW_DATABASE, READ_SQL_AT_HOST, READ_SQL_AT_PARTICIPANT,
    SEND_CONTRACT_TO_PARTICIPANT, SET_POLICY, TRY_AUTH_PARTICIPANT, VIEW_PENDING_CONTRACTS,
    WRITE_SQL_AT_HOST, WRITE_SQL_AT_PARTICIPANT,
};
use rcdproto::rcdp::sql_client_client::SqlClientClient;
use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, AcceptPendingContractReply,
    AcceptPendingContractRequest, AddParticipantReply, AddParticipantRequest, AuthRequest,
    ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest, ChangeHostStatusReply,
    ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
    ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
    ChangesUpdatesFromHostBehaviorReply, Contract, CreateUserDatabaseReply,
    CreateUserDatabaseRequest, EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest,
    ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
    ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest, GenerateContractReply,
    GenerateContractRequest, GenerateHostInfoReply, GenerateHostInfoRequest,
    GetActiveContractReply, GetActiveContractRequest, GetDataHashReply, GetDataHashRequest,
    GetDatabasesReply, GetDatabasesRequest, GetLogicalStoragePolicyReply,
    GetLogicalStoragePolicyRequest, GetParticipantsReply, GetParticipantsRequest,
    GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsReply, GetReadRowIdsRequest,
    HasTableReply, HasTableRequest, SendParticipantContractReply, SendParticipantContractRequest,
    SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest, StatementResultset, TestReply,
    TestRequest, TryAuthAtParticipantRequest, TryAuthAtPartipantReply, ViewPendingContractsReply,
    ViewPendingContractsRequest,
};

use rcd_common::rcd_enum::{
    DatabaseType, DeletesFromHostBehavior, DeletesToHostBehavior, LogicalStoragePolicy,
    RemoteDeleteBehavior, UpdatesFromHostBehavior, UpdatesToHostBehavior,
};

use log::info;
use std::error::Error;
use std::time::Duration;
use tonic::transport::Channel;

pub mod client_type;

/// An abstraction over the protobuff definition in Rust. Effectively exposes all the calls to the
/// `SQLClient` service and is used to talk to an rcd instance as a client
/// (and not as another `rcd` instance, aka a DataService client service. For that, use the `rcd_data_client` module).
#[derive(Debug)]
#[allow(dead_code)]
pub struct RcdClient {
    /// The HTTP (or HTTPS) address and port of the `rcd` instance you are talking to. Example: `http://[::1]:50051`
    grpc_client_addr_port: String,
    /// The user name you will identify yourself to the `rcd` instance
    user_name: String,
    pw: String,
    timeout_in_seconds: u32,
    http_addr: String,
    http_port: u32,
    client_type: RcdClientType,
}

impl RcdClient {
    pub fn new(
        grpc_client_addr_port: String,
        user_name: String,
        pw: String,
        timeout_in_seconds: u32,
        http_addr: String,
        http_port: u32,
        client_type: RcdClientType,
    ) -> RcdClient {
        return RcdClient {
            grpc_client_addr_port: grpc_client_addr_port,
            user_name: user_name,
            pw: pw,
            timeout_in_seconds,
            http_addr,
            http_port,
            client_type,
        };
    }

    pub fn new_grpc_client(
        grpc_client_addr_port: String,
        user_name: String,
        pw: String,
        timeout_in_seconds: u32,
    ) -> RcdClient {
        return RcdClient {
            grpc_client_addr_port: grpc_client_addr_port,
            user_name: user_name,
            pw: pw,
            timeout_in_seconds,
            http_addr: "".to_string(),
            http_port: 0,
            client_type: RcdClientType::Grpc,
        };
    }

    pub fn new_http_client(
        user_name: String,
        pw: String,
        timeout_in_seconds: u32,
        http_addr: String,
        http_port: u32,
    ) -> RcdClient {
        return RcdClient {
            grpc_client_addr_port: "".to_string(),
            user_name: user_name,
            pw: pw,
            timeout_in_seconds,
            http_addr,
            http_port,
            client_type: RcdClientType::Http,
        };
    }

    pub async fn is_online_reply(self: &Self, message: String) -> TestReply {
        match self.client_type {
            RcdClientType::Grpc => {
                let mut client = self.get_client().await;

                let request = TestRequest {
                    request_time_utc: "".to_string(),
                    request_origin_url: "".to_string(),
                    request_origin_ip4: "".to_string(),
                    request_origin_ip6: "".to_string(),
                    request_port_number: 0,
                    request_echo_message: message.clone().to_string(),
                };

                let result = client.is_online(request).await.unwrap();

                return result.into_inner();
            }
            RcdClientType::Http => {
                let request = TestRequest {
                    request_time_utc: "".to_string(),
                    request_origin_url: "".to_string(),
                    request_origin_ip4: "".to_string(),
                    request_origin_ip6: "".to_string(),
                    request_port_number: 0,
                    request_echo_message: message.clone().to_string(),
                };

                let url = self.get_http_url(IS_ONLINE);
                let request_json = serde_json::to_string(&request).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: TestReply = serde_json::from_str(&result_json).unwrap();

                return result;
            }
        };
    }

    pub async fn is_online(self: &Self) -> bool {
        match self.client_type {
            RcdClientType::Grpc => {
                let mut client = self.get_client().await;

                let test_string = "is_online";

                let request = TestRequest {
                    request_time_utc: "".to_string(),
                    request_origin_url: "".to_string(),
                    request_origin_ip4: "".to_string(),
                    request_origin_ip6: "".to_string(),
                    request_port_number: 0,
                    request_echo_message: test_string.clone().to_string(),
                };

                let result = client.is_online(request).await.unwrap();

                return result.into_inner().reply_echo_message == test_string;
            }
            RcdClientType::Http => {
                let test_string = "is_online";

                let request = TestRequest {
                    request_time_utc: "".to_string(),
                    request_origin_url: "".to_string(),
                    request_origin_ip4: "".to_string(),
                    request_origin_ip6: "".to_string(),
                    request_port_number: 0,
                    request_echo_message: test_string.clone().to_string(),
                };

                let url = self.get_http_url(IS_ONLINE);
                let request_json = serde_json::to_string(&request).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: TestReply = serde_json::from_str(&result_json).unwrap();

                return result.reply_echo_message == test_string;
            }
        };
    }

    pub async fn get_active_contract(
        self: &Self,
        db_name: &str,
    ) -> Result<GetActiveContractReply, Box<dyn Error>> {
        match self.client_type {
            RcdClientType::Grpc => {
                let auth = self.gen_auth_request();
                info!("sending request");

                let mut client = self.get_client().await;

                let request = GetActiveContractRequest {
                    authentication: Some(auth),
                    database_name: db_name.to_string(),
                };

                let response = client
                    .get_active_contract(request)
                    .await
                    .unwrap()
                    .into_inner();

                Ok(response)
            }
            RcdClientType::Http => {
                let auth = self.gen_auth_request();
                info!("sending request");

                let request = GetActiveContractRequest {
                    authentication: Some(auth),
                    database_name: db_name.to_string(),
                };

                let url = self.get_http_url(GET_ACTIVE_CONTRACT);
                let request_json = serde_json::to_string(&request).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: GetActiveContractReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result);
            }
        }
    }

    pub async fn accept_pending_action_at_participant(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<AcceptPendingActionReply, Box<dyn Error>> {
        match self.client_type {
            RcdClientType::Grpc => {
                let auth = self.gen_auth_request();
                info!("sending request");

                let mut client = self.get_client().await;

                let request = AcceptPendingActionRequest {
                    authentication: Some(auth),
                    database_name: db_name.to_string(),
                    table_name: table_name.to_string(),
                    row_id,
                };

                let response = client
                    .accept_pending_action_at_participant(request)
                    .await
                    .unwrap()
                    .into_inner();

                Ok(response)
            }
            RcdClientType::Http => {
                let auth = self.gen_auth_request();
                info!("sending request");

                let request = AcceptPendingActionRequest {
                    authentication: Some(auth),
                    database_name: db_name.to_string(),
                    table_name: table_name.to_string(),
                    row_id,
                };

                let url = self.get_http_url(ACCEPT_PENDING_ACTION);
                let request_json = serde_json::to_string(&request).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: AcceptPendingActionReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result);
            }
        }
    }

    pub async fn get_participants_for_database(
        self: &Self,
        db_name: &str,
    ) -> Result<GetParticipantsReply, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GetParticipantsRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client.get_participants(request).await.unwrap().into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GET_PARTICIPANTS);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: GetParticipantsReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result);
            }
        }
    }

    pub async fn get_pending_actions_at_participant(
        self: &Self,
        db_name: &str,
        table_name: &str,
        action: &str,
    ) -> Result<GetPendingActionsReply, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GetPendingActionsRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            action: action.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .get_pending_actions_at_participant(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GET_PENDING_ACTIONS);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: GetPendingActionsReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result);
            }
        }
    }

    pub async fn get_row_id_at_participant(
        self: &Self,
        db_name: &str,
        table_name: &str,
        where_clause: &str,
    ) -> Result<Vec<u32>, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GetReadRowIdsRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            where_clause: where_clause.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .read_row_id_at_participant(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.row_ids)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GET_ROW_AT_PARTICIPANT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: GetReadRowIdsReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result.row_ids);
            }
        }
    }

    pub async fn get_data_hash_at_participant(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<u64, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GetDataHashRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id: row_id,
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .get_data_hash_at_participant(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.data_hash)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GET_DATA_HASH_AT_PARTICIPANT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: GetDataHashReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result.data_hash);
            }
        }
    }

    pub async fn get_data_hash_at_host(
        self: &Self,
        db_name: &str,
        table_name: &str,
        row_id: u32,
    ) -> Result<u64, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GetDataHashRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id: row_id,
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .get_data_hash_at_host(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.data_hash)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GET_DATA_HASH_AT_HOST);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: GetDataHashReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result.data_hash);
            }
        }
    }

    pub async fn change_deletes_to_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: DeletesToHostBehavior,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ChangeDeletesToHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: DeletesToHostBehavior::to_u32(behavior),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .change_deletes_to_host_behavior(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(CHANGE_DELETES_TO_HOST_BEHAVIOR);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: ChangeDeletesToHostBehaviorReply =
                    serde_json::from_str(&result_json).unwrap();

                return Ok(result.is_successful);
            }
        }
    }

    pub async fn change_updates_to_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: UpdatesToHostBehavior,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ChangeUpdatesToHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: UpdatesToHostBehavior::to_u32(behavior),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .change_updates_to_host_behavior(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(CHANGE_UPDATES_TO_HOST_BEHAVIOR);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: ChangeUpdatesToHostBehaviorReply =
                    serde_json::from_str(&result_json).unwrap();

                return Ok(result.is_successful);
            }
        }
    }

    pub async fn change_deletes_from_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: DeletesFromHostBehavior,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ChangeDeletesFromHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: DeletesFromHostBehavior::to_u32(behavior),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .change_deletes_from_host_behavior(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(CHANGE_DELETES_FROM_HOST_BEHAVIOR);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: ChangeDeletesFromHostBehaviorReply =
                    serde_json::from_str(&result_json).unwrap();

                return Ok(result.is_successful);
            }
        }
    }

    pub async fn change_updates_from_host_behavior(
        self: &Self,
        db_name: &str,
        table_name: &str,
        behavior: UpdatesFromHostBehavior,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ChangeUpdatesFromHostBehaviorRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            behavior: UpdatesFromHostBehavior::to_u32(behavior),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .change_updates_from_host_behavior(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(CHANGE_UPDATES_FROM_HOST_BEHAVIOR);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: ChangesUpdatesFromHostBehaviorReply =
                    serde_json::from_str(&result_json).unwrap();

                return Ok(result.is_successful);
            }
        }
    }

    pub async fn change_host_status_by_id(
        self: &Self,
        host_id: &str,
        status: u32,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ChangeHostStatusRequest {
            authentication: Some(auth),
            host_alias: String::from(""),
            host_id: host_id.to_string(),
            status,
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .change_host_status(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(CHANGE_HOST_STATUS_ID);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: ChangeHostStatusReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result.is_successful);
            }
        }
    }

    pub async fn change_host_status_by_name(
        self: &Self,
        host_name: &str,
        status: u32,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ChangeHostStatusRequest {
            authentication: Some(auth),
            host_alias: host_name.to_string(),
            host_id: String::from(""),
            status,
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .change_host_status(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(CHANGE_HOST_STATUS_NAME);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();

                let result_json = self.send_http_message(request_json, url).await;

                let result: ChangeHostStatusReply = serde_json::from_str(&result_json).unwrap();

                return Ok(result.is_successful);
            }
        }
    }

    pub async fn generate_host_info(self: &Self, host_name: &str) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GenerateHostInfoRequest {
            authentication: Some(auth),
            host_name: host_name.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");
                let mut client = self.get_client().await;

                let response = client
                    .generate_host_info(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GENERATE_HOST_INFO);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: GenerateHostInfoReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn get_databases(self: &Self) -> Result<GetDatabasesReply, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = GetDatabasesRequest {
            authentication: Some(auth),
        };

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;
                let response = client.get_databases(request).await.unwrap().into_inner();

                println!("{:?}", response);

                return Ok(response);
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GET_DATABSES);
                let request_json = serde_json::to_string(&request).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: GetDatabasesReply = serde_json::from_str(&result_json).unwrap();

                Ok(result)
            }
        }
    }

    pub async fn execute_cooperative_write_at_host(
        self: &Self,
        db_name: &str,
        cmd: &str,
        participant_alias: &str,
        where_clause: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = ExecuteCooperativeWriteRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: cmd.to_string(),
            database_type: DatabaseType::to_u32(DatabaseType::Sqlite),
            alias: participant_alias.to_string(),
            participant_id: String::from(""),
            where_clause: where_clause.to_string(),
        };

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;
                let response = client
                    .execute_cooperative_write_at_host(request)
                    .await
                    .unwrap()
                    .into_inner();

                println!("{:?}", response);

                return Ok(response.is_successful);
            }
            RcdClientType::Http => {
                let url = self.get_http_url(COOPERATIVE_WRITE_SQL_AT_HOST);
                let request_json = serde_json::to_string(&request).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: ExecuteCooperativeWriteReply =
                    serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn view_pending_contracts(self: &Self) -> Result<Vec<Contract>, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ViewPendingContractsRequest {
            authentication: Some(auth),
        });
        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .review_pending_contracts(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.contracts)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(VIEW_PENDING_CONTRACTS);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: ViewPendingContractsReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.contracts)
            }
        }
    }

    pub async fn accept_pending_contract(
        self: &Self,
        host_alias: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(AcceptPendingContractRequest {
            authentication: Some(auth),
            host_alias: host_alias.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .accept_pending_contract(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(ACCEPT_PENDING_CONTRACT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: AcceptPendingContractReply =
                    serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn send_participant_contract(
        self: &Self,
        db_name: &str,
        participant_alias: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(SendParticipantContractRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            participant_alias: participant_alias.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .send_participant_contract(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_sent)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(SEND_CONTRACT_TO_PARTICIPANT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: SendParticipantContractReply =
                    serde_json::from_str(&result_json).unwrap();

                Ok(result.is_sent)
            }
        }
    }

    pub async fn add_participant(
        self: &Self,
        db_name: &str,
        participant_alias: &str,
        participant_ip4addr: &str,
        participant_db_port: u32,
        participant_http_addr: String,
        participant_http_port: u16,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(AddParticipantRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            alias: participant_alias.to_string(),
            ip4_address: participant_ip4addr.to_string(),
            port: participant_db_port,
            http_addr: participant_http_addr,
            http_port: participant_http_port as u32,
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client.add_participant(request).await.unwrap().into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(ADD_PARTICIPANT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: AddParticipantReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn generate_contract(
        self: &Self,
        db_name: &str,
        host_name: &str,
        desc: &str,
        remote_delete_behavior: RemoteDeleteBehavior,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GenerateContractRequest {
            authentication: Some(auth),
            host_name: host_name.to_string(),
            description: desc.to_string(),
            database_name: db_name.to_string(),
            remote_delete_behavior: RemoteDeleteBehavior::to_u32(remote_delete_behavior),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .generate_contract(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GENERATE_CONTRACT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: GenerateContractReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn has_table(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(HasTableRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client.has_table(request).await.unwrap().into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.has_table)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GENERATE_CONTRACT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: HasTableReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.has_table)
            }
        }
    }

    pub async fn get_logical_storage_policy(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> Result<LogicalStoragePolicy, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GetLogicalStoragePolicyRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .get_logical_storage_policy(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                let policy = LogicalStoragePolicy::from_i64(response.policy_mode as i64);

                Ok(policy)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(GET_POLICY);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: GetLogicalStoragePolicyReply =
                    serde_json::from_str(&result_json).unwrap();

                Ok(LogicalStoragePolicy::from_i64(result.policy_mode as i64))
            }
        }
    }

    pub async fn set_logical_storage_policy(
        self: &Self,
        db_name: &str,
        table_name: &str,
        policy: LogicalStoragePolicy,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(SetLogicalStoragePolicyRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            policy_mode: LogicalStoragePolicy::to_u32(policy),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .set_logical_storage_policy(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(SET_POLICY);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: SetLogicalStoragePolicyReply =
                    serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn execute_write_at_host(
        self: &Self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ExecuteWriteRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .execute_write_at_host(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(WRITE_SQL_AT_HOST);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: ExecuteWriteReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn execute_write_at_participant(
        self: &Self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
        where_clause: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ExecuteWriteRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
            where_clause: where_clause.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .execute_write_at_participant(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(WRITE_SQL_AT_PARTICIPANT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: ExecuteWriteReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn try_auth_at_participant(
        self: &Self,
        alias: &str,
        id: &str,
        db_name: &str,
    ) -> bool {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(TryAuthAtParticipantRequest {
            authentication: Some(auth),
            participant_id: id.to_string(),
            participant_alias: alias.to_string(),
            db_name: db_name.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .try_auth_at_participant(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                return response.is_successful;
            }
            RcdClientType::Http => {
                let url = self.get_http_url(TRY_AUTH_PARTICIPANT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: TryAuthAtPartipantReply = serde_json::from_str(&result_json).unwrap();

                return result.is_successful;
            }
        }
    }

    pub async fn execute_read_at_participant(
        self: &Self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ExecuteReadRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .execute_read_at_participant(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.results[0].clone())
            }
            RcdClientType::Http => {
                let url = self.get_http_url(READ_SQL_AT_PARTICIPANT);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: ExecuteReadReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.results[0].clone())
            }
        }
    }

    pub async fn execute_read_at_host(
        self: &Self,
        db_name: &str,
        sql_statement: &str,
        db_type: u32,
    ) -> Result<StatementResultset, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ExecuteReadRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            sql_statement: sql_statement.to_string(),
            database_type: db_type,
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .execute_read_at_host(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.results[0].clone())
            }
            RcdClientType::Http => {
                let url = self.get_http_url(READ_SQL_AT_HOST);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: ExecuteReadReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.results[0].clone())
            }
        }
    }

    pub async fn enable_cooperative_features(
        self: &Self,
        db_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(EnableCoooperativeFeaturesRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .enable_coooperative_features(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_successful)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(ENABLE_COOPERATIVE_FEATURES);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: EnableCoooperativeFeaturesReply =
                    serde_json::from_str(&result_json).unwrap();

                Ok(result.is_successful)
            }
        }
    }

    pub async fn create_user_database(self: &Self, db_name: &str) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(CreateUserDatabaseRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        });

        match self.client_type {
            RcdClientType::Grpc => {
                info!("sending request");

                let mut client = self.get_client().await;

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                Ok(response.is_created)
            }
            RcdClientType::Http => {
                let url = self.get_http_url(NEW_DATABASE);
                let request_json = serde_json::to_string(&request.into_inner()).unwrap();
                let result_json = self.send_http_message(request_json, url).await;
                let result: CreateUserDatabaseReply = serde_json::from_str(&result_json).unwrap();

                Ok(result.is_created)
            }
        }
    }

    async fn get_client(self: &Self) -> SqlClientClient<Channel> {
        println!("get_client addr_port {}", self.grpc_client_addr_port);
        // need to make the timeout configurable
        let endpoint =
            tonic::transport::Channel::builder(self.grpc_client_addr_port.parse().unwrap())
                .timeout(Duration::from_secs(self.timeout_in_seconds.into()));
        let channel = endpoint.connect().await.unwrap();
        return SqlClientClient::new(channel);
    }

    fn gen_auth_request(&self) -> AuthRequest {
        let auth = AuthRequest {
            user_name: self.user_name.clone(),
            pw: self.pw.clone(),
            pw_hash: Vec::new(),
            token: Vec::new(),
        };

        return auth;
    }

    async fn send_http_message(&self, json_message: String, url: String) -> String {
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

    fn get_http_url(&self, action_url: &str) -> String {
        let http_base = format!(
            "{}{}:{}",
            "http://",
            self.http_addr,
            self.http_port.to_string()
        );

        let result = format!("{}{}", http_base, action_url);
        println!("{}", result);
        return result;
    }
}
