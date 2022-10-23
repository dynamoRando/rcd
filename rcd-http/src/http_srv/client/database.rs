use rcdproto::rcdp::{GetDatabasesRequest, GetDatabasesReply};
use rocket::{serde::{json::Json}, get};


#[allow(dead_code, unused_variables)]
#[get("/client/databases", format = "application/json", data = "<request>")]
pub fn get(request: Json<GetDatabasesRequest>) -> Json<GetDatabasesReply> { 
   
    unimplemented!()
    
 }