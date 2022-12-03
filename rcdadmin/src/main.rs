use rcd_ui::{
    RcdConn, RcdConnUi, RcdContractGenUi, RcdContractInfo, RcdInputOutputUi, RcdPageUi,
    RcdTablePolicy,
};
use serde::Deserialize;
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
mod ui;
mod urls;

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
}

pub enum ContractIntent {
    Unknown,
    GetPending,
    GetAccepted,
    GetRejected,
    AcceptContract(String),
    GenerateContract,
    SendContractToParticipant(String),
    RejectContract(String),
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
    SetExecuteSQLDatabase(String),
    HandleContract(ContractIntent),
    HandleTablePolicy(TableIntent),
    HandleTablePolicyResponse(AttrValue),
    HandleTablePolicyUpdateResponse(AttrValue),
    HandleToggleVisiblity(UiVisibility),
}

struct ApplicationState {
    conn_ui: RcdConnUi,
    page_ui: RcdPageUi,
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
        conn::view_input_for_connection(self, link)
    }

    pub fn view_databases(&self, link: &Scope<Self>) -> Html {
        db::view_databases(self, link)
    }

    pub fn view_tables_for_database(&self, link: &Scope<Self>) -> Html {
        db::view_tables::view_tables_for_database(self, link)
    }

    pub fn view_columns_for_table(&self, _link: &Scope<Self>) -> Html {
        db::view_columns::view_columns_for_table(self, _link)
    }

    pub fn view_input_for_sql(&self, link: &Scope<Self>) -> Html {
        sql::view_input_for_sql(self, link)
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

    pub fn view_participants(&self, _link: &Scope<Self>) -> Html {
        participant::view_participants(self, _link)
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
        let conn = RcdConn {
            un: "tester".to_string(),
            pw: "123456".to_string(),
            ip: "localhost".to_string(),
            port: 8000,
            databases: Vec::new(),
            current_db_name: "".to_string(),
            current_table_name: "".to_string(),
            sql_input: "".to_string(),
            sql_output: "".to_string(),
            url: "".to_string(),
            auth_request_json: "".to_string(),
        };

        let policy = RcdTablePolicy {
            db_name: "".to_string(),
            table_name: "".to_string(),
            policy: 0,
            policy_text: "".to_string(),
            policy_node: NodeRef::default(),
            new_policy: NodeRef::default(),
        };

        let con_gen = RcdContractGenUi {
            host_name_ui: NodeRef::default(),
            contract_desc_ui: NodeRef::default(),
        };

        let ci = RcdContractInfo {
            contract_detail_ui: NodeRef::default(),
            contract_markdown: "".to_string(),
            pending_contracts: Vec::new(),
            accepted_contracts: Vec::new(),
            contract_gen_ui: con_gen,
        };

        let input_output = RcdInputOutputUi {
            execute_sql: NodeRef::default(),
            sql_result: NodeRef::default(),
            db_name: NodeRef::default(),
            selected_db_name: "".to_string(),
            current_policy: policy,
            current_contract: ci,
        };

        let conn_ui = RcdConnUi {
            conn,
            un: NodeRef::default(),
            pw: NodeRef::default(),
            ip: NodeRef::default(),
            port: NodeRef::default(),
            databases: NodeRef::default(),
            sql: input_output,
            sql_text_result: "".to_string(),
            current_selected_table: NodeRef::default(),
        };

        let page_ui = RcdPageUi {
            conn_is_visible: true,
            contract_is_visible: true,
            host_is_visible: true,
            databases_is_visible: true,
            sql_is_visible: true,
            participants_is_visible: true,
            behaviors_is_visible: true,
            coop_hosts_is_visible: true,
        };

        let app_state = ApplicationState { conn_ui, page_ui };

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
        // console::log_1(&"update".into());
        match msg {
            AppMessage::Connect() => conn::handle_connect(self, ctx),
            AppMessage::GetDatabases(db_response) => db::handle_get_databases(self, db_response),
            AppMessage::GetTablesForDatabase(db_name) => {
                db::handle_get_tables_for_database(self, db_name, ctx)
            }
            AppMessage::GetColumnsForTable(db_name, table_name) => {
                db::handle_get_columns_for_table(self, db_name, table_name, ctx)
            }
            AppMessage::ExecuteSQL(intent) => sql::handle_execute_sql(self, ctx, intent),
            AppMessage::SQLReadResult(json_response) => {
                sql::handle_sql_read_result(self, ctx, json_response)
            }
            AppMessage::SetExecuteSQLDatabase(db_name) => db::handle_execute_sql_db(self, db_name),
            AppMessage::HandleContract(_) => todo!(),
            AppMessage::HandleTablePolicy(intent) => policy::handle_table_policy(intent, self, ctx),
            AppMessage::HandleTablePolicyResponse(json_response) => {
                policy::handle_table_response(json_response, self)
            }
            AppMessage::HandleTablePolicyUpdateResponse(json_response) => {
                policy::handle_table_update_response(json_response, self)
            }
            AppMessage::HandleToggleVisiblity(ui) => handle_ui_visibility(ui, self),
            AppMessage::SQLWriteResult(json_response) => {
                sql::handle_sql_write_result(self, ctx, json_response)
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
            app.state.page_ui.conn_is_visible = is_visible;
        }
        UiVisibility::SQL(is_visible) => 
        {
            app.state.page_ui.sql_is_visible = is_visible;
        },
        UiVisibility::Databases(is_visible) => {
            app.state.page_ui.databases_is_visible = is_visible;
        },
        UiVisibility::Contract(is_visible) => 
        {
            app.state.page_ui.contract_is_visible = is_visible;
        },
        UiVisibility::Host(is_visible) => {
            app.state.page_ui.host_is_visible = is_visible;
        },
        UiVisibility::Participant(is_visible) => {
            app.state.page_ui.participants_is_visible = is_visible;
        },
        UiVisibility::Behaviors(is_visible) => {
            app.state.page_ui.behaviors_is_visible = is_visible;
        },
        UiVisibility::CoopHosts(is_visible) => {
            app.state.page_ui.coop_hosts_is_visible = is_visible;
        },
    }
}
