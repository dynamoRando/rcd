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
        get_images();
    })
    .join()
    .unwrap();
}

#[tokio::main]
async fn get_images() {
    let result = RcdDocker::new("tcp://127.0.0.1:2375".to_string());
    if let Ok(docker) = result {
        let result = docker.get_docker_images().await;
        if let Ok(images) = result {
            for image in &images {
                println!("{}", image);
            }

            let name = r#"["rcd:latest"]"#;
            let has_name = images.contains(&name.to_string());
            assert!(has_name);
        } else {
            println!("{}", DOCKER_NOT_RUNNING_MESSAGE);
        }
    }
}
