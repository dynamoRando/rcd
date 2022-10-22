#[macro_use] extern crate rocket;

pub mod rcd_service;
pub mod client;
pub mod data;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, client::status, client::version])
        .launch()
        .await?;

    Ok(())
}