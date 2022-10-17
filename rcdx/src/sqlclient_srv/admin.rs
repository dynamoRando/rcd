use super::SqlClientImpl;
use rcdproto::rcdp::{GetDatabasesReply, GetDatabasesRequest, AuthResult, DatabaseSchema};

pub async fn get_databases(
    request: GetDatabasesRequest,
    client: &SqlClientImpl,
) -> GetDatabasesReply {

    println!("{:?}", request);

    let mut db_result: Vec<DatabaseSchema> = Vec::new();

    let message = request.clone();
    let a = message.authentication.unwrap();
    let is_authenticated = client.verify_login(&a.user_name, &a.pw);

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    if is_authenticated {
        let db_names = client.dbi().get_database_names();
        for name in &db_names {
            let db_schema = client.dbi().get_database_schema(&name);
            db_result.push(db_schema);
        }
    }

    let result =  GetDatabasesReply{
        authentication_result: Some(auth_response),
        databases: db_result,
    };

    return result;

    /*
    we need to add a function to dbi to get a list of database names
    and then we want to add to DatabaseSchema the database type (Mysql, Sqlite) and the rcd database type (host, partial)

    for every database name that we have, we should be able to call dbi.get_database_schema and get the resulting schema
    and return that back to the caller if authenticated
    */

}
