mod rcd;

#[path = "rcd/crypt.rs"] 
pub mod crypt;
#[path = "rcd/db_srv.rs"] 
pub mod db_srv;
#[path = "rcd/rcd_db.rs"] 
pub mod rcd_db;
#[path = "rcd/client_srv.rs"] 
pub mod client_srv;
#[path = "rcd/sqlitedb.rs"] 
pub mod sqlitedb;
pub mod cdata;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    rcd::hello();
    let service = rcd::get_service_from_config_file();
    println!("{:?}", service);
    service.start();

    // https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
    tokio::task::spawn_blocking(move || {
        service.start_client_service();
    }).await.expect("Task panicked");

    Ok(())
}
