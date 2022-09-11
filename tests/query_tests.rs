pub mod sqlite {
    pub mod determine_statement_type {
        use rcd::{query_parser::sqlite::determine_statement_type, rcd_enum::DmlType};
        #[test]
        pub fn unknown() {
            let example = "ABCD";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Unknown);
        }

        #[test]
        pub fn select() {
            let example = "SELECT col1, col2 FROM asdf;";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Select);
        }

        #[test]
        pub fn insert() {
            let example = "INSERT INTO asdf ( col1, col2 ) VALUES ( 1, 'abcd');";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Insert);
        }

        #[test]
        pub fn update() {
            let example = "UPDATE asdf SET col1 = 'foo' WHERE col2 = 3;";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Update);
        }

        #[test]
        pub fn delete() {
            let example = "DELETE FROM asdf WHERE col1 = 'a';";
            let statement_type = determine_statement_type(example.to_string());
            assert_eq!(statement_type, DmlType::Delete);
        }
    }

    pub mod determine_table_name {
        use rcd::{query_parser::sqlite::get_table_name, rcd_enum::DatabaseType};
        #[test]
        pub fn unknown() {
            let example = "ABCD";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, String::from(""));
        }

        #[test]
        pub fn select() {
            let example = "SELECT col1, col2 FROM asdf;";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }

        #[test]
        pub fn insert() {
            let example = "INSERT INTO asdf ( col1, col2 ) VALUES ( 1, 'abcd');";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }

        #[test]
        pub fn update() {
            let example = "UPDATE asdf SET col1 = 'foo' WHERE col2 = 3;";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }

        #[test]
        pub fn delete() {
            let example = "DELETE FROM asdf WHERE col1 = 'a';";
            let table_name = get_table_name(example, DatabaseType::Sqlite);
            assert_eq!(table_name, "asdf");
        }
    }
}
