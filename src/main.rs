// Rocket
use rocket::fs::FileServer;
use rocket::{get, routes};

// Rocket extensions
use rocket_dyn_templates::{context, Template};
use rocket_oauth2::OAuth2;

// Docker connection
use bollard::Docker;

// Database
use sled::Db;

mod login;
use login::*;

mod containers;
use containers::*;

pub struct AppState {
    pub docker: Docker,
    pub sled: Db,
}

#[get("/")]
fn index(user: User) -> Template {
    Template::render(
        "index",
        context! {
            email: user.email,
            id: user.id,
        },
    )
}

#[get("/", rank = 2)]
fn index_anonymous() -> Template {
    Template::render("anonymous_index", context! {})
}

#[rocket::launch]
fn rocket() -> _ {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let sled = sled::open("./ports").unwrap();

    let state = AppState { docker, sled };

    rocket::build()
        .manage(state)
        .mount("/", routes![microsoft_login, microsoft_callback, logout])
        .mount("/", routes![index, index_anonymous])
        .mount("/", routes![create, start])
        .mount("/errors", routes![error_old_token])
        .mount("/static", FileServer::from("./static/"))
        .attach(OAuth2::<MicrosoftUserInfo>::fairing("microsoft"))
        .attach(Template::fairing())
}
