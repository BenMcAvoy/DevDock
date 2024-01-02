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
use bollard::container::StopContainerOptions;

// Filtering
use std::collections::HashMap;

// State
use crate::AppState;

pub async fn container_exists(user: &User, state: &State<AppState>) -> bool {
    let id = user.id.as_str();

    let mut filters = HashMap::new();
    filters.insert("name", vec![id]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = state.docker.list_containers(options).await.unwrap();

    !containers.is_empty()
}

#[get("/create")]
pub async fn create(user: User, state: &State<AppState>) -> Status {
    if container_exists(&user, state).await {
        return Status::Conflict;
    }

    let id = user.id.as_str();

    let options = Some(CreateContainerOptions {
        name: id,
        platform: None,
    });

    // TODO: Use secure information for username
    // and password.
    let env = vec![
        format!("USERNAME={}", id),
        format!("PASSWORD={}", id),
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
    if !container_exists(&user, state).await {
        return Status::NotFound;
    }

    state
        .docker
        .start_container(&user.id, None::<StartContainerOptions<&str>>)
        .await
        .unwrap();

    Status::NoContent
}

#[get("/stop")]
pub async fn stop(user: User, state: &State<AppState>) -> Status {
    if !container_exists(&user, state).await {
        return Status::NotFound;
    }

    let options = Some(StopContainerOptions {
        ..Default::default()
    });

    state.docker.stop_container(&user.id, options).await.unwrap();

    Status::NoContent
}

#[get("/delete")]
pub async fn delete(user: User, state: &State<AppState>) -> Status {
    if !container_exists(&user, state).await {
        return Status::NotFound;
    }

    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    state.docker.remove_container(&user.id, options).await.unwrap();

    Status::NoContent
}
