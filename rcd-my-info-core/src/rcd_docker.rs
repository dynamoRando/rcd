use anyhow::Result;
use docker_api::{opts::ContainerCreateOpts, Docker};

#[derive(Debug)]
pub struct RcdDocker {
    docker_ip: String,
    docker: Docker,
}

impl RcdDocker {
    pub fn new(ip_addr_port: String) -> Result<Self, String> {
        let docker = docker_api::Docker::new(&ip_addr_port);
        if let Ok(docker) = docker {
            Ok(RcdDocker {
                docker_ip: ip_addr_port,
                docker: docker,
            })
        } else {
            return Err("could not connect to docker".to_string());
        }
    }

    pub async fn new_rcd_container(&self, name: &String) -> Result<bool, String> {
        if !self.has_container(name).await.unwrap() {
            let opts = ContainerCreateOpts::builder()
                .name(name)
                .image("rcd:latest")
                .build();
            let result = self.docker.containers().create(&opts).await;
            match result {
                Ok(_container) => Ok(true),
                Err(err) => return Err(format!("Something bad happened! {err}")),
            }
        } else {
            Ok(false)
        }
    }

    pub async fn get_docker_images(&self) -> Result<Vec<String>> {
        let images = self.docker.images().list(&Default::default()).await?;
        let mut image_names: Vec<String> = Vec::new();
        for image in images {
            let name = format!("{:?}", image.repo_tags);
            image_names.push(name);
        }

        Ok(image_names)
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

    pub async fn has_container(&self, name: &String) -> Result<bool, String> {
        let docker = docker_api::Docker::new(&self.docker_ip).unwrap();

        match docker.containers().list(&Default::default()).await {
            Ok(containers) => {
                for container in containers {
                    let names = container.names.unwrap();

                    return Ok(names.contains(name));
                }
            }
            Err(e) => eprintln!("Something bad happened! {e}"),
        }

        Ok(false)
    }

    pub async fn list_docker_containers(&self) {
        let docker = docker_api::Docker::new(&self.docker_ip).unwrap();

        match docker.containers().list(&Default::default()).await {
            Ok(containers) => {
                for container in containers {
                    println!("{:?}", container.names.unwrap());
                }
            }
            Err(e) => eprintln!("Something bad happened! {e}"),
        }
    }
}
