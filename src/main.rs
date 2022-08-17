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
    })
    .await
    .expect("Task panicked");

    Ok(())
}
