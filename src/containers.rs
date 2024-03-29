// Rocket
use rocket::get;
use rocket::http::Status;
use rocket::State;

// User
use crate::mapping;
use crate::User;

// Docker
use bollard::container::Config;
use bollard::container::CreateContainerOptions;
use bollard::container::ListContainersOptions;
use bollard::container::RemoveContainerOptions;
use bollard::container::StartContainerOptions;
use bollard::container::StopContainerOptions;
use bollard::service::HostConfig;
use bollard::service::PortBinding;

// Internal
use crate::envmap;
use crate::hashmap;
use crate::AppState;

// Database
use sled::IVec;

// Standard
use std::net::TcpListener;

pub async fn container_exists(user: &User, state: &State<AppState>) -> bool {
    let id = user.id.as_str();

    let filters = hashmap!("name" => vec![id]);

    let containers = state.docker.list_containers(Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    }));

    !containers.await.unwrap().is_empty()
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
    let env = envmap!(
        "USERNAME" => id,
        "PASSWORD" => id,
    );

    let env: Vec<&str> = env.iter().map(|s| &**s).collect();

    let random_port = add_stowed_port(&user, state).to_string();

    println!("Port is {random_port}");

    let config = Config {
        host_config: Some(HostConfig {
            port_bindings: Some(mapping!("8080/tcp" => random_port)),
            ..Default::default()
        }),

        image: Some("devdock"),
        env: Some(env),
        ..Default::default()
    };

    state
        .docker
        .create_container(options, config)
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

    state
        .docker
        .stop_container(&user.id, options)
        .await
        .unwrap();

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

    state
        .docker
        .remove_container(&user.id, options)
        .await
        .unwrap();

    drop_stowed_port(&user, state);

    Status::NoContent
}

fn add_stowed_port(user: &User, state: &State<AppState>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    drop(listener);

    state
        .sled
        .insert(IVec::from(&*user.id), IVec::from(&*port.to_string()))
        .unwrap();

    port
}

fn drop_stowed_port(user: &User, state: &State<AppState>) {
    state.sled.remove(IVec::from(&*user.id)).unwrap();
}
