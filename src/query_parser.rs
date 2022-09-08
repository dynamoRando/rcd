use crate::rcd_enum::DmlType;


#[allow(dead_code, unused_variables)]
/// Takes a SQL statement and returns a list of tables involved in that SQL statement
pub fn get_table_names(cmd: &str) -> Vec<String> {
    unimplemented!();
}

#[allow(dead_code, unused_variables)]
pub fn get_table_name(cmd: &str) -> String {
    unimplemented!()
}

#[allow(dead_code, unused_variables)]
pub fn determine_dml_type(cmd: &str) -> DmlType {
    unimplemented!()
}