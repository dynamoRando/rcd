use rcd_proxy::{proxy_server::ProxyServer, RcdProxy};
use simple_logger::SimpleLogger;
use std::{env, path::Path};

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();

    let dir = &cwd();
    let result_proxy = RcdProxy::get_proxy_from_config(dir);

    match result_proxy {
        Ok(proxy) => {
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
