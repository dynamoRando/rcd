use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use rcd_proxy::{proxy_server::ProxyServer, RcdProxy};
use std::{env, path::Path};

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    process_cmd_args(args);

    // SimpleLogger::new().env().init().unwrap();
    init_log_to_screen_fern(LevelFilter::Trace);

    let dir = &cwd();
    let result_proxy = RcdProxy::get_proxy_from_config(dir);

    match result_proxy {
        Ok(proxy) => {
            let proxy = proxy.clone();
            proxy.start();
            proxy.start_grpc_client().await;
            proxy.start_grpc_data().await;
            let server = ProxyServer::new(proxy);
            if let Err(e) = server.start().await {
                println!("Error: {e:?}");
            }
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

fn cwd() -> String {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();
    let cur_dir = Path::new(cwd);
    cur_dir.to_str().unwrap().to_string()
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


fn process_cmd_args(args: Vec<String>){
    if args.len() >= 2 {
        let cmd = args[1].as_str();

        match cmd {
            "-v" => {
                let version = env!("CARGO_PKG_VERSION");
                println!("rcd-proxy version: {}", version);
            }
            _ => {},
        }
    }
}