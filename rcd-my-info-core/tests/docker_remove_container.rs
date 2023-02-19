use crate::test_harness::DOCKER_NOT_RUNNING_MESSAGE;

use rcd_my_info_core::rcd_docker::RcdDocker;
use simple_logger::SimpleLogger;
use std::thread;

use log::{debug, info, warn, error};

use std::sync::{Arc, Mutex};

use crate::test_harness::is_docker_running;

#[path = "test_harness.rs"]
mod test_harness;

// https://stackoverflow.com/questions/64216274/docker-desktop-for-mac-bind-to-tcp-port
// https://stackoverflow.com/questions/51119922/how-to-connect-to-docker-via-tcp-on-macos
// socat TCP-LISTEN:2375,range=127.0.0.1/32,reuseaddr,fork UNIX-CLIENT:/var/run/docker.sock
// brew install socat

#[test]
fn docker_remove_container() {
    // SimpleLogger::new().env().init().unwrap();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    thread::spawn(move || {
        remove_container();
    })
    .join()
    .unwrap();
}

#[tokio::main]
async fn remove_container() {
    let docker_status = false;
    let docker_status = Mutex::new(docker_status);
    let docker_status = Arc::new(docker_status);
    let docker_ip = "tcp://127.0.0.1:2375";

    {
        let docker_status = docker_status.clone();
        let docker_ip = docker_ip.clone();
        thread::spawn(move || {
            let mut data = docker_status.lock().unwrap();
            let is_running = is_docker_running(&docker_ip);
            debug!("is docker running: {is_running}");
            *data = is_running;
        })
        .join()
        .unwrap();
    }

    let docker_is_running = docker_status.lock().unwrap();

    if *docker_is_running {
        let result = RcdDocker::new(docker_ip.to_string());
        if let Ok(docker) = result {
            let container_name = "/remove_container".to_string();

            if docker.has_container(&container_name).await.unwrap() {
                warn!("container {container_name} already exists");
                docker.remove_container(&container_name).await.unwrap();
            }

            let result = docker.new_rcd_container(&container_name).await;
            match result {
                Ok(create_result) => {
                    info!("{create_result}");
                    assert!(create_result.len() > 0);

                    if create_result.len() > 0 {
                        let remove_result = docker.remove_container(&container_name).await;
                        match remove_result {
                            Ok(result) => {
                                assert!(result);
                            }
                            Err(e) => {
                                error!("{e}")
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("{e}");
                    error!("{}", DOCKER_NOT_RUNNING_MESSAGE);
                }
            }
        }
    } else {
        error!("{}", DOCKER_NOT_RUNNING_MESSAGE);
    }
}
