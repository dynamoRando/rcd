use core::time;
use std::{env, thread};

use rcd::defaults;
use tokio::task;

#[tokio::main]
async fn main() {
    println!("rcd version {}. obediently yours.", defaults::VERSION);

    let mut service = rcd::get_service_from_config_file();
    println!("{:?}", service);
    service.start();

    let settings = service.rcd_settings.clone();
    let db_name = settings.backing_database_name.clone();
    let client_port = settings.client_service_addr_port.clone();
    let db_port = settings.database_service_addr_port.clone();

    let wd = env::current_dir().unwrap().clone();
    let cwd = wd.to_str().unwrap().to_string().clone();
    
    let _ = task::spawn_blocking(move || {
        let _ = service.start_services_at_addrs(db_name, client_port, db_port, cwd.to_string());
    }).await;
}
