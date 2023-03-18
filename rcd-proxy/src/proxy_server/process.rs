use rcd_core::rcd::Rcd;
use rcd_messages::proxy::{request_type::RequestType, server_messages::ExecuteRequest};
use rcdproto::rcdp::CreateUserDatabaseRequest;
use rcdx::rcd_service::RcdService;

pub async fn process_request(
    request: ExecuteRequest,
    core: &Rcd,
) -> Result<String, String> {
    let result_request_type = RequestType::try_from(request.request_type);
    match result_request_type {
        Ok(request_type) => match request_type {
            RequestType::Unknown => todo!(),
            RequestType::Auth => todo!(),
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
            RequestType::EnableCooperativeFeatures => todo!(),
            RequestType::Read => todo!(),
            RequestType::Write => todo!(),
            RequestType::HasTable => todo!(),
            RequestType::SetLogicalStoragePolicy => todo!(),
            RequestType::GetLogicalStoragePolicy => todo!(),
            RequestType::GenerateContract => todo!(),
            RequestType::AddParticipant => todo!(),
            RequestType::SendParticipantContract => todo!(),
            RequestType::ViewPendingContracts => todo!(),
            RequestType::AcceptPendingContract => todo!(),
            RequestType::RejectPendingContract => todo!(),
            RequestType::GeneratHostInfo => todo!(),
            RequestType::ChangeHostStatus => todo!(),
            RequestType::TryAuthAtParticipant => todo!(),
            RequestType::ChangeUpdatesFromHostBehavior => todo!(),
            RequestType::ChangeDeletesFromHostBehavior => todo!(),
            RequestType::ChangeUpdatesToHostBehavior => todo!(),
            RequestType::ChangeDeletseToHostBehavior => todo!(),
            RequestType::GetDataHash => todo!(),
            RequestType::GetReadRowIds => todo!(),
            RequestType::GetDataLogTableStatus => todo!(),
            RequestType::SetDataLogTableStatus => todo!(),
            RequestType::GetPendingActions => todo!(),
            RequestType::AcceptPendingAction => todo!(),
            RequestType::GetDatabases => todo!(),
            RequestType::GetParticipants => todo!(),
            RequestType::GetActiveContract => todo!(),
            RequestType::GetUpdatesFromHostBehavior => todo!(),
            RequestType::GetUpdatesToHostBehavior => todo!(),
            RequestType::GetDeletesFromHostBehavior => todo!(),
            RequestType::GetDeletesToHostBehavior => todo!(),
            RequestType::GetCooperativeHosts => todo!(),
            RequestType::GetSettings => todo!(),
            RequestType::GetLogsByLastNumber => todo!(),
        },
        Err(e) => Err(e.to_string()),
    }
}
