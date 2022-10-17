use super::SqlClientImpl;
use rcdproto::rcdp::{GetDatabasesReply, GetDatabasesRequest};

#[allow(dead_code, unused_variables)]
pub async fn get_databases(
    request: GetDatabasesRequest,
    client: &SqlClientImpl,
) -> GetDatabasesReply {
    unimplemented!()

    /*
    we need to add a function to dbi to get a list of database names
    and then we want to add to DatabaseSchema the database type (Mysql, Sqlite) and the rcd database type (host, partial)

    for every database name that we have, we should be able to call dbi.get_database_schema and get the resulting schema
    and return that back to the caller if authenticated
    */

}
