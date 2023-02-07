
#[derive(Debug)]
pub struct RcdDocker {
    docker_ip: String
}

impl RcdDocker {
    pub fn new(ip_addr_port: String) -> Self {
        RcdDocker {
            docker_ip: ip_addr_port
        }
    }

    pub async fn list_docker_images(&self) {
        let docker = docker_api::Docker::new(&self.docker_ip).unwrap();
    
        match docker.images().list(&Default::default()).await {
            Ok(images) => {
                for image in images {
                    println!("{:?}", image.repo_tags);
                }
            }
            Err(e) => eprintln!("Something bad happened! {e}"),
        }
    }

    async fn list_docker_containers(&self) {
        let docker = docker_api::Docker::new(&self.docker_ip).unwrap();
    
        match docker.containers().list(&Default::default()).await {
            Ok(containers) => {
                for container in containers {
                    println!("{:?}", container.names.unwrap());
                }
            }
            Err(e) => eprintln!("Something bad happened! {}", e),
        }
    }
    
}
