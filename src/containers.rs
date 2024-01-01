
// Rocket
use rocket::get;
use rocket::http::Status;

// User
use crate::User;

// Docker
use bollard::container::CreateContainerOptions;
use bollard::container::ListContainersOptions;
use bollard::container::StartContainerOptions;
use bollard::container::Config;
use bollard::Docker;
use rocket::State;

// Filtering
use std::collections::HashMap;

#[get("/create")]
pub async fn create(user: User, docker: &State<Docker>) -> Status {
    let id = user.id.as_str();

    let mut filters = HashMap::new();
    filters.insert("name", vec![id]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = docker.list_containers(options).await.unwrap();

    if !containers.is_empty() {
        return Status::Conflict;
    }

    let options = Some(CreateContainerOptions {
        name: id,
        platform: None,
    });

    let config = Config {
        image: Some("debian:bookworm"),
        ..Default::default()
    };

    docker.create_container(options, config).await.unwrap();

    docker
        .start_container(id, None::<StartContainerOptions<&str>>)
        .await
        .unwrap();

    Status::Created
}
