use config::Config;
use fern::colors::{Color, ColoredLevelConfig};
use tracing::{debug, info};
use rcd_client::RcdClient;
use srv::TrackingServer;
use tracing_subscriber::EnvFilter;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tracing_subscriber::util::SubscriberInitExt;

pub mod error;
mod srv;

#[derive(Debug, Clone)]
pub struct ApiSettings {
    pub id: String,
    pub proxy_addr: String,
    pub proxy_user: String,
    pub proxy_auth: String,
    pub shark_ip: String,
    pub shark_port: u16,
}

impl ApiSettings {
    pub async fn get_rcd_client(&self) -> RcdClient {
        let mut client = RcdClient::new_grpc_client(
            self.proxy_addr.clone(),
            self.proxy_user.clone(),
            self.proxy_auth.clone(),
            60,
        )
        .await;

        client.set_host_id(&self.id);
        client
    }
}

#[tokio::main]
async fn main() {
    //init_log_to_screen_fern(log::LevelFilter::Trace);
    init_to_screen();

    let settings_location: String;

    let settings_status = has_settings();
    if !settings_status.0 {
        settings_location = create_default_settings();
    } else {
        settings_location = settings_status.1.as_ref().unwrap().to_string();
    }

    let settings = read_settings(&settings_location);
    let server = TrackingServer::new(&settings.shark_ip, settings.shark_port);
    server.start(settings).await.unwrap();
}

fn init_to_screen() {
    let filter = EnvFilter::builder()
       .parse_lossy("rcd=trace,tracking=trace");

    println!("{filter:?}");

    let subscriber = 
    tracing_subscriber::fmt().compact()
    .with_file(true)
    .with_line_number(true)
    .with_target(true)
    .with_env_filter(filter)
    .finish();
    
    subscriber.init();
 }

#[allow(dead_code)]
fn init_log_to_screen_fern(level: log::LevelFilter) {
    use ignore_result::Ignore;

    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Blue)
        .error(Color::BrightRed)
        .warn(Color::Magenta)
        .trace(Color::BrightWhite);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .level_for("tokio", log::LevelFilter::Off)
        .level_for("hyper", log::LevelFilter::Off)
        .level_for("rocket", log::LevelFilter::Off)
        .level_for("h2", log::LevelFilter::Off)
        .level_for("tower", log::LevelFilter::Off)
        .level_for("_", log::LevelFilter::Off)
        .level_for("mio", log::LevelFilter::Off)
        .level_for("tracing", log::LevelFilter::Off)
        .level_for("tokio_util", log::LevelFilter::Off)
        .level_for("want", log::LevelFilter::Off)
        .level_for("tonic", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .ignore();
}

fn has_settings() -> (bool, Option<String>) {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();
    let settings_filename = "Settings.toml";

    let settings_in_cwd = Path::new(cwd).join(settings_filename.clone());

    let settings_location = if Path::exists(&settings_in_cwd) {
        settings_in_cwd.to_str().unwrap()
    } else {
        "src/Settings"
    };

    if Path::exists(Path::new(settings_location)) {
        return (true, Some(settings_location.to_string()));
    }

    (false, None)
}

fn read_settings(settings_location: &str) -> ApiSettings {
    let error_message = format!(
        "{}{}{}{}",
        "Could not find ",
        "Settings.toml",
        "in current directory or in default ",
        settings_location
    );

    let settings = Config::builder()
        .add_source(config::File::with_name(settings_location))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect(&error_message);

    info!("Using settings file: {settings_location}");

    let id = settings.get_string(&String::from("id")).unwrap();
    let proxy_addr = settings.get_string(&String::from("proxy_addr")).unwrap();
    let proxy_user = settings.get_string(&String::from("proxy_user")).unwrap();
    let proxy_auth = settings.get_string(&String::from("proxy_auth")).unwrap();
    let shark_ip = settings.get_string(&String::from("shark_ip")).unwrap();
    let shark_port = settings.get_string(&String::from("shark_port")).unwrap();

    let settings = ApiSettings {
        id: id,
        proxy_addr: proxy_addr,
        proxy_user: proxy_user,
        proxy_auth: proxy_auth,
        shark_ip: shark_ip,
        shark_port: shark_port.parse().unwrap(),
    };

    debug!("{settings:?}");
    settings
}

fn create_default_settings() -> String {
    let cwd = get_current_directory();
    let default_settings_content = String::from(
        r#"
id = "871551FA-34EE-61A7-D792-F4401B8C8318"
proxy_addr = "http://proxy.home:50051"
proxy_user = "shark"
proxy_auth = "shark"
    "#,
    );

    let default_src_path = Path::new(&cwd).join("src/Settings.toml");
    let path = Path::new(&cwd).join("Settings.toml");
    if !Path::exists(&default_src_path) && !Path::exists(&path) {
        println!(
            "creating default Settings.toml at: {}",
            &path.to_str().unwrap()
        );
        let mut output = File::create(path.clone()).unwrap();
        write!(output, "{default_settings_content}").unwrap();
        return path.clone().to_str().unwrap().to_string();
    } else {
        println!("Settings.toml was found, skipping default settings");

        if Path::exists(&path) {
            return path.to_str().unwrap().to_string();
        } else {
            return default_src_path.to_str().unwrap().to_string();
        }
    }
}

fn get_current_directory() -> String {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap().to_string();
    cwd
}
