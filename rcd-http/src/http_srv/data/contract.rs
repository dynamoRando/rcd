use super::get_data;
use rcdproto::rcdp::{SaveContractRequest, SaveContractResult};
use rocket::{http::Status, post, serde::json::Json};

#[post("/data/contract/save", format = "application/json", data = "<request>")]
pub async fn save_contract(
    request: Json<SaveContractRequest>,
) -> (Status, Json<SaveContractResult>) {
    let result = get_data().save_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}
