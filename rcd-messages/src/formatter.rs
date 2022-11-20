use crate::client::Row;
use indexmap::IndexMap;

/// takes a Vec of rows and formats a table similar to MySQL
#[allow(dead_code, unused_variables)]
pub fn rows_to_string_table(rows: Vec<Row>) -> String {
    todo!()
}

/// takes a Vec of rows and formats a table similiar to Markdown
pub fn rows_to_string_markdown_table(rows: &[Row]) -> String {
    let max_lengths = build_max_lengths_for_columns(&rows);

    let mut markdown_table = String::new();

    let mut total_length_of_table = 0;

    for length in max_lengths.values() {
        // for every column, we also want to pad both sides with a | divider plus a space
        total_length_of_table = total_length_of_table + length + 3;
    }

    // add the column names
    for lengths in &max_lengths {
        markdown_table = markdown_table + "|";
        let pad_length = lengths.1 - lengths.0.len() as u32 - 1;
        markdown_table = markdown_table + " " + lengths.0;
        markdown_table =
            markdown_table + format!(" {:<width$}", "", width = pad_length as usize).as_str();
    }

    markdown_table = markdown_table + "|";
    markdown_table = markdown_table + "\n";

    // add the header seperator
    for lengths in &max_lengths {
        markdown_table = markdown_table + "|";
        let pad_length = lengths.1 - 1;
        markdown_table =
            markdown_table + format!(" {:-<width$} ", "", width = pad_length as usize).as_str();
    }

    markdown_table = markdown_table + "|";
    markdown_table = markdown_table + "\n";

    for row in rows {
        markdown_table = markdown_table + "|";
        for value in &row.values {
            let col_name = value.column.as_ref().unwrap().column_name.clone();
            let col_max = max_lengths.get_key_value(&col_name).unwrap();
            let max_length = *col_max.1;
            markdown_table = markdown_table + " " + &value.string_value.clone();

            // println!("max_length: {:?}", max_length);
            // println!("max_length: {:?}", value.string_value.len());

            // let pad_length = max_length - *&value.string_value.len() as u32 - 1;
            let pad_length = max_length - *&value.string_value.len() as u32;

            markdown_table =
                markdown_table + format!("{:<width$}", "", width = pad_length as usize).as_str();
            markdown_table = markdown_table + "|";
        }

        markdown_table = markdown_table + "\n";
    }

    return markdown_table;
}

fn build_max_lengths_for_columns(rows: &[Row]) -> IndexMap<String, u32> {
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

    // println!("{:?}", max_lengths);

    return max_lengths;
}

/*
# String Padding In Rust
- https://stackoverflow.com/questions/50458144/what-is-the-easiest-way-to-pad-a-string-with-0-to-the-left
- https://stackoverflow.com/questions/64810657/how-to-pad-left-in-rust
- https://stackoverflow.com/questions/69067436/how-do-i-make-the-fill-padding-in-stdformat-dynamic
 */
