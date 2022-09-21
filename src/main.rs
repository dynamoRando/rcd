use rcd::defaults;
use std::io::Write;
use std::{env, fs::File, io, path::Path};
use tokio::task;

#[tokio::main]
async fn main() {
    println!("rcd version {}.", defaults::VERSION);

    let args: Vec<String> = env::args().collect();
    process_cmd_args(args);

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

fn process_cmd_args(args: Vec<String>) {
    if args.len() >= 2 {
        let cmd = &args[1];
        if cmd == "default_settings" {
            let cwd = rcd::get_current_directory();
            let default_settings_content = String::from(
                "
debug = false
database_type = 1
backing_database_name = \"rcd.db\"
rcd_schema = \"rcd\"
client_service_addr_port = \"[::1]:50051\"
data_service_addr_port = \"[::1]:50052\"
admin_un = \"tester\"
admin_pw = \"123456\"
            ",
            );

            let default_src_path = Path::new(&cwd).join("src/Settings.toml");
            if !Path::exists(&default_src_path) {
                let path = Path::new(&cwd).join("Settings.toml");
                if !Path::exists(&path) {
                    println!(
                        "creating default Settings.toml at: {}",
                        &path.to_str().unwrap()
                    );
                    let mut output = File::create(path).unwrap();
                    write!(output, "{}", default_settings_content).unwrap();
                }
            } else {
            }
            println!("Settings.toml was found, skipping default settings");
        }
    }
}
