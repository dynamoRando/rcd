use crate::RcdAdminApp;
use yew::html;
use yew::{html::Scope, Html};

pub fn view_columns_for_table(app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.state.page_ui.databases_is_visible;
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
            let mut column_info = String::new();
            column_info = column_info + &column.column_name.clone();
            column_info = column_info + &" - ";
            column_info = column_info + &get_datatype_for_column_num(column.column_type);
            if column.column_length > 0 {
                column_info = column_info + " - " + &column.column_length.to_string();
            }

            col_names.push(column_info.clone());
        }

        html! {
           <div hidden={is_visible}>
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

fn get_datatype_for_column_num(data_type: u32) -> String {
    return match data_type {
        0 => "Unknown".to_string(),
        1 => "Int".to_string(),
        2 => "Bit".to_string(),
        3 => "Char".to_string(),
        4 => "DateTime".to_string(),
        5 => "Decimal".to_string(),
        6 => "Varchar".to_string(),
        7 => "Binary".to_string(),
        8 => "Varbinary".to_string(),
        9 => "Text".to_string(),
        _ => "Unknown".to_string(),
    };
}
