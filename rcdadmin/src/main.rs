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

pub enum ParticipantIntent{
    Unknown,
    Add,
    View
}

/// Format: State (Module) - Sub Action If Applicable - Set or Http (Request/Resonse) - Detail
#[allow(non_camel_case_types)]
pub enum AppMessage {
    Connect(),
    Db_HttpResponse_GetDatabases(AttrValue),
    Db_SetAndView_Tables(String),
    Db_SetAndView_Columns(String, String),
    Db_Set_ActiveDatabase(String),
    Sql_HttpRequest(ExecuteSQLIntent),
    Sql_HttpResponse_ReadResult(AttrValue),
    Sql_HttpResponse_WriteResult(AttrValue),
    Sql_HttpResponse_CooperativeWriteResult(AttrValue),
    Sql_Set_ActiveParticipant(String),
    Contract_Generate_Set_RemoteDeleteBehavior(u32),
    Contract_HttpRequest(ContractIntent),
    Contract_HttpResponse_GenerateContract(AttrValue),
    Contract_HttpResponse_SendToParticipant(AttrValue),
    Contract_HttpResponse_GetActiveContract(AttrValue),
    Contract_HttpResponse_GetPendingContracts(AttrValue),
    Policy_HttpRequest(TableIntent),
    Policy_HttpResponse_GetPolicy(AttrValue),
    Policy_HttpResponse_SetPolicy(AttrValue),
    Page_Set_Visibility(UiVisibility),
    Participant_HttpRequest(ParticipantIntent),
    Participant_HttpResponse_Add(AttrValue),
    Participant_HttpResponse_view(AttrValue),
}

pub struct RcdAdminApp {
    pub page: PageUi,
    pub connection: RcdConnection,
    pub databases: RcdDatabases,
    pub tables: RcdTables,
    pub sql: RcdSql,
    pub participants: RcdParticipants,
    pub contract: RcdContract,
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
        conn::view_input_for_connection(&self.page, link, &self.connection)
    }

    pub fn view_databases(&self, link: &Scope<Self>) -> Html {
        db::view_databases(&self.page, link, &self.databases)
    }

    pub fn view_tables_for_database(&self, link: &Scope<Self>) -> Html {
        db::view_tables::view_tables_for_database(
            &self.page,
            link,
            &self.databases,
            &self.tables,
        )
    }

    pub fn view_columns_for_table(&self, _link: &Scope<Self>) -> Html {
        db::view_columns::view_columns_for_table(
            &self.page,
            &self.databases,
            &self.tables,
        )
    }

    pub fn view_input_for_sql(&self, link: &Scope<Self>) -> Html {
        sql::view_input_for_sql(
            &self.page,
            link,
            &self.databases,
            &self.participants,
            &self.sql,
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
            &self.page,
            link,
            &self.databases,
            &self.participants,
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
        return init_app()
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
            AppMessage::Connect() => conn::handle_connect(&mut self.connection, ctx),
            AppMessage::Db_HttpResponse_GetDatabases(db_response) => db::handle_get_databases(self, db_response),
            AppMessage::Db_SetAndView_Tables(db_name) => {
                self.databases.data.active.database_name = db_name.clone();
                self.tables.data.active.database_name = db_name.clone();
                db::handle_get_tables_for_database(self, ctx)
            }
            AppMessage::Db_SetAndView_Columns(db_name, table_name) => {
                db::handle_get_columns_for_table(self, db_name, table_name, ctx)
            }
            AppMessage::Sql_HttpRequest(intent) => sql::handle_execute_sql(self, ctx, intent),
            AppMessage::Sql_HttpResponse_ReadResult(json_response) => {
                sql::handle_sql_read_result(self, ctx, json_response)
            }
            AppMessage::Db_Set_ActiveDatabase(db_name) => db::handle_execute_sql_db(self, db_name),
            AppMessage::Contract_HttpRequest(intent) => {
                contract::handle_contract_intent(self, intent, ctx.link())
            }
            AppMessage::Policy_HttpRequest(intent) => policy::handle_table_policy(intent, self, ctx),
            AppMessage::Policy_HttpResponse_GetPolicy(json_response) => {
                policy::handle_table_response(json_response, &mut self.tables)
            }
            AppMessage::Policy_HttpResponse_SetPolicy(json_response) => {
                policy::handle_table_update_response(json_response, self)
            }
            AppMessage::Page_Set_Visibility(ui) => handle_ui_visibility(ui, self),
            AppMessage::Sql_HttpResponse_WriteResult(json_response) => {
                sql::handle_sql_write_result(self, ctx, json_response)
            }
            AppMessage::Contract_HttpResponse_GenerateContract(json_response) => {
                contract::handle_contract_response(self, json_response.to_string())
            }
            AppMessage::Contract_Generate_Set_RemoteDeleteBehavior(behavior) => {
                self.contract.generate.data.delete_behavior = behavior;
            }
            AppMessage::Participant_HttpResponse_Add(json_response) => {
                participant::handle_add_participant_response(self, ctx, json_response)
            }
            AppMessage::Participant_HttpResponse_view(json_response) => {
                participant::handle_view_participant_response(self, ctx, json_response)
            }
            AppMessage::Contract_HttpResponse_GetActiveContract(json_response) => {
                contract::handle_view_active_contract(self, json_response.to_string())
            }
            AppMessage::Contract_HttpResponse_SendToParticipant(json_response) => {
                contract::handle_send_contract_to_participant_response(
                    self,
                    json_response.to_string(),
                )
            }
            AppMessage::Sql_Set_ActiveParticipant(participant_alias) => {
                sql::handle_set_sql_participant(&participant_alias, self, ctx);
            }
            AppMessage::Sql_HttpResponse_CooperativeWriteResult(json_response) => {
                sql::handle_cooperative_write_result(self, ctx, json_response);
            }
            AppMessage::Contract_HttpResponse_GetPendingContracts(json_response) => {
                contract::handle_get_pending_contract_response(json_response.to_string());
            }
            AppMessage::Participant_HttpRequest(intent) => {
                match intent {
                    ParticipantIntent::Unknown => todo!(),
                    ParticipantIntent::Add => participant::handle_add_participant(self, ctx),
                    ParticipantIntent::View => participant::handle_view_participants(self, ctx),
                }
            },
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
            app.page.conn_is_visible = is_visible;
        }
        UiVisibility::SQL(is_visible) => {
            app.page.sql_is_visible = is_visible;
        }
        UiVisibility::Databases(is_visible) => {
            app.page.databases_is_visible = is_visible;
        }
        UiVisibility::Contract(is_visible) => {
            app.page.contract_is_visible = is_visible;
        }
        UiVisibility::Host(is_visible) => {
            app.page.host_is_visible = is_visible;
        }
        UiVisibility::Participant(is_visible) => {
            app.page.participants_is_visible = is_visible;
        }
        UiVisibility::Behaviors(is_visible) => {
            app.page.behaviors_is_visible = is_visible;
        }
        UiVisibility::CoopHosts(is_visible) => {
            app.page.coop_hosts_is_visible = is_visible;
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

fn init_app() -> RcdAdminApp {
   
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

    let app = RcdAdminApp {
        page: page_ui,
        connection: RcdConnection::new(),
        databases: RcdDatabases::new(),
        tables: RcdTables::new(),
        sql: RcdSql::new(),
        participants: RcdParticipants::new(),
        contract: RcdContract::new(),
    };

    return app;
}
