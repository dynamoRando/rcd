use config::Config;
use fern::colors::{Color, ColoredLevelConfig};
use lazy_static::lazy_static;
use log::{debug, info, LevelFilter};
use srv::TrackingServer;
use std::env;
use std::path::Path;
use std::sync::Mutex;

pub mod error;
mod srv;

#[derive(Debug, Clone)]
pub struct ApiSettings {
    pub id: String,
    pub proxy_addr: String,
    pub proxy_user: String,
    pub proxy_auth: String,
}

#[tokio::main]
async fn main() {
    init_log_to_screen_fern(LevelFilter::Trace);
    let settings = read_settings();
    let server = TrackingServer::new("0.0.0.0", 8020);
    server.start(settings).await.unwrap();
}

fn init_log_to_screen_fern(level: LevelFilter) {
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

fn read_settings() -> ApiSettings {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();
    let settings_filename = "Settings.toml";

    let settings_in_cwd = Path::new(cwd).join(settings_filename.clone());

    let settings_location = if Path::exists(&settings_in_cwd) {
        settings_in_cwd.to_str().unwrap()
    } else {
        "src/Settings"
    };

    let error_message = format!(
        "{}{}{}{}",
        "Could not find ",
        settings_filename,
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

    ApiSettings {
        id: id,
        proxy_addr: proxy_addr,
        proxy_user: proxy_user,
        proxy_auth: proxy_auth,
    }
}
