use rcd_messages::{
    client::{ColumnSchema, Row, RowValue},
    formatter,
};

#[test]
pub fn test_markdown() {
    let mut rows: Vec<Row> = Vec::new();

    let cs_name = ColumnSchema {
        column_name: "Name".to_string(),
        column_type: 6,
        column_length: 10,
        is_nullable: true,
        ordinal: 1,
        table_id: "Test".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cs_text = ColumnSchema {
        column_name: "Text".to_string(),
        column_type: 6,
        column_length: 20,
        is_nullable: true,
        ordinal: 2,
        table_id: "Test".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cs_address = ColumnSchema {
        column_name: "Address".to_string(),
        column_type: 6,
        column_length: 50,
        is_nullable: true,
        ordinal: 2,
        table_id: "Test".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let rv1_name = RowValue {
        column: Some(cs_name.clone()),
        is_null_value: false,
        value: "Randy".as_bytes().to_vec(),
        string_value: "Randy".to_string(),
    };

    let rv1_text = RowValue {
        column: Some(cs_text.clone()),
        is_null_value: false,
        value: "This is a line of text".as_bytes().to_vec(),
        string_value: "This is a line of text".to_string(),
    };

    let rv1_address = RowValue {
        column: Some(cs_address.clone()),
        is_null_value: false,
        value: "1234 Main Street, Yuma, AZ 12345".as_bytes().to_vec(),
        string_value: "1234 Main Street, Yuma, AZ 12345".to_string(),
    };

    let mut values: Vec<RowValue> = Vec::new();
    values.push(rv1_name);
    values.push(rv1_text);
    values.push(rv1_address);

    let rv_1 = Row {
        database_name: "Example".to_string(),
        table_name: "Test".to_string(),
        row_id: 1,
        values: values,
        is_remoteable: false,
        remote_metadata: None,
        hash: Vec::new(),
    };

    rows.push(rv_1);

    let rv2_name = RowValue {
        column: Some(cs_name.clone()),
        is_null_value: false,
        value: "Rando".as_bytes().to_vec(),
        string_value: "Rando".to_string(),
    };

    let rv2_text = RowValue {
        column: Some(cs_text.clone()),
        is_null_value: false,
        value: "This is text".as_bytes().to_vec(),
        string_value: "This is text".to_string(),
    };

    let rv2_address = RowValue {
        column: Some(cs_address.clone()),
        is_null_value: false,
        value: "5678 Main Street, Tucson, AZ 54321".as_bytes().to_vec(),
        string_value: "5678 Main Street, Tucson, AZ 54321".to_string(),
    };

    let mut values: Vec<RowValue> = Vec::new();
    values.push(rv2_name);
    values.push(rv2_text);
    values.push(rv2_address);

    let rv_2 = Row {
        database_name: "Example".to_string(),
        table_name: "Test".to_string(),
        row_id: 2,
        values: values,
        is_remoteable: false,
        remote_metadata: None,
        hash: Vec::new(),
    };

    rows.push(rv_2);

    let rv3_name = RowValue {
        column: Some(cs_name.clone()),
        is_null_value: false,
        value: "Jimmy Tester Le".as_bytes().to_vec(),
        string_value: "Jimmy Tester Le".to_string(),
    };

    let rv3_text = RowValue {
        column: Some(cs_text),
        is_null_value: false,
        value: "More text goes here etc.".as_bytes().to_vec(),
        string_value: "More text goes here etc.".to_string(),
    };

    let rv3_address = RowValue {
        column: Some(cs_address),
        is_null_value: false,
        value: "9999 Brooklyn St, New York City, NY 123456"
            .as_bytes()
            .to_vec(),
        string_value: "9999 Brooklyn St, New York City, NY 123456".to_string(),
    };

    let mut values: Vec<RowValue> = Vec::new();
    values.push(rv3_name);
    values.push(rv3_text);
    values.push(rv3_address);

    let rv_3 = Row {
        database_name: "Example".to_string(),
        table_name: "Test".to_string(),
        row_id: 2,
        values: values,
        is_remoteable: false,
        remote_metadata: None,
        hash: Vec::new(),
    };

    rows.push(rv_3);

    let md_table = formatter::rows_to_string_markdown_table(&rows);

    let result_table = r"| Name               | Text                      | Address                                       |
| ------------------ | ------------------------- | --------------------------------------------- |
| Randy              | This is a line of text    | 1234 Main Street, Yuma, AZ 12345              |
| Rando              | This is text              | 5678 Main Street, Tucson, AZ 54321            |
| Jimmy Tester Le    | More text goes here etc.  | 9999 Brooklyn St, New York City, NY 123456    |
";

    println!("{}", md_table);
    println!("{}", result_table);

    assert_eq!(md_table, result_table);
}
