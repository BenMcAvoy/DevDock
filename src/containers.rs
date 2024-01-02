// Rocket
use rocket::get;
use rocket::http::Status;
use rocket::State;

// User
use crate::User;

// Docker
use bollard::container::Config;
use bollard::container::CreateContainerOptions;
use bollard::container::ListContainersOptions;
use bollard::container::RemoveContainerOptions;
use bollard::container::StartContainerOptions;

// Filtering
use std::collections::HashMap;

// State
use crate::AppState;

#[get("/create")]
pub async fn create(user: User, state: &State<AppState>) -> Status {
    let id = user.id.as_str();

    let mut filters = HashMap::new();
    filters.insert("name", vec![id]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = state.docker.list_containers(options).await.unwrap();

    if !containers.is_empty() {
        return Status::Conflict;
    }

    let options = Some(CreateContainerOptions {
        name: id,
        platform: None,
    });

    // TODO: Use secure information for username
    // and password.
    let env = vec![
        format!("USERNAME={}", user.id),
        format!("PASSWORD={}", user.id),
    ];

    let env: Vec<&str> = env.iter().map(|s| &**s).collect();

    let config = Config {
        image: Some("devdock"),
        env: Some(env),
        ..Default::default()
    };

    state
        .docker
        .create_container(options, config)
        .await
        .unwrap();

    state
        .docker
        .start_container(id, None::<StartContainerOptions<&str>>)
        .await
        .unwrap();

    Status::NoContent
}

#[get("/start")]
pub async fn start(user: User, state: &State<AppState>) -> Status {
    let id = user.id.as_str();

    let mut filters = HashMap::new();
    filters.insert("name", vec![id]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = state.docker.list_containers(options).await.unwrap();

    if containers.is_empty() {
        return Status::NotFound;
    }

    state
        .docker
        .start_container(id, None::<StartContainerOptions<&str>>)
        .await
        .unwrap();

    Status::NoContent
}

#[get("/delete")]
pub async fn delete(user: User, state: &State<AppState>) -> Status {
    let id = user.id.as_str();

    let mut filters = HashMap::new();
    filters.insert("name", vec![id]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = state.docker.list_containers(options).await.unwrap();

    if containers.is_empty() {
        return Status::NotFound;
    }

    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    state.docker.remove_container(id, options).await.unwrap();

    Status::NoContent
}
