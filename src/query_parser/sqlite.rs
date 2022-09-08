use crate::rcd_enum::DatabaseType;

#[derive(Debug, Clone, Copy)]
pub struct SqliteSqlParser {
    
}

impl SqliteSqlParser {
    #[allow(dead_code,unused_variables, unused_mut)]
    pub fn foo(sql_statement: String) {

    }
}


#[allow(dead_code, unused_variables)]
pub fn get_table_name(cmd: &str, db_type: DatabaseType) -> String {
    unimplemented!()
}