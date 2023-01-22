use indexmap::IndexMap;

pub fn build_table(key_value: IndexMap<String, String>) -> String {
    let mut markdown_table = String::new();
    let mut keys: Vec<&str> = Vec::new();
    let mut values: Vec<&str> = Vec::new();

    for item in key_value.keys() {
        keys.push(item.as_str());
    }

    keys.push("Key");

    let max_length_key_strings = get_max_length_for_vec_strings(keys);

    for item in key_value.values() {
        values.push(item);
    }

    values.push("Value");

    let max_length_key_values = get_max_length_for_vec_strings(values);

    markdown_table = markdown_table
        + &build_markdown_row(
            &"Key".to_string(),
            &"Value".to_string(),
            max_length_key_strings,
            max_length_key_values,
        );

    markdown_table += "\n";

    markdown_table += "|";

    markdown_table = markdown_table + &build_markdown_seperator(max_length_key_strings + 1);

    markdown_table += "|";

    markdown_table = markdown_table + &build_markdown_seperator(max_length_key_values + 1);

    markdown_table += "|";

    markdown_table += "\n";

    for (k, v) in &key_value {
        markdown_table = markdown_table
            + &build_markdown_row(k, v, max_length_key_strings, max_length_key_values);
        markdown_table += "\n";
    }

    markdown_table
}


/// builds a markdown seperator without the "|" between
fn build_markdown_seperator(max_length: u32) -> String {
    let mut return_row = String::new();
    let pad_length = max_length;
    return_row += format!(" {:-<width$} ", "", width = pad_length as usize).as_str();

    return_row
}



/// takes a vec of strings and returns the max length found + 4 (to account for space and | for column)
fn get_max_length_for_vec_strings(items: Vec<&str>) -> u32 {
    let mut max_length: u32 = 0;

    for item in items {
        let item_length = item.len() as u32;

        if item_length >= max_length {
            max_length = item_length;
        }
    }

    max_length
}


/*
# String Padding In Rust
- https://stackoverflow.com/questions/50458144/what-is-the-easiest-way-to-pad-a-string-with-0-to-the-left
- https://stackoverflow.com/questions/64810657/how-to-pad-left-in-rust
- https://stackoverflow.com/questions/69067436/how-do-i-make-the-fill-padding-in-stdformat-dynamic
 */
 
fn build_markdown_row(
    key_string: &String,
    key_value: &String,
    key_string_max_length: u32,
    key_value_max_length: u32,
) -> String {
    let mut string_row: String = "".to_string();

    string_row += "| ";
    string_row = string_row + key_string;

    let mut pad_length: u32;
    pad_length = key_string_max_length - key_string.len() as u32;

    string_row += format!(" {:<width$} ", "", width = pad_length as usize).as_str();
    string_row += "| ";
    string_row = string_row + key_value;

    pad_length = key_value_max_length - key_value.len() as u32;

    string_row += format!(" {:<width$} ", "", width = pad_length as usize).as_str();
    string_row += "|";

    string_row
}
