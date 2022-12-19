use crate::{get_auth_request, get_base_address, request, AppMessage, ContractIntent, RcdAdminApp};
use rcd_http_common::url::client::{
    GENERATE_CONTRACT, GET_ACTIVE_CONTRACT, SEND_CONTRACT_TO_PARTICIPANT,
};
use rcd_messages::client::{
    GenerateContractReply, GenerateContractRequest, GetActiveContractReply,
    GetActiveContractRequest, SendParticipantContractReply, SendParticipantContractRequest,
    ViewPendingContractsReply,
};
use rcd_messages::formatter;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;
use yew::{html::Scope, Html};

pub mod active;
pub mod generate;
pub mod pending;

pub fn view_contracts(app: &RcdAdminApp, link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.page.contract_is_visible;

    html!(
      <div hidden={is_visible}>
          <h1> {"Contracts"} </h1>
          // pending contracts
          { pending::view(app, link) }
          // generate contract
          { generate::view(app,link) }
          // view active contract
           { active::view(app, link)} 
          </div>
    )
}

pub fn handle_contract_intent(
    app: &mut RcdAdminApp,
    intent: ContractIntent,
    link: &Scope<RcdAdminApp>,
) {
    match intent {
        ContractIntent::Unknown => todo!(),
        ContractIntent::GetPending => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), GENERATE_CONTRACT);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;

            let host_name_ui = &app.contract.generate.ui.host_name;

            let host_name = host_name_ui.cast::<HtmlInputElement>().unwrap().value();

            let desc_ui = &app.contract.generate.ui.description;

            let description = desc_ui.cast::<HtmlInputElement>().unwrap().value();

            let behavior = app.contract.generate.data.delete_behavior;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let request = GenerateContractRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                host_name: host_name,
                description: description,
                remote_delete_behavior: behavior,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = link.callback(AppMessage::HandleContractResponse);

            request::get_data(url, request_json, callback);

            todo!()
        }
        ContractIntent::GetAccepted => todo!(),
        ContractIntent::GetRejected => todo!(),
        ContractIntent::AcceptContract(_) => todo!(),
        ContractIntent::GenerateContract => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), GENERATE_CONTRACT);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;

            let host_name_ui = &app.contract.generate.ui.host_name;

            let host_name = host_name_ui.cast::<HtmlInputElement>().unwrap().value();

            let desc_ui = &app.contract.generate.ui.description;

            let description = desc_ui.cast::<HtmlInputElement>().unwrap().value();

            let behavior = app.contract.generate.data.delete_behavior;

            console::log_1(&"selected db".into());
            console::log_1(&db_name.into());

            let request = GenerateContractRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                host_name: host_name,
                description: description,
                remote_delete_behavior: behavior,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = link.callback(AppMessage::HandleContractResponse);

            request::get_data(url, request_json, callback);
        }
        ContractIntent::SendContractToParticipant => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), SEND_CONTRACT_TO_PARTICIPANT);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app
                
                .databases
                .data
                .active
                .database_name
                .clone();
            let participant_alias = app.participants.send_contract.data.alias.clone();

            let request = SendParticipantContractRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
                participant_alias: participant_alias,
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = link.callback(AppMessage::HandleContractSendToParticipant);

            request::get_data(url, request_json, callback);
        }
        ContractIntent::RejectContract(_) => todo!(),
        ContractIntent::ViewCurrentContract => {
            let base_address = get_base_address(&app.connection.data);
            let url = format!("{}{}", base_address.clone(), GET_ACTIVE_CONTRACT);
            let auth = get_auth_request(&app.connection.data);
            let db_name = &app.databases.data.active.database_name;

            let request = GetActiveContractRequest {
                authentication: Some(auth),
                database_name: db_name.clone(),
            };

            let request_json = serde_json::to_string(&request).unwrap();

            let callback = link.callback(AppMessage::HandleGetActiveContractResponse);

            request::get_data(url, request_json, callback);
        }
        ContractIntent::SetParticipantForPendingContractSend(participant_alias) => {
            app.participants.send_contract.data.alias = participant_alias.clone();
        }
    }
}

pub fn handle_contract_response(app: &mut RcdAdminApp, json_response: String) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: GenerateContractReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        let mut result_message = String::new();

        result_message =
            result_message + &format!("Is result successful: {}", reply.is_successful.to_string());

        console::log_1(&result_message.to_string().clone().into());
        app
            
            .contract
            .generate
            .result
            .data
            .is_successful = reply.is_successful;
    }
}

pub fn handle_view_active_contract(app: &mut RcdAdminApp, json_response: String) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: GetActiveContractReply = serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        let contract_markdown =
            formatter::markdown::contract::contract_to_markdown_table(&reply.contract.unwrap());
        app.contract.data.active.markdown = contract_markdown;
    }
}

pub fn handle_send_contract_to_participant_response(app: &mut RcdAdminApp, json_response: String) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: SendParticipantContractReply =
        serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        // app.contract.send.result.is_successful = reply.is_sent;
        app.participants.data.result.send_contract = reply.is_sent;
    }
}

pub fn handle_get_pending_contract_response(json_response: String) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: ViewPendingContractsReply =
        serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        todo!()
    }
}

#[allow(dead_code)]
fn remote_delete_behavior_status_to_text(behavior: u32) -> String {
    match behavior {
        0 => "Unknown".to_string(),
        1 => "Ignore".to_string(),
        2 => "AutoDelete".to_string(),
        3 => "UpdateStatusOnly".to_string(),
        _ => "Unknown".to_string(),
    }
}
