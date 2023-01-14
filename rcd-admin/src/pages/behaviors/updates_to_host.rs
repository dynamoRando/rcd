use yew::{function_component, html, use_state_eq, Callback, Html};

use crate::{
    pages::common::{select_database::SelectDatabase, select_table::SelectTable},
    request::get_databases, log::log_to_console,
};

#[function_component]
pub fn UpdatesToHost() -> Html {
    let active_database = use_state_eq(move || String::from(""));
    let active_table_database = active_database.clone();
    let active_table = use_state_eq(move || String::from(""));

    let table_names = use_state_eq(move || {
        let x: Vec<String> = Vec::new();
        return x;
    });

    let onclick_db = {
        let table_names = table_names.clone();
        Callback::from(move |db_name: String| {
            let databases = get_databases();

            let database = databases
                .iter()
                .find(|x| x.database_name.as_str() == db_name)
                .unwrap()
                .clone();

            let mut names: Vec<String> = Vec::new();

            for table in &database.tables {
                names.push(table.table_name.clone());
            }

            table_names.set(names);
        })
    };

    let onclick_table = {
        Callback::from(move |table_name: String| {
            if table_name != "" {
                log_to_console(table_name.clone());
            }
        })
    };

    html! {
        <div>
            <p><h1 class="subtitle">{"Updates To Host"}</h1></p>
            <p><label for="databases">{ "Select Database " }</label></p>
            <p>< SelectDatabase active_db_name={active_database} onclick_db={onclick_db}/></p>
            <p><label for="tables">{ "Select Table " }</label></p>
            <p>< SelectTable
                active_database_name={active_table_database}
                active_table_name = {active_table}
                onclick_table={onclick_table}/>
            </p>
            <p>{"Current Behavior: "}</p>
        </div>
    }
}
