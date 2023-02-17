use std::{env, path::Path, fs};

use log::info;
use rcd_my_info_core::rcd_docker::RcdDocker;

#[allow(dead_code)]
pub const DOCKER_NOT_RUNNING_MESSAGE: &str = "docker not running - test skipped";

#[allow(dead_code)]
/// Returns a blank directory in the $TMPDIR under the `RCD_MY_INFO` folder. If the 
/// directory already exists, it will delete it and create it.
pub fn get_test_temp_dir(test_name: &str) -> String {
    let dir = env::temp_dir();
    let tmp = dir.as_os_str().to_str().unwrap();
    let path = Path::new(&tmp).join("RCD_MY_INFO").join(test_name);

    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    fs::create_dir_all(&path).unwrap();

    return path.as_path().to_str().unwrap().to_string();
}

#[allow(dead_code)]
/// Checks docker to see if the container name already exists, and if so, will remove it
pub async fn remove_container_if_exists(docker_ip: &str, container_name: &str) {
    let result = RcdDocker::new(docker_ip.to_string());
    if let Ok(docker) = result {
        let container_name = container_name.to_string();
        if docker.has_container(&container_name).await.unwrap() {
            info!("container {container_name} already exists");
            docker.remove_container(&container_name).await.unwrap();
        }
    }
}