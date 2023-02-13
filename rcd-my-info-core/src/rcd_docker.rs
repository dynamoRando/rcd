use anyhow::Result;
use docker_api::{
    opts::{ContainerCreateOpts, ContainerListOptsBuilder},
    Docker,
};

use crate::{container_error::CreateContainerError};

#[derive(Debug)]
#[allow(dead_code)]
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
                docker,
            })
        } else {
            Err("Error: could not connect to docker".to_string())
        }
    }

    pub async fn new_rcd_container(&self, name: &String) -> Result<String, CreateContainerError> {
        if !self.has_container(name).await.unwrap() {
            let opts = ContainerCreateOpts::builder()
                .name(name)
                .image("rcd:latest")
                .build();
            let result = self.docker.containers().create(&opts).await;
            match result {
                Ok(container) => Ok(container.id().to_string()),
                Err(err) => return Err(CreateContainerError::DockerError(format!("{err}"))),
            }
        } else {
            Err(CreateContainerError::ContainerAlreadyExists)
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
        match self.docker.images().list(&Default::default()).await {
            Ok(images) => {
                for image in images {
                    println!("{:?}", image.repo_tags);
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    pub async fn has_container(&self, name: &String) -> Result<bool, String> {
        let containers = self
            .docker
            .containers()
            .list(&ContainerListOptsBuilder::default().all(true).build())
            .await;

        match containers {
            Ok(containers) => {
                for container in containers {
                    let names = container.names.unwrap();
                    // println!("{:?}", names);
                    let n = name.to_string();

                    for x in names {
                        if x == n {
                            return Ok(true);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }

        Ok(false)
    }

    pub async fn get_container_id(&self, name: &String) -> Result<Option<String>, String> {
        match self
            .docker
            .containers()
            .list(&ContainerListOptsBuilder::default().all(true).build())
            .await
        {
            Ok(containers) => {
                for container in containers {
                    let names = container.names.as_ref().unwrap();

                    for n in names {
                        if n == name {
                            // println!("found container id {:?}", &container);
                            return Ok(Some(container.id.unwrap()));
                        }
                    }

                    return Ok(None);
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }

        Ok(None)
    }

    pub async fn remove_container(&self, name: &String) -> Result<bool, String> {
        if self.has_container(name).await.unwrap() {
            let id = self.get_container_id(name).await.unwrap().unwrap();

            let result = self.docker.containers().get(id).delete().await;
            match result {
                Ok(_) => Ok(true),
                Err(err) => return Err(format!("Error: {err}")),
            }
        } else {
            Ok(false)
        }
    }

    pub async fn list_docker_containers(&self) {
        match self
            .docker
            .containers()
            .list(&ContainerListOptsBuilder::default().all(true).build())
            .await
        {
            Ok(containers) => {
                for container in containers {
                    println!("{:?}", container.names.unwrap());
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
