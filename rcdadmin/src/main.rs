use rcd_messages::client::AuthRequest;
use rcd_ui::{
    PageUi,
};
use serde::Deserialize;
use state::{
    connection::{RcdConnection, RcdConnectionData},
    contract::RcdContract,
    databases::RcdDatabases,
    participant::RcdParticipants,
    sql::RcdSql,
    tables::RcdTables,
    AdminUi,
};
use yew::{html::Scope, prelude::*, virtual_dom::AttrValue};

mod behaviors;
mod conn;
mod contract;
mod db;
mod host;
mod participant;
mod policy;
mod rcd_ui;
mod request;
mod sql;
mod state;
mod ui;

#[derive(Debug, Copy, Clone)]
pub enum UiVisibility {
    Connection(bool),
    Databases(bool),
    SQL(bool),
    Contract(bool),
    Host(bool),
    Participant(bool),
    Behaviors(bool),
    CoopHosts(bool),
}

// for testing, use the databases from the test "host_only"

pub enum ExecuteSQLIntent {
    Unknown,
    ReadAtHost,
    ReadAtPart,
    WriteAtHost,
    WriteAtPart,
    CoopWriteAtHost,
}

pub enum ContractIntent {
    Unknown,
    GetPending,
    GetAccepted,
    GetRejected,
    AcceptContract(String),
    GenerateContract,
    SetParticipantForPendingContractSend(String),
    SendContractToParticipant,
    RejectContract(String),
    ViewCurrentContract,
}

pub enum TableIntent {
    Unknown,
    // Database, Table
    GetTablePolicy((String, String)),
    /// Database, Table, Logical Storage Policy
    SetTablePolicy,
}

pub enum AppMessage {
    Connect(),
    GetDatabases(AttrValue),
    GetTablesForDatabase(String),
    GetColumnsForTable(String, String),
    ExecuteSQL(ExecuteSQLIntent),
    SQLReadResult(AttrValue),
    SQLWriteResult(AttrValue),
    SQLCooperativeWriteResult(AttrValue),
    SetExecuteSQLDatabase(String),
    SetExecuteSQLForParticipant(String),
    SetRemoteDeleteBehavior(u32),
    HandleContract(ContractIntent),
    HandleContractResponse(AttrValue),
    HandleContractSendToParticipant(AttrValue),
    HandleGetActiveContractResponse(AttrValue),
    HandleGetPendingContractResponse(AttrValue),
    HandleTablePolicy(TableIntent),
    HandleTablePolicyResponse(AttrValue),
    HandleTablePolicyUpdateResponse(AttrValue),
    HandleToggleVisiblity(UiVisibility),
    HandleAddParticipant,
    HandleAddParticipantResponse(AttrValue),
    HandleViewParticipants,
    HandleViewParticipantsResponse(AttrValue),
}

struct ApplicationState {
    page: PageUi,
    instance: AdminUi,
}

impl ApplicationState {}

pub struct RcdAdminApp {
    state: ApplicationState,
}

#[derive(Clone, PartialEq, Deserialize)]
struct AdminMsg {
    msg: String,
}

impl RcdAdminApp {
    pub fn view_ui_options(&self, link: &Scope<Self>) -> Html {
        ui::view_ui_options(self, link)
    }

    pub fn view_input_for_connection(&self, link: &Scope<Self>) -> Html {
        conn::view_input_for_connection(&self.state.page, link, &self.state.instance.connection)
    }

    pub fn view_databases(&self, link: &Scope<Self>) -> Html {
        db::view_databases(&self.state.page, link, &self.state.instance.databases)
    }

    pub fn view_tables_for_database(&self, link: &Scope<Self>) -> Html {
        db::view_tables::view_tables_for_database(
            &self.state.page,
            link,
            &self.state.instance.databases,
            &self.state.instance.tables,
        )
    }

    pub fn view_columns_for_table(&self, _link: &Scope<Self>) -> Html {
        db::view_columns::view_columns_for_table(
            &self.state.page,
            &self.state.instance.databases,
            &self.state.instance.tables,
        )
    }

    pub fn view_input_for_sql(&self, link: &Scope<Self>) -> Html {
        sql::view_input_for_sql(
            &self.state.page,
            link,
            &self.state.instance.databases,
            &self.state.instance.participants,
            &self.state.instance.sql,
        )
    }

    pub fn view_sql_result(&self, _link: &Scope<Self>) -> Html {
        sql::view_sql_result(self, _link)
    }

    pub fn view_contracts(&self, link: &Scope<Self>) -> Html {
        contract::view_contracts(self, link)
    }

    pub fn view_host_info(&self, _link: &Scope<Self>) -> Html {
        host::view_host_info(self, _link)
    }

    pub fn view_participants(&self, link: &Scope<Self>) -> Html {
        participant::view_participants(
            &self.state.page,
            link,
            &self.state.instance.databases,
            &self.state.instance.participants,
        )
    }

    pub fn view_write_behaviors(&self, _link: &Scope<Self>) -> Html {
        behaviors::view_write_behaviors(self, _link)
    }

    pub fn view_coop_hosts(&self, _link: &Scope<Self>) -> Html {
        host::view_coop_hosts(self, _link)
    }
}

impl Component for RcdAdminApp {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let app_state = init_state();
        Self { state: app_state }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {<div>
            <h1>{ "Rcd Admin" }</h1>
            <section class="ui">
            <header>
            { self.view_ui_options(ctx.link()) }
            </header>
        </section>
            <section class="rcdadmin">
                <header class="header">
                    { self.view_input_for_connection(ctx.link()) }
                </header>
            </section>
            <section class="databases">
                {self.view_databases(ctx.link())}
            </section>
            <section class="tables">
                {self.view_tables_for_database(ctx.link())}
            </section>
            <section class="columns">
                {self.view_columns_for_table(ctx.link())}
            </section>
            <section class="input_sql">
                {self.view_input_for_sql(ctx.link())}
            </section>
            <section class="sql_result">
                {self.view_sql_result(ctx.link())}
            </section>
            <section class="contracts">
                {self.view_contracts(ctx.link())}
            </section>
            <section class="host_info">
                {self.view_host_info(ctx.link())}
            </section>
            <section class="participants">
                {self.view_participants(ctx.link())}
            </section>
            <section class="behaviors">
                {self.view_write_behaviors(ctx.link())}
            </section>
            <section class="coop_hosts">
                {self.view_coop_hosts(ctx.link())}
            </section>
        </div>}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Connect() => conn::handle_connect(&mut self.state.instance.connection, ctx),
            AppMessage::GetDatabases(db_response) => db::handle_get_databases(self, db_response),
            AppMessage::GetTablesForDatabase(db_name) => {
                self.state.instance.databases.data.active.database_name = db_name.clone();
                self.state.instance.tables.data.active.database_name = db_name.clone();
                db::handle_get_tables_for_database(self, ctx)
            }
            AppMessage::GetColumnsForTable(db_name, table_name) => {
                db::handle_get_columns_for_table(self, db_name, table_name, ctx)
            }
            AppMessage::ExecuteSQL(intent) => sql::handle_execute_sql(self, ctx, intent),
            AppMessage::SQLReadResult(json_response) => {
                sql::handle_sql_read_result(self, ctx, json_response)
            }
            AppMessage::SetExecuteSQLDatabase(db_name) => db::handle_execute_sql_db(self, db_name),
            AppMessage::HandleContract(intent) => {
                contract::handle_contract_intent(self, intent, ctx.link())
            }
            AppMessage::HandleTablePolicy(intent) => policy::handle_table_policy(intent, self, ctx),
            AppMessage::HandleTablePolicyResponse(json_response) => {
                policy::handle_table_response(json_response, &mut self.state.instance.tables)
            }
            AppMessage::HandleTablePolicyUpdateResponse(json_response) => {
                policy::handle_table_update_response(json_response, self)
            }
            AppMessage::HandleToggleVisiblity(ui) => handle_ui_visibility(ui, self),
            AppMessage::SQLWriteResult(json_response) => {
                sql::handle_sql_write_result(self, ctx, json_response)
            }
            AppMessage::HandleContractResponse(json_response) => {
                contract::handle_contract_response(self, json_response.to_string())
            }
            AppMessage::SetRemoteDeleteBehavior(behavior) => {
                self.state.instance.contract.generate.data.delete_behavior = behavior;
            }
            AppMessage::HandleAddParticipant => participant::handle_add_participant(self, ctx),
            AppMessage::HandleAddParticipantResponse(json_response) => {
                participant::handle_add_participant_response(self, ctx, json_response)
            }
            AppMessage::HandleViewParticipants => participant::handle_view_participants(self, ctx),
            AppMessage::HandleViewParticipantsResponse(json_response) => {
                participant::handle_view_participant_response(self, ctx, json_response)
            }
            AppMessage::HandleGetActiveContractResponse(json_response) => {
                contract::handle_view_active_contract(self, json_response.to_string())
            }
            AppMessage::HandleContractSendToParticipant(json_response) => {
                contract::handle_send_contract_to_participant_response(
                    self,
                    json_response.to_string(),
                )
            }
            AppMessage::SetExecuteSQLForParticipant(participant_alias) => {
                sql::handle_set_sql_participant(&participant_alias, self, ctx);
            }
            AppMessage::SQLCooperativeWriteResult(json_response) => {
                sql::handle_cooperative_write_result(self, ctx, json_response);
            }
            AppMessage::HandleGetPendingContractResponse(json_response) => {
                contract::handle_get_pending_contract_response(json_response.to_string());
            }
        }
        true
    }
}

fn main() {
    yew::Renderer::<RcdAdminApp>::new().render();
}

fn handle_ui_visibility(item: UiVisibility, app: &mut RcdAdminApp) {
    match item {
        UiVisibility::Connection(is_visible) => {
            app.state.page.conn_is_visible = is_visible;
        }
        UiVisibility::SQL(is_visible) => {
            app.state.page.sql_is_visible = is_visible;
        }
        UiVisibility::Databases(is_visible) => {
            app.state.page.databases_is_visible = is_visible;
        }
        UiVisibility::Contract(is_visible) => {
            app.state.page.contract_is_visible = is_visible;
        }
        UiVisibility::Host(is_visible) => {
            app.state.page.host_is_visible = is_visible;
        }
        UiVisibility::Participant(is_visible) => {
            app.state.page.participants_is_visible = is_visible;
        }
        UiVisibility::Behaviors(is_visible) => {
            app.state.page.behaviors_is_visible = is_visible;
        }
        UiVisibility::CoopHosts(is_visible) => {
            app.state.page.coop_hosts_is_visible = is_visible;
        }
    }
}

pub fn get_base_address(connection: &RcdConnectionData) -> String {
    return connection.active.url.clone();
}

pub fn get_auth_request(connection: &RcdConnectionData) -> AuthRequest {
    let auth_json = &connection.active.authentication_json;
    let auth: AuthRequest = serde_json::from_str(&auth_json).unwrap();
    return auth;
}

fn init_state() -> ApplicationState {
   
    let page_ui = PageUi {
        conn_is_visible: true,
        contract_is_visible: true,
        host_is_visible: true,
        databases_is_visible: true,
        sql_is_visible: true,
        participants_is_visible: true,
        behaviors_is_visible: true,
        coop_hosts_is_visible: true,
    };

    let instance = init_admin();

    let state = ApplicationState {
        page: page_ui,
        instance: instance,
    };

    return state;
}

pub fn init_admin() -> AdminUi {
    let instance = AdminUi {
        connection: RcdConnection::new(),
        databases: RcdDatabases::new(),
        sql: RcdSql::new(),
        tables: RcdTables::new(),
        participants: RcdParticipants::new(),
        contract: RcdContract::new(),
    };

    return instance;
}
