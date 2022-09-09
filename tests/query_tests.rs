pub mod sqlite {

    use rcd::query_parser::sqlite::foo;

    #[test]
    pub fn test_select() {
        let example = "SELECT col1, col2 FROM asdf;";
        let _ = foo(example.to_string());
        panic!()
    }

    #[test]
    pub fn test_insert() {
        let example = "INSERT INTO asdf ( col1, col2 ) VALUES ( 1, 'abcd');";
        let _ = foo(example.to_string());
        panic!()
    }

    #[test]
    pub fn test_update() {
        let example = "UPDATE asdf SET col1 = 'foo' WHERE col2 = 3;";
        let _ = foo(example.to_string());
        panic!()
    }

    #[test]
    pub fn test_delete() {
        let example = "DELETE FROM asdf WHERE col1 = 'a';";
        let _ = foo(example.to_string());
        panic!()
    }
}
