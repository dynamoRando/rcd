use super::get_dbi;
use rcdproto::rcdp::{AuthResult, DatabaseSchema, GetDatabasesReply, GetDatabasesRequest};
use rocket::{post, http::Status, serde::json::Json};

#[allow(dead_code, unused_variables)]
#[post("/client/databases", format = "application/json", data = "<request>")]
pub fn post_get_databases(request: Json<GetDatabasesRequest>) -> (Status, Json<GetDatabasesReply>) {

    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let dbi = get_dbi();
    let mut db_result: Vec<DatabaseSchema> = Vec::new();

    let auth = request.authentication.as_ref().unwrap().clone();
    let is_authenticated = dbi.verify_login(&auth.user_name, &auth.pw);

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    if is_authenticated {
        let db_names = dbi.get_database_names();
        for name in &db_names {
            let db_schema = dbi.get_database_schema(&name);
            db_result.push(db_schema);
        }
    }

    let result = GetDatabasesReply {
        authentication_result: Some(auth_response),
        databases: db_result,
    };

    (Status::Ok, Json(result))
}
