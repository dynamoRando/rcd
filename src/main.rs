use std::thread;
mod rcd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    rcd::hello();
    let service = rcd::get_service_from_config_file();
    println!("{:?}", service);
    service.start();

    tokio::task::spawn_blocking(move || {
        service.start_client_service();
    }).await.expect("Task panicked");

    Ok(())
}
