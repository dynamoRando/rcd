use rcd_messages::{
    client::{
        AuthRequest, ExecuteReadReply, GetLogicalStoragePolicyReply,
        GetLogicalStoragePolicyRequest, SetLogicalStoragePolicyReply,
        SetLogicalStoragePolicyRequest,
    },
    formatter,
};
use rcd_ui::{RcdConn, RcdConnUi, RcdInputOutputUi, RcdTablePolicy};
use serde::Deserialize;
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use yew::{html::Scope, prelude::*, virtual_dom::AttrValue};

mod conn;
mod contract;
mod db;
mod host;
mod participant;
mod rcd_ui;
mod request;
mod sql;

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
    SQLResult(AttrValue),
    SetExecuteSQLDatabase(String),
    HandleContract(ContractIntent),
    HandleTablePolicy(TableIntent),
    HandleTablePolicyResponse(AttrValue),
    HandleTablePolicyUpdateResponse(AttrValue),
}

struct ApplicationState {
    conn_ui: RcdConnUi,
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
    pub fn view_input_for_connection(&self, link: &Scope<Self>) -> Html {
        conn::view_input_for_connection(self, link)
    }

    pub fn view_databases(&self, link: &Scope<Self>) -> Html {
        let mut db_names: Vec<String> = Vec::new();

        for db in &self.state.conn_ui.conn.databases {
            db_names.push(db.database_name.clone());
        }

        html! {
           <div>
           <h1> {"Databases"} </h1>
           <ul>
           {
            db_names.into_iter().map(|name| {
                let db_name = name.clone();
                html!{<div key={db_name.clone()}>
                <li onclick={link.callback(move |_| AppMessage::GetTablesForDatabase(name.clone()))}>{db_name.clone()}</li></div>}
            }).collect::<Html>()
        }</ul>
           </div>
        }
    }

    pub fn view_tables_for_database(&self, link: &Scope<Self>) -> Html {
        let db_name = self.state.conn_ui.conn.current_db_name.clone();
        let current_table_policy = self.state.conn_ui.sql.current_policy.policy_text.clone();

        if db_name == "" {
            html! {
                <div/>
            }
        } else {
            let tables = self
                .state
                .conn_ui
                .conn
                .databases
                .iter()
                .find(|x| x.database_name.as_str() == db_name)
                .unwrap()
                .tables
                .clone();

            let mut table_names: Vec<String> = Vec::new();
            for table in &tables {
                table_names.push(table.table_name.clone());
            }

            let table_names_clone = table_names.clone();

            html! {
               <div>
               <h1> {"Tables for database "}{&db_name}</h1>
               <ul>
               {
                table_names.into_iter().map(|name| {
                    let table_name = name.clone();
                    let d_name = db_name.clone();
                    html!{<div key={table_name.clone()}>
                    <li onclick={link.callback(move |_| AppMessage::GetColumnsForTable(d_name.clone(), table_name.clone()))}>{name.clone()}</li></div>}
                }).collect::<Html>()
            }</ul>
            <p>
            <h2>{"Table Policies"}</h2>
            <label for="table_policy">{ "Table Name" }</label>
            <select name="select_table_for_policy" id="table_policy"

            onchange={link.batch_callback(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                    // console::log_1(&"some onchange".into());
                    let table_name = input.value();
                    let d_name = db_name.clone();
                    Some(AppMessage::HandleTablePolicy(TableIntent::GetTablePolicy((d_name.clone(), table_name))))
                } else {
                    // console::log_1(&"none onchange".into());
                    None
                }
            })}
            >
            <option value="SELECT TABLE">{"SELECT TABLE"}</option>
            {
                table_names_clone.into_iter().map(|name| {
                    // console::log_1(&name.clone().into());
                    html!{
                    <option value={name.clone()}>{name.clone()}</option>}
                }).collect::<Html>()
            }
            </select>
            <label for="selected_table_policy">{"Current Policy:"}</label>
            <input type="text" id ="selected_table_policy" placeholder="Logical Storage Policy" ref={&self.state.conn_ui.sql.current_policy.policy_node}
             value={current_table_policy}/>
            <label for="table_policy_value">{ "Set Policy" }</label>
            <select name="set_policy_for_table" id="set_policy_for_table" ref={&self.state.conn_ui.sql.current_policy.new_policy}>
            /*
                None = 0,
                HostOnly = 1,
                ParticpantOwned = 2,
                Shared = 3,
                Mirror = 4,
             */
            <option value={"0"}>{"None"}</option>
            <option value={"1"}>{"Host Only"}</option>
            <option value={"2"}>{"Participant Owned"}</option>
            <option value={"3"}>{"Shared"}</option>
            <option value={"4"}>{"Mirror"}</option>
            </select>
            <input type="button" id="submit_new_table_policy" value="Update Policy" onclick={link.callback(move |_|
                {
                    console::log_1(&"submit_new_table_policy".into());

                    AppMessage::HandleTablePolicy(TableIntent::SetTablePolicy)
                })}/>
            </p>
               </div>
            }
        }
    }

    pub fn view_columns_for_table(&self, _link: &Scope<Self>) -> Html {
        let db_name = self.state.conn_ui.conn.current_db_name.clone();
        let table_name = self.state.conn_ui.conn.current_table_name.clone();

        if db_name == "" || table_name == "" {
            html! {
                <div/>
            }
        } else {
            let table = self
                .state
                .conn_ui
                .conn
                .databases
                .iter()
                .find(|x| x.database_name.as_str() == db_name)
                .unwrap()
                .tables
                .iter()
                .find(|x| x.table_name.as_str() == table_name)
                .unwrap()
                .clone();

            let mut col_names: Vec<String> = Vec::new();

            for column in &table.columns {
                col_names.push(column.column_name.clone());
            }

            html! {
               <div>
               <h1> {"Columns for table "}{&table_name} {" in database "}{&db_name}</h1>
               <ul>
               {
                col_names.into_iter().map(|name| {
                    let col_name = name.clone();
                    html!{<div key={col_name.clone()}>
                    <li>{col_name.clone()}</li></div>}
                }).collect::<Html>()
            }</ul>
               </div>
            }
        }
    }

    pub fn view_input_for_sql(&self, link: &Scope<Self>) -> Html {
        let mut db_names: Vec<String> = Vec::new();

        for db in &self.state.conn_ui.conn.databases {
            db_names.push(db.database_name.clone());
        }

        // console::log_1(&"view_input_for_sql".into());
        // console::log_1(&db_names.len().to_string().into());

        html! {
            <div>
            <h1> {"Execute SQL"} </h1>
            <label for="execute_sql">{ "Enter SQL" }</label>
            <p>
            <label for="execute_sql_dbs">{ "Select Database " }</label>
            <select name="execute_sql_dbs" id="execute_sql_dbs"

            onchange={link.batch_callback(|e: Event| {
                if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                    // console::log_1(&"some onchange".into());
                    Some(AppMessage::SetExecuteSQLDatabase(input.value()))
                } else {
                    // console::log_1(&"none onchange".into());
                    None
                }
            })}
            >
            <option value="SELECT DATABASE">{"SELECT DATABASE"}</option>
            {
                db_names.into_iter().map(|name| {
                    // console::log_1(&name.clone().into());
                    html!{
                    <option value={name.clone()}>{name.clone()}</option>}
                }).collect::<Html>()
            }
            </select>
            </p>
            <p>
            <textarea rows="5" cols="60"  id ="execute_sql" placeholder="SELECT * FROM TABLE_NAME" ref={&self.state.conn_ui.sql.execute_sql}/>
            </p>
            <input type="button" id="read_at_host" value="Execute Read At Host" onclick={link.callback(|_|
                {
                    AppMessage::ExecuteSQL(ExecuteSQLIntent::ReadAtHost)
                })}/>
                <input type="button" id="read_at_part" value="Execute Read At Part" onclick={link.callback(|_|
                {
                    AppMessage::ExecuteSQL(ExecuteSQLIntent::ReadAtPart)
                })}/>
                <input type="button" id="write_at_host" value="Execute Write At Host" onclick={link.callback(|_|
                {
                    AppMessage::ExecuteSQL(ExecuteSQLIntent::WriteAtHost)
                })}/>
                <input type="button" id="write_at_part" value="Execute Write At Part" onclick={link.callback(|_|
                {
                    AppMessage::ExecuteSQL(ExecuteSQLIntent::WriteAtPart)
                })}/>
            </div>
        }
    }

    pub fn view_sql_result(&self, _link: &Scope<Self>) -> Html {
        let text = self.state.conn_ui.sql_text_result.clone();

        html!(
          <div>
              <h1> {"SQL Results"} </h1>
              <label for="sql_result">{ "Results" }</label>
              <p>
              <textarea rows="5" cols="60"  id ="sql_Result" placeholder="SQL Results Will Be Displayed Here"
              ref={&self.state.conn_ui.sql.sql_result} value={text}/>
              </p>
              </div>
        )
    }

    pub fn view_contracts(&self, link: &Scope<Self>) -> Html {
        html!(
          <div>
              <h1> {"Contracts"} </h1>
              <p>
              <input type="button" id="view_pending_contracts" value="View Pending Contracts" onclick={link.callback(|_|
                  {
                      AppMessage::HandleContract(ContractIntent::GetPending)
                  })}/>
              <input type="button" id="view_accepted_contracts" value="View Accepted Contracts" onclick={link.callback(|_|
                  {
                      AppMessage::HandleContract(ContractIntent::GetAccepted)
                  })}/>
              <input type="button" id="accepted_contracts" value="Accept Contract" onclick={link.callback(|_|
                  {
                      AppMessage::HandleContract(ContractIntent::AcceptContract("".to_string()))
                  })}/>
                  <input type="button" id="reject_contracts" value="Reject Contract" onclick={link.callback(|_|
                    {
                        AppMessage::HandleContract(ContractIntent::RejectContract("".to_string()))
                    })}/>
              </p>
              </div>
        )
    }

    pub fn view_host_info(&self, _link: &Scope<Self>) -> Html {
        html!(
          <div>
              <h1> {"Host Info"} </h1>
              <p>
              </p>
              </div>
        )
    }

    pub fn view_participants(&self, _link: &Scope<Self>) -> Html {
        html!(
          <div>
              <h1> {"Participants"} </h1>
              <p>
              </p>
              </div>
        )
    }

    pub fn view_write_behaviors(&self, _link: &Scope<Self>) -> Html {
        html!(
          <div>
              <h1> {"Configure Incoming Behaviors (Update, Delete)"} </h1>
              <p>
              </p>
              </div>
        )
    }

    pub fn view_coop_hosts(&self, _link: &Scope<Self>) -> Html {
        html!(
          <div>
              <h1> {"Cooperating Hosts"} </h1>
              <p>
              </p>
              </div>
        )
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

        let input_output = RcdInputOutputUi {
            execute_sql: NodeRef::default(),
            sql_result: NodeRef::default(),
            db_name: NodeRef::default(),
            selected_db_name: "".to_string(),
            current_policy: policy,
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

        let app_state = ApplicationState { conn_ui };

        Self { state: app_state }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {<div>
            <h1>{ "Rcd Admin" }</h1>
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
            AppMessage::SQLResult(json_response) => {
                console::log_1(&json_response.to_string().clone().into());
                let read_reply: ExecuteReadReply =
                    serde_json::from_str(&&json_response.to_string()).unwrap();

                if read_reply.authentication_result.unwrap().is_authenticated {
                    let rows = read_reply.results.first().unwrap().rows.clone();

                    let sql_table_text = formatter::rows_to_string_markdown_table(&rows);
                    self.state.conn_ui.sql_text_result = sql_table_text;
                }
            }
            AppMessage::SetExecuteSQLDatabase(db_name) => {
                // console::log_1(&db_name.into());
                self.state.conn_ui.sql.selected_db_name = db_name.clone();
                console::log_1(&self.state.conn_ui.sql.selected_db_name.clone().into());
            }
            AppMessage::HandleContract(_) => todo!(),
            AppMessage::HandleTablePolicy(intent) => match intent {
                TableIntent::Unknown => todo!(),
                TableIntent::GetTablePolicy(data) => {
                    self.state.conn_ui.sql.current_policy.db_name = data.0.clone();
                    self.state.conn_ui.sql.current_policy.table_name = data.1.clone();

                    if data.1 == "SELECT TABLE" {
                        return true;
                    }

                    let auth_json = &self.state.conn_ui.conn.auth_request_json;
                    let auth: AuthRequest = serde_json::from_str(&auth_json).unwrap();

                    let request = GetLogicalStoragePolicyRequest {
                        authentication: Some(auth),
                        database_name: data.0.clone(),
                        table_name: data.1.clone(),
                    };

                    let request_json = serde_json::to_string(&request).unwrap();
                    let base_address = self.state.conn_ui.conn.url.clone();
                    let url = format!(
                        "{}{}",
                        base_address.clone(),
                        "/client/databases/table/policy/get"
                    );
                    let callback = ctx.link().callback(AppMessage::HandleTablePolicyResponse);

                    request::get_data(url, request_json, callback);
                }
                TableIntent::SetTablePolicy => {
                    let policy_node = &self.state.conn_ui.sql.current_policy.new_policy;
                    let policy_val = policy_node.cast::<HtmlInputElement>().unwrap().value();

                    let db = self.state.conn_ui.sql.current_policy.db_name.clone();
                    let table = self.state.conn_ui.sql.current_policy.table_name.clone();
                    let policy_num: u32 = policy_val.parse().unwrap();

                    let auth_json = &self.state.conn_ui.conn.auth_request_json;
                    let auth: AuthRequest = serde_json::from_str(&auth_json).unwrap();

                    let request = SetLogicalStoragePolicyRequest {
                        authentication: Some(auth),
                        database_name: db,
                        table_name: table,
                        policy_mode: policy_num,
                    };

                    let request_json = serde_json::to_string(&request).unwrap();
                    let base_address = self.state.conn_ui.conn.url.clone();
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
            },
            AppMessage::HandleTablePolicyResponse(json_response) => {
                console::log_1(&json_response.to_string().clone().into());
                let reply: GetLogicalStoragePolicyReply =
                    serde_json::from_str(&&json_response.to_string()).unwrap();

                if reply.authentication_result.unwrap().is_authenticated {
                    let policy_value = reply.policy_mode;
                    self.state.conn_ui.sql.current_policy.policy = policy_value;

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

                    self.state.conn_ui.sql.current_policy.policy_text = policy_name.to_string();
                }
            }
            AppMessage::HandleTablePolicyUpdateResponse(json_response) => {
                console::log_1(&json_response.to_string().clone().into());
                let reply: SetLogicalStoragePolicyReply =
                    serde_json::from_str(&&json_response.to_string()).unwrap();

                if reply.authentication_result.unwrap().is_authenticated {
                    let policy_update_result = reply.is_successful;
                    console::log_1(&"policy_update_response".to_string().clone().into());
                    console::log_1(&policy_update_result.to_string().clone().into());
                }
            }
        }
        true
    }
}

fn main() {
    yew::Renderer::<RcdAdminApp>::new().render();
}
