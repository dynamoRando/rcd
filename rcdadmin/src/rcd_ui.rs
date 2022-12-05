use rcd_messages::client::{DatabaseSchema, Contract, ParticipantStatus};
use serde::{Deserialize, Serialize};
use yew::{Component, Html, NodeRef};

use crate::AppMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct RcdPageUi {
    pub conn_is_visible: bool,
    pub databases_is_visible: bool,
    pub sql_is_visible: bool,
    pub contract_is_visible: bool,
    pub host_is_visible: bool,
    pub participants_is_visible: bool,
    pub behaviors_is_visible: bool,
    pub coop_hosts_is_visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RcdConn {
    pub un: String,
    pub pw: String,
    pub ip: String,
    pub port: u32,
    pub databases: Vec<DatabaseSchema>,
    pub current_db_name: String,
    pub current_table_name: String,
    pub sql_input: String,
    pub sql_output: String,
    pub url: String,
    pub auth_request_json: String,
}

pub struct RcdConnUi {
    pub conn: RcdConn,
    pub un: NodeRef,
    pub pw: NodeRef,
    pub ip: NodeRef,
    pub port: NodeRef,
    pub databases: NodeRef,
    pub sql: RcdInputOutputUi,
    pub sql_text_result: String,
    pub current_selected_table: NodeRef,
    pub add_participant_ui: RcdAddParticipantUi,
}

impl RcdConnUi {}

impl Component for RcdConnUi {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        todo!()
    }
}

#[allow(dead_code)]
pub struct RcdInputOutputUi {
    pub execute_sql: NodeRef,
    pub sql_result: NodeRef,
    pub db_name: NodeRef,
    pub selected_db_name: String,
    pub current_policy: RcdTablePolicy,
    pub current_contract: RcdContractInfo,
}

pub struct RcdTablePolicy {
    pub db_name: String,
    pub table_name: String,
    pub policy: u32,
    pub policy_text: String,
    pub policy_node: NodeRef,
    pub new_policy: NodeRef,
}

pub struct RcdContractInfo {
    pub contract_detail_ui: NodeRef,
    pub contract_markdown: String,
    pub active_contract_markdown: String,
    pub pending_contracts: Vec<Contract>,
    pub accepted_contracts: Vec<Contract>,
    pub contract_gen_ui: RcdContractGenUi,
    pub contract_detail_db_ui: NodeRef
}

pub struct RcdContractGenUi {
    pub host_name_ui: NodeRef,
    pub contract_desc_ui: NodeRef,
    pub contract_gen_remote_delete_behavior: u32,
    pub last_gen_result: bool,
}

pub struct RcdAddParticipantUi {
    pub alias_ui: NodeRef,
    pub ip4_address_ui: NodeRef,
    pub port_num_ui: NodeRef,
    pub last_add_result: bool,
    pub current_participants: Vec<ParticipantStatus>
}