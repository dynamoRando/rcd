use rcdproto::rcdp::{CreateTableRequest, CreateTableResult};

use super::RcdData;

pub async fn create_table_in_database(
    core: &RcdData,
    request: CreateTableRequest,
) -> CreateTableResult {
    let auth_result = core.authenticate_host(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;
    let table_schema = request.columns;
    let mut table_is_created = false;
    let mut table_id = String::from("");
    let mut db_id = String::from("");

    if auth_result.0 {
        let result =
            core.dbi()
                .create_table_in_partial_database(&db_name, &table_name, table_schema);
        if result.is_ok() {
            table_is_created = true;
            table_id = core.dbi().get_table_id(&db_name, &table_name);
            db_id = core.dbi().get_db_id(db_name.as_str());
        }
    }

    CreateTableResult {
        authentication_result: Some(auth_result.1),
        is_successful: table_is_created,
        database_name: db_name,
        result_message: String::from(""),
        table_id,
        table_name,
        database_id: db_id,
    }
}
