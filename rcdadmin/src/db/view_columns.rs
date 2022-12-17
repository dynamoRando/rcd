use crate::rcd_ui::PageUi;
use crate::state::databases::RcdDatabases;
use crate::state::tables::RcdTables;
use yew::html;
use yew::Html;

pub fn view_columns_for_table(page: &PageUi, databases: &RcdDatabases, tables: &RcdTables) -> Html {
    let is_visible = !page.databases_is_visible;
    let db_name = tables.data.active.database_name.clone();
    let table_name = tables.data.active.table_name.clone();

    if db_name == "" || table_name == "" {
        html! {
            <div/>
        }
    } else {
        let table = databases
            .data
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
