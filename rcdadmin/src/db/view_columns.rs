use yew::html;
use yew::{html::Scope, Html};
use crate::RcdAdminApp;

pub fn view_columns_for_table(app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    let db_name = app.state.conn_ui.conn.current_db_name.clone();
    let table_name = app.state.conn_ui.conn.current_table_name.clone();

    if db_name == "" || table_name == "" {
        html! {
            <div/>
        }
    } else {
        let table = app
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
