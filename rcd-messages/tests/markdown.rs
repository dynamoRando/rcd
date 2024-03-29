use tracing::debug;
use rcd_messages::{
    client::{ColumnSchema, Contract, DatabaseSchema, Host, Row, RowValue, TableSchema},
    formatter::{
        self,
        markdown::{contract, db},
    },
};

#[test]
pub fn test_markdown_rows() {
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

    let values: Vec<RowValue> = vec![rv1_name, rv1_text, rv1_address];

    let rv_1 = Row {
        database_name: "Example".to_string(),
        table_name: "Test".to_string(),
        row_id: 1,
        values,
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

    let values: Vec<RowValue> = vec![rv2_name, rv2_text, rv2_address];

    let rv_2 = Row {
        database_name: "Example".to_string(),
        table_name: "Test".to_string(),
        row_id: 2,
        values,
        is_remoteable: false,
        remote_metadata: None,
        hash: Vec::new(),
    };

    rows.push(rv_2);

    let rv3_name = RowValue {
        column: Some(cs_name),
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

    let values: Vec<RowValue> = vec![rv3_name, rv3_text, rv3_address];

    let rv_3 = Row {
        database_name: "Example".to_string(),
        table_name: "Test".to_string(),
        row_id: 2,
        values,
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

    debug!("{md_table}");
    debug!("{result_table}");

    assert_eq!(md_table, result_table);
}

#[test]
pub fn test_host() {
    let host = Host {
        host_guid: "76A9AC34-B28C-DC39-09A6-59F401E496C7".to_string(),
        host_name: "Example".to_string(),
        ip4_address: "127.0.0.1".to_string(),
        ip6_address: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
        database_port_number: 5050,
        token: Vec::new(),
        http_addr: String::from(""),
        http_port: 0,
    };

    let table = formatter::markdown::host::host_to_markdown_table(&host);

    debug!("{table}");

    let md_table = r#"| Key          | Value                                    |
| ------------ | ---------------------------------------- |
| GUID:        | 76A9AC34-B28C-DC39-09A6-59F401E496C7     |
| Host Name:   | Example                                  |
| IP 4:        | 127.0.0.1                                |
| IP 6:        | 2001:0db8:85a3:0000:0000:8a2e:0370:7334  |
| Db Port:     | 5050                                     |
| Token:       |                                          |
| HTTP Addr:   |                                          |
| HTTP Port:   | 0                                        |
"#;

    assert_eq!(table, md_table)
}

#[test]
pub fn test_database_schema() {
    let cs11 = ColumnSchema {
        column_name: "Id".to_string(),
        column_type: 1,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cs12 = ColumnSchema {
        column_name: "Name".to_string(),
        column_type: 9,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cv1: Vec<ColumnSchema> = vec![cs11, cs12];

    let ts1 = TableSchema {
        table_name: "EMPLOYEE".to_string(),
        table_id: "EMPLOYEE".to_string(),
        database_name: "TEST".to_string(),
        database_id: "TEST".to_string(),
        columns: cv1,
        logical_storage_policy: 1,
    };

    let cs21 = ColumnSchema {
        column_name: "Id".to_string(),
        column_type: 1,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cs22 = ColumnSchema {
        column_name: "Address".to_string(),
        column_type: 9,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cv2: Vec<ColumnSchema> = vec![cs21, cs22];

    let ts2 = TableSchema {
        table_name: "ADDRESS".to_string(),
        table_id: "ADDRESS".to_string(),
        database_name: "TEST".to_string(),
        database_id: "TEST".to_string(),
        columns: cv2,
        logical_storage_policy: 2,
    };

    let tv: Vec<TableSchema> = vec![ts1, ts2];

    let ds = DatabaseSchema {
        database_name: "TEST".to_string(),
        database_id: "TEST".to_string(),
        tables: tv,
        database_type: 1,
        rcd_database_type: 2,
        cooperation_enabled: false,
        has_participants: false,
    };

    let md = db::full_database_schema_to_tables(&ds);

    debug!("{md}");

    let md_expect = r#"
Tables: 
| Key       | Value             |
| --------- | ----------------- |
| EMPLOYEE  | HostOnly          |
| ADDRESS   | ParticipantOwned  |

Table Details: 
EMPLOYEE
| Key   | Value  |
| ----- | ------ |
| Id    | Int    |
| Name  | Text   |

ADDRESS
| Key      | Value  |
| -------- | ------ |
| Id       | Int    |
| Address  | Text   |
"#;

    assert_eq!(md, md_expect)
}

#[test]
pub fn test_contract() {
    let cs11 = ColumnSchema {
        column_name: "Id".to_string(),
        column_type: 1,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cs12 = ColumnSchema {
        column_name: "Name".to_string(),
        column_type: 9,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cv1: Vec<ColumnSchema> = vec![cs11, cs12];

    let ts1 = TableSchema {
        table_name: "EMPLOYEE".to_string(),
        table_id: "EMPLOYEE".to_string(),
        database_name: "TEST".to_string(),
        database_id: "TEST".to_string(),
        columns: cv1,
        logical_storage_policy: 1,
    };

    let cs21 = ColumnSchema {
        column_name: "Id".to_string(),
        column_type: 1,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cs22 = ColumnSchema {
        column_name: "Address".to_string(),
        column_type: 9,
        column_length: 0,
        is_nullable: false,
        ordinal: 1,
        table_id: "EMPLOYEE".to_string(),
        column_id: "".to_string(),
        is_primary_key: false,
    };

    let cv2: Vec<ColumnSchema> = vec![cs21, cs22];

    let ts2 = TableSchema {
        table_name: "ADDRESS".to_string(),
        table_id: "ADDRESS".to_string(),
        database_name: "TEST".to_string(),
        database_id: "TEST".to_string(),
        columns: cv2,
        logical_storage_policy: 2,
    };

    let tv: Vec<TableSchema> = vec![ts1, ts2];

    let ds = DatabaseSchema {
        database_name: "TEST".to_string(),
        database_id: "TEST".to_string(),
        tables: tv,
        database_type: 1,
        rcd_database_type: 2,
        cooperation_enabled: false,
        has_participants: false,
    };

    let host = Host {
        host_guid: "76A9AC34-B28C-DC39-09A6-59F401E496C7".to_string(),
        host_name: "Example".to_string(),
        ip4_address: "127.0.0.1".to_string(),
        ip6_address: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
        database_port_number: 5050,
        token: Vec::new(),
        http_addr: String::from(""),
        http_port: 0,
    };

    let contract = Contract {
        contract_guid: "76A9AC34-B28C-DC39-09A6-59F401E496C7".to_string(),
        description: "This is a test contract".to_string(),
        schema: Some(ds),
        contract_version: "76A9AC34-B28C-DC39-09A6-59F401E496C7".to_string(),
        host_info: Some(host),
        status: 2,
    };

    let md = contract::contract_to_markdown_table(&contract);

    debug!("{md}");

    let md_expect = r#"Contract Details: 
| Key          | Value                                 |
| ------------ | ------------------------------------- |
| GUID         | 76A9AC34-B28C-DC39-09A6-59F401E496C7  |
| Description  | This is a test contract               |
| Status       | Pending                               |
| Version      | 76A9AC34-B28C-DC39-09A6-59F401E496C7  |

Database Schema: 

Tables: 
| Key       | Value             |
| --------- | ----------------- |
| EMPLOYEE  | HostOnly          |
| ADDRESS   | ParticipantOwned  |

Table Details: 
EMPLOYEE
| Key   | Value  |
| ----- | ------ |
| Id    | Int    |
| Name  | Text   |

ADDRESS
| Key      | Value  |
| -------- | ------ |
| Id       | Int    |
| Address  | Text   |

Host: 
| Key          | Value                                    |
| ------------ | ---------------------------------------- |
| GUID:        | 76A9AC34-B28C-DC39-09A6-59F401E496C7     |
| Host Name:   | Example                                  |
| IP 4:        | 127.0.0.1                                |
| IP 6:        | 2001:0db8:85a3:0000:0000:8a2e:0370:7334  |
| Db Port:     | 5050                                     |
| Token:       |                                          |
| HTTP Addr:   |                                          |
| HTTP Port:   | 0                                        |

"#;

    assert_eq!(md, md_expect);
}
