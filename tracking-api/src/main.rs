use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use srv::TrackingServer;

mod srv;


#[tokio::main]
async fn main() {
    init_log_to_screen_fern(LevelFilter::Debug);
    let server = TrackingServer::new("0.0.0.0",8020);
    server.start().await.unwrap();
}

fn init_log_to_screen_fern(level: LevelFilter) {
    use ignore_result::Ignore;

    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Blue)
        .error(Color::BrightRed)
        .warn(Color::Magenta);

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
        .chain(std::io::stdout())
        .apply()
        .ignore();
}
