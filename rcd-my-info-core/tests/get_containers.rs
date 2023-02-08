use std::thread;

use rcd_my_info_core::rcd_docker::RcdDocker;

#[test]
#[ignore = "need to have docker running"]
fn test_get_containers() {
    thread::spawn(move || {
        get_names();
    })
    .join()
    .unwrap();
}

#[tokio::main]
async fn get_names() {
    let docker = RcdDocker::new("tcp://127.0.0.1:2375".to_string());
    let images = docker.get_docker_images().await.unwrap();

    // for i in &images {
    //     println!("{:?}", i);
    // }
    
    let name = r#"["rcd:latest"]"#;
    
    // println!("{}", name);
    
    let has_name = images.contains(&name.to_string());
    assert!(has_name);
}
