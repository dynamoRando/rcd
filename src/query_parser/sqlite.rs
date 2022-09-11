use antlr_rust::{
    common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory, InputStream,
};

use crate::rcd_enum::{DatabaseType, DmlType};

use self::{sqlitelexer::SQLiteLexer, sqliteparser::SQLiteParser, rcd_sqlite_listener::{RcdSqliteListener, DmlData}, rcd_insert_sqlite_listener::{RcdInsertSqliteListener, InsertData}};

mod rcd_insert_sqlite_listener;
mod rcd_sqlite_listener;
mod sqlitelexer;
mod sqlitelistener;
mod sqliteparser;

#[allow(dead_code, unused_variables, unused_mut)]
pub fn get_table_name(cmd: &str, db_type: DatabaseType) -> String {
    let tf = CommonTokenFactory::default();
    let input = InputStream::new(cmd.into());
    let mut lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let rcd_listener = RcdSqliteListener {
        statement_type: Box::new(DmlData {
            data: DmlType::Unknown,
            table_name: String::from(""),
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(rcd_listener));
    let result = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    return item.statement_type.table_name;
}

#[allow(dead_code, unused_variables, unused_mut)]
pub fn determine_statement_type(sql_text: String) -> DmlType {
    let text = sql_text.as_str();

    // println!("{}", sql_text);

    let tf = CommonTokenFactory::default();
    let input = InputStream::new(text.into());
    let mut lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let rcd_listener = RcdSqliteListener {
        statement_type: Box::new(DmlData {
            data: DmlType::Unknown,
            table_name: String::from(""),
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(rcd_listener));
    let result = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    return item.statement_type.data;
}

#[allow(unused_variables)]
pub fn get_values_from_insert_statement(insert_statement: &str) -> Vec<String> {
    let tf = CommonTokenFactory::default();
    let input = InputStream::new(insert_statement.into());
    let lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let rcd_listener = RcdInsertSqliteListener {
        insert_data: Box::new(InsertData{
            table_name: String::from(""),
            column_names: Vec::new(),
            values: Vec::new(),
            col_and_vals: Vec::new(),
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(rcd_listener));
    let result = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    return item.insert_data.values;
}