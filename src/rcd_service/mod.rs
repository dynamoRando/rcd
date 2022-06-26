use config::Config;
use std::collections::HashMap;

pub fn hello() {
    println!("hello rcd_service");
    read_config();
}

pub fn start() {
    unimplemented!("not completed yet");

    read_config();
    configure_backing_store();
}

/// reads the Settings.toml config file
fn read_config() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("src/rcd_service/Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let priority_setting = String::from("priority");

    let priority_value = settings.get_int(&priority_setting).unwrap();

    println!("{:?}", priority_value);

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    )
}

/// checks the backing database to see if it needs to be setup
fn configure_backing_store() {}
