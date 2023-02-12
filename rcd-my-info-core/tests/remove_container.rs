use crate::test_harness::DOCKER_NOT_RUNNING_MESSAGE;
use rcd_my_info_core::rcd_docker::RcdDocker;
use std::thread;

#[path = "test_harness.rs"]
mod test_harness;

// https://stackoverflow.com/questions/64216274/docker-desktop-for-mac-bind-to-tcp-port
// https://stackoverflow.com/questions/51119922/how-to-connect-to-docker-via-tcp-on-macos
// socat TCP-LISTEN:2375,range=127.0.0.1/32,reuseaddr,fork UNIX-CLIENT:/var/run/docker.sock
// brew install socat

#[test]
fn test() {
    thread::spawn(move || {
        let _ = remove_container();
    })
    .join()
    .unwrap();
}

#[tokio::main]
async fn remove_container() {
    let result = RcdDocker::new("tcp://127.0.0.1:2375".to_string());
    if let Ok(docker) = result {
        let container_name = "/remove_container".to_string();

        if docker.has_container(&container_name).await.unwrap() {
            println!("container {container_name} already exists");
            docker.remove_container(&container_name).await.unwrap();
        }

        let result = docker.new_rcd_container(&container_name).await;
        match result {
            Ok(create_result) => {
                assert!(create_result);

                if create_result {
                    let remove_result = docker.remove_container(&container_name).await;
                    match remove_result {
                        Ok(result) => {
                            assert!(result);
                        }
                        Err(e) => {
                            println!("{e}")
                        }
                    }
                }
            }
            Err(e) => {
                println!("{e}");
                println!("{}", DOCKER_NOT_RUNNING_MESSAGE);
            }
        }
    }
}
