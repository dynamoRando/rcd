use tracing::debug;
use rcd_core::rcd::Rcd;
use rcd_messages::proxy::{request_type::RequestType, server_messages::ExecuteRequest};
use rcdproto::rcdp::{
    AcceptPendingActionRequest, AcceptPendingContractRequest, AddParticipantRequest, AuthRequest,
    ChangeDeletesFromHostBehaviorRequest, ChangeDeletesToHostBehaviorRequest,
    ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
    ChangeUpdatesToHostBehaviorRequest, CreateUserDatabaseRequest,
    EnableCoooperativeFeaturesRequest, ExecuteCooperativeWriteRequest, ExecuteReadRequest,
    ExecuteWriteRequest, GenerateContractRequest, GenerateHostInfoRequest,
    GetActiveContractRequest, GetCooperativeHostsRequest, GetDataHashRequest, GetDatabasesRequest,
    GetDeletesFromHostBehaviorRequest, GetDeletesToHostBehaviorRequest,
    GetLogicalStoragePolicyRequest, GetLogsByLastNumberRequest, GetParticipantsRequest,
    GetPendingActionsRequest, GetReadRowIdsRequest, GetSettingsRequest,
    GetUpdatesFromHostBehaviorRequest, GetUpdatesToHostBehaviorRequest, HasTableRequest,
    SendParticipantContractRequest, SetLogicalStoragePolicyRequest, TryAuthAtParticipantRequest,
    ViewPendingContractsRequest,
};

pub async fn process_request(request: &ExecuteRequest, core: &Rcd) -> Result<String, String> {
    let result_request_type = RequestType::try_from(request.request_type);
    match result_request_type {
        Ok(request_type) => {
            debug!("request_type: {request_type:?}");
            match request_type {
                RequestType::Unknown => todo!(),
                RequestType::Auth => {
                    let result_request = serde_json::from_str::<AuthRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.auth_for_token(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::CreateUserDatabase => {
                    let result_request =
                        serde_json::from_str::<CreateUserDatabaseRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.create_user_database(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::EnableCooperativeFeatures => {
                    let result_request = serde_json::from_str::<EnableCoooperativeFeaturesRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.enable_coooperative_features(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ReadAtHost => {
                    let result_request =
                        serde_json::from_str::<ExecuteReadRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.execute_read_at_host(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::WriteAtHost => {
                    let result_request =
                        serde_json::from_str::<ExecuteWriteRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.execute_write_at_host(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::HasTable => {
                    let result_request =
                        serde_json::from_str::<HasTableRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.has_table(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::SetLogicalStoragePolicy => {
                    let result_request = serde_json::from_str::<SetLogicalStoragePolicyRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.set_logical_storage_policy(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetLogicalStoragePolicy => {
                    let result_request = serde_json::from_str::<GetLogicalStoragePolicyRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_logical_storage_policy(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GenerateContract => {
                    let result_request =
                        serde_json::from_str::<GenerateContractRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.generate_contract(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::AddParticipant => {
                    let result_request =
                        serde_json::from_str::<AddParticipantRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.add_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::SendParticipantContract => {
                    let result_request = serde_json::from_str::<SendParticipantContractRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.send_participant_contract(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ViewPendingContracts => {
                    let result_request =
                        serde_json::from_str::<ViewPendingContractsRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.review_pending_contracts(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::AcceptPendingContract => {
                    let result_request =
                        serde_json::from_str::<AcceptPendingContractRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.accept_pending_contract(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::RejectPendingContract => {
                    todo!()
                }
                RequestType::GenerateHostInfo => {
                    let result_request =
                        serde_json::from_str::<GenerateHostInfoRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.generate_host_info(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ChangeHostStatus => {
                    let result_request =
                        serde_json::from_str::<ChangeHostStatusRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.change_host_status(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::TryAuthAtParticipant => {
                    let result_request =
                        serde_json::from_str::<TryAuthAtParticipantRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.try_auth_at_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ChangeUpdatesFromHostBehavior => {
                    let result_request = serde_json::from_str::<ChangeUpdatesFromHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.change_updates_from_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ChangeDeletesFromHostBehavior => {
                    let result_request = serde_json::from_str::<ChangeDeletesFromHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.change_deletes_from_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ChangeUpdatesToHostBehavior => {
                    let result_request = serde_json::from_str::<ChangeUpdatesToHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.change_updates_to_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ChangeDeletesToHostBehavior => {
                    let result_request = serde_json::from_str::<ChangeDeletesToHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.change_deletes_to_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetDataHashAtHost => {
                    let result_request =
                        serde_json::from_str::<GetDataHashRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_data_hash_at_host(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetReadRowIds => {
                    let result_request =
                        serde_json::from_str::<GetReadRowIdsRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.read_row_id_at_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetDataLogTableStatus => {
                    todo!()
                }
                RequestType::SetDataLogTableStatus => todo!(),
                RequestType::GetPendingActions => {
                    let result_request =
                        serde_json::from_str::<GetPendingActionsRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_pending_actions_at_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::AcceptPendingAction => {
                    let result_request =
                        serde_json::from_str::<AcceptPendingActionRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.accept_pending_action_at_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetDatabases => {
                    let result_request =
                        serde_json::from_str::<GetDatabasesRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_databases(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetParticipants => {
                    let result_request =
                        serde_json::from_str::<GetParticipantsRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_participants(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetActiveContract => {
                    let result_request =
                        serde_json::from_str::<GetActiveContractRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_active_contact(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetUpdatesFromHostBehavior => {
                    let result_request = serde_json::from_str::<GetUpdatesFromHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_updates_from_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetUpdatesToHostBehavior => {
                    let result_request = serde_json::from_str::<GetUpdatesToHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_updates_to_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetDeletesFromHostBehavior => {
                    let result_request = serde_json::from_str::<GetDeletesFromHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_deletes_from_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetDeletesToHostBehavior => {
                    let result_request = serde_json::from_str::<GetDeletesToHostBehaviorRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_deletes_to_host_behavior(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetCooperativeHosts => {
                    let result_request =
                        serde_json::from_str::<GetCooperativeHostsRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_cooperative_hosts(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetSettings => {
                    let result_request =
                        serde_json::from_str::<GetSettingsRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_settings(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetLogsByLastNumber => {
                    let result_request =
                        serde_json::from_str::<GetLogsByLastNumberRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_last_log_entries(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ReadAtPart => {
                    let result_request =
                        serde_json::from_str::<ExecuteReadRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.execute_read_at_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::WriteAtPart => {
                    let result_request =
                        serde_json::from_str::<ExecuteWriteRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.execute_write_at_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::CooperativeWriteAtHost => {
                    let result_request = serde_json::from_str::<ExecuteCooperativeWriteRequest>(
                        &request.request_json,
                    );
                    match result_request {
                        Ok(request) => {
                            let reply = core.execute_cooperative_write_at_host(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::GetDataHashAtPart => {
                    let result_request =
                        serde_json::from_str::<GetDataHashRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_data_hash_at_participant(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                RequestType::ViewHostInfo => {
                    let result_request = serde_json::from_str::<AuthRequest>(&request.request_json);
                    match result_request {
                        Ok(request) => {
                            let reply = core.get_host_info(request).await;
                            return Ok(serde_json::to_string(&reply).unwrap());
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
