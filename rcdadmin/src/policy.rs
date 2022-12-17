use crate::{request, state::tables::RcdTables, AppMessage, RcdAdminApp, TableIntent};
use rcd_messages::client::{
    AuthRequest, GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest,
    SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest,
};
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

pub fn handle_table_policy(intent: TableIntent, app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    match intent {
        TableIntent::Unknown => todo!(),
        TableIntent::GetTablePolicy(data) => {
            app.state.instance.tables.data.active.database_name = data.0.clone();
            app.state.instance.tables.data.active.table_name = data.1.clone();

            if data.1 == "SELECT TABLE" {
                return;
            }

            let auth_json = &app
                .state
                .instance
                .connection
                .data
                .active
                .authentication_json;
            let auth: AuthRequest = serde_json::from_str(&auth_json).unwrap();

            let request = GetLogicalStoragePolicyRequest {
                authentication: Some(auth),
                database_name: data.0.clone(),
                table_name: data.1.clone(),
            };

            let request_json = serde_json::to_string(&request).unwrap();
            let base_address = app.state.instance.connection.data.active.url;
            let url = format!(
                "{}{}",
                base_address.clone(),
                "/client/databases/table/policy/get"
            );
            let callback = ctx.link().callback(AppMessage::HandleTablePolicyResponse);

            request::get_data(url, request_json, callback);
        }
        TableIntent::SetTablePolicy => {
            let policy_node = &app.state.instance.tables.ui.new_policy;
            let policy_val = policy_node.cast::<HtmlInputElement>().unwrap().value();

            let db = app.state.instance.tables.data.active.database_name;
            let table = app.state.instance.tables.data.active.table_name;
            let policy_num: u32 = policy_val.parse().unwrap();

            let auth_json = &app
                .state
                .instance
                .connection
                .data
                .active
                .authentication_json;
            let auth: AuthRequest = serde_json::from_str(&auth_json).unwrap();

            let request = SetLogicalStoragePolicyRequest {
                authentication: Some(auth),
                database_name: db,
                table_name: table,
                policy_mode: policy_num,
            };

            let request_json = serde_json::to_string(&request).unwrap();
            let base_address = &app.state.instance.connection.data.active.url;
            let url = format!(
                "{}{}",
                base_address.clone(),
                "/client/databases/table/policy/set"
            );
            let callback = ctx
                .link()
                .callback(AppMessage::HandleTablePolicyUpdateResponse);

            request::get_data(url, request_json, callback);
        }
    }
}

pub fn handle_table_response(json_response: AttrValue, tables: &mut RcdTables) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: GetLogicalStoragePolicyReply =
        serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        let policy_value = reply.policy_mode;
        tables.data.active.policy_value = policy_value;

        /*
          None = 0,
          HostOnly = 1,
          ParticpantOwned = 2,
          Shared = 3,
          Mirror = 4,
        */

        let policy_name = match policy_value {
            0 => "None",
            1 => "Host Only",
            2 => "Participant Owned",
            3 => "Shared",
            4 => "Mirror",
            _ => "Unknown",
        };

        tables.data.active.policy_name = policy_name.to_string();
    }
}

pub fn handle_table_update_response(json_response: AttrValue, _app: &mut RcdAdminApp) {
    console::log_1(&json_response.to_string().clone().into());
    let reply: SetLogicalStoragePolicyReply =
        serde_json::from_str(&&json_response.to_string()).unwrap();

    if reply.authentication_result.unwrap().is_authenticated {
        let policy_update_result = reply.is_successful;
        console::log_1(&"policy_update_response".to_string().clone().into());
        console::log_1(&policy_update_result.to_string().clone().into());
    }
}
