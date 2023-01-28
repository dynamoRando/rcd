use crate::client::Row;
use indexmap::IndexMap;

pub mod markdown;

/// takes a Vec of rows and formats a table similar to MySQL

pub fn rows_to_string_table(_rows: Vec<Row>) -> String {
    todo!()
}

/// takes a Vec of rows and formats a table similiar to Markdown
pub fn rows_to_string_markdown_table(rows: &[Row]) -> String {
    markdown::rows::rows_to_string_markdown_table(rows)
}

pub fn build_max_lengths_for_columns(rows: &[Row]) -> IndexMap<String, u32> {
    let mut max_lengths: IndexMap<String, u32> = IndexMap::new();

    for row in rows {
        for value in &row.values {
            let col_name = value.column.as_ref().unwrap().column_name.clone();
            let col_length = col_name.len() + 4;

            if !max_lengths.contains_key(&col_name) {
                max_lengths.insert(col_name.clone(), col_length.try_into().unwrap());
            }

            if max_lengths.contains_key(&col_name) {
                let kv = max_lengths.get_key_value(&col_name).unwrap();
                let value_length = value.string_value.len() as u32;
                if value_length > *kv.1 {
                    // max_lengths.remove(&col_name);
                    // max_lengths.insert(col_name.clone(), value_length);
                    *max_lengths.get_mut(&col_name).unwrap() = value_length + 4;
                }
            }
        }
    }

    max_lengths
}
