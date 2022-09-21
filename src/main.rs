use std::io;

use rcd::defaults;
use tokio::task;

#[tokio::main]
async fn main() {
    println!("rcd version {}.", defaults::VERSION);

    let mut service = rcd::get_service_from_config_file();
    println!("rcd settings found:");
    println!("{:?}", service.rcd_settings);
    println!("root dir: {}", service.root_dir);
    service.start();

    let settings = service.rcd_settings.clone();
    let db_name = settings.backing_database_name.clone();
    let client_port = settings.client_service_addr_port.clone();
    let db_port = settings.database_service_addr_port.clone();
    let root_dir = service.root_dir.clone();

    let _ = task::spawn_blocking(move || {
        let _ =
            service.start_services_at_addrs(db_name, client_port, db_port, root_dir.to_string());
    })
    .await;

    let mut input = String::from("");
    println!("rcd is running. please press 'q' and enter to quit.");

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.contains("q") {
            break;
        }
    }

    println!("rcd is exiting. i remain obediently yours.");
}
