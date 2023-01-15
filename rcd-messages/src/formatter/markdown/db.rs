use indexmap::IndexMap;

use crate::client::DatabaseSchema;

use super::{build_markdown_key_value_table, table};

/// returns a series of markdown tables representing the entire database
pub fn full_database_schema_to_tables(database: &DatabaseSchema) -> String {
    let mut total_schema: String = String::new();

    total_schema = total_schema + "\n";
    total_schema = total_schema + "Tables: ";
    total_schema = total_schema + "\n";

    total_schema = total_schema + &database_schema_to_markdown_table(database);

    total_schema = total_schema + "\n";
    total_schema = total_schema + "Table Details: ";

    for table in &database.tables {
        total_schema = total_schema + "\n";
        total_schema = total_schema + &table.table_name.clone();
        total_schema = total_schema + "\n";
        total_schema = total_schema + &table::table_schema_to_markdown_table(table);
    }

    return total_schema;
}

/// returns a list of tables and their logical storage policy in the database
pub fn database_schema_to_markdown_table(database: &DatabaseSchema) -> String {
    let mut kv: IndexMap<String, String> = IndexMap::new();

    for table in &database.tables {
        let policy_name = get_string_logical_storage_policy(table.logical_storage_policy);
        kv.insert(table.table_name.clone(), policy_name);
    }

    return build_markdown_key_value_table(kv);
}

fn get_string_logical_storage_policy(policy_num: u32) -> String {
    return match policy_num {
        0 => "None".to_string(),
        1 => "HostOnly".to_string(),
        2 => "ParticipantOwned".to_string(),
        3 => "Shared".to_string(),
        4 => "Mirror".to_string(),
        _ => "Unknown".to_string(),
    };
}
