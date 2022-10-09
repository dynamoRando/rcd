use rcdproto::rcdp::sql_client_client::SqlClientClient;
use rcdproto::rcdp::{
    AcceptPendingContractRequest, AddParticipantRequest, AuthRequest,
    ChangeDeletesFromHostBehaviorRequest, ChangeDeletesToHostBehaviorRequest,
    ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
    ChangeUpdatesToHostBehaviorRequest, Contract, CreateUserDatabaseRequest,
    EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteRequest, ExecuteReadRequest,
    ExecuteWriteRequest, GenerateContractRequest, GenerateHostInfoRequest, GetDataHashRequest,
    GetLogicalStoragePolicyRequest, GetPendingUpdatesReply, GetPendingUpdatesRequest,
    GetReadRowIdsRequest, HasTableRequest, SendParticipantContractRequest,
    SetLogicalStoragePolicyRequest, StatementResultset, TryAuthAtParticipantRequest,
    ViewPendingContractsRequest, AcceptPendingUpdateRequest, AcceptPendingUpdateReply,
};

use rcdx::rcd_enum::{
    DatabaseType, DeletesFromHostBehavior, DeletesToHostBehavior, LogicalStoragePolicy,
    RemoteDeleteBehavior, UpdatesFromHostBehavior, UpdatesToHostBehavior,
};

use log::info;
use std::error::Error;
use tonic::transport::Channel;

/// An abstraction over the protobuff definition in Rust. Effectively exposes all the calls to the
/// `SQLClient` service and is used to talk to an rcd instance as a client
/// (and not as another `rcd` instance, aka a DataService client service. For that, use the `rcd_data_client` module).
pub struct RcdClient {
    /// The HTTP (or HTTPS) address and port of the `rcd` instance you are talking to. Example: `http://[::1]:50051`
    addr_port: String,
    /// The user name you will identify yourself to the `rcd` instance
    user_name: String,
    pw: String,
}

impl RcdClient {
    pub fn new(addr_port: String, user_name: String, pw: String) -> RcdClient {
        return RcdClient {
            addr_port: addr_port,
            user_name: user_name,
            pw: pw,
        };
    }

    pub async fn accept_pending_update_at_participant(self: &Self, db_name: &str, table_name: &str, row_id: u32)
    -> Result<AcceptPendingUpdateReply, Box<dyn Error>> {
        let auth = self.gen_auth_request();
        info!("sending request");

        let mut client = self.get_client().await;

        let request = AcceptPendingUpdateRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
            row_id,
        };

        let response = client.accept_pending_update_at_participant(request).await.unwrap().into_inner();

        Ok(response)
    }

    pub async fn get_pending_updates_at_participant(
        self: &Self,
        db_name: &str,
        table_name: &str,
    ) -> Result<GetPendingUpdatesReply, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GetPendingUpdatesRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            table_name: table_name.to_string(),
        });

        info!("sending request");

        let mut client = self.get_client().await;

        let response = client
            .get_pending_updates_at_participant(request)
            .await
            .unwrap()
            .into_inner();
        println!("RESPONSE={:?}", response);
        info!("response back");

        Ok(response)
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

    pub async fn generate_host_info(self: &Self, host_name: &str) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(GenerateHostInfoRequest {
            authentication: Some(auth),
            host_name: host_name.to_string(),
        });

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

        info!("sending request");

        let mut client = self.get_client().await;
        let response = client
            .execute_cooperative_write_at_host(request)
            .await
            .unwrap()
            .into_inner();

        // println!("{:?}", response);

        return Ok(response.is_successful);
    }

    pub async fn view_pending_contracts(self: &Self) -> Result<Vec<Contract>, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(ViewPendingContractsRequest {
            authentication: Some(auth),
        });

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

    pub async fn accept_pending_contract(
        self: &Self,
        host_alias: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(AcceptPendingContractRequest {
            authentication: Some(auth),
            host_alias: host_alias.to_string(),
        });

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

    pub async fn add_participant(
        self: &Self,
        db_name: &str,
        participant_alias: &str,
        participant_ip4addr: &str,
        participant_db_port: u32,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(AddParticipantRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
            alias: participant_alias.to_string(),
            ip4_address: participant_ip4addr.to_string(),
            port: participant_db_port,
        });

        info!("sending request");

        let mut client = self.get_client().await;

        let response = client.add_participant(request).await.unwrap().into_inner();
        println!("RESPONSE={:?}", response);
        info!("response back");

        Ok(response.is_successful)
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

        info!("sending request");

        let mut client = self.get_client().await;

        let response = client.has_table(request).await.unwrap().into_inner();
        println!("RESPONSE={:?}", response);
        info!("response back");

        Ok(response.has_table)
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

    pub async fn enable_cooperative_features(
        self: &Self,
        db_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(EnableCoooperativeFeaturesRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        });

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

    pub async fn create_user_database(self: &Self, db_name: &str) -> Result<bool, Box<dyn Error>> {
        let auth = self.gen_auth_request();

        let request = tonic::Request::new(CreateUserDatabaseRequest {
            authentication: Some(auth),
            database_name: db_name.to_string(),
        });

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

    async fn get_client(self: &Self) -> SqlClientClient<Channel> {
        println!("get_client addr_port {}", self.addr_port);
        let endpoint = tonic::transport::Channel::builder(self.addr_port.parse().unwrap());
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
}
