use std::thread;

use log::{debug, error};
use rcd_proxy_container::rcd_docker::RcdDocker;
// use rcd_sqlite_log::SqliteLog;

fn main() {
    thread::spawn(move || {
        docker_up();
    })
    .join()
    .unwrap();
}

#[tokio::main]
async fn docker_up() {
    debug!("connecting");
    let result = RcdDocker::new("tcp://127.0.0.1:2375".to_string());
    match result {
        Ok(docker) => {
            docker.list_docker_containers().await;
            docker.list_docker_images().await;
            let _ = docker.new_rcd_container(&"test".to_string()).await;
        }
        Err(e) => {
            error!("{e}")
        }
    }
}

#[allow(dead_code)]
fn log_test() {
    // SqliteLog::init(LevelFilter::Trace).unwrap();

    // debug!("this is a debug");
    // warn!("this is a warn");
    // info!("this is an info");
    // error!("this is an error");
    // trace!("this is a trace");

    // let entries = SqliteLog::default_get_last_x_logs(10);

    // for e in &entries {
    //     let message = format!("{} {} {} {}", e.dt, e.dt_utc, e.level, e.message);
    //     debug!("{message}");
    // }
}
