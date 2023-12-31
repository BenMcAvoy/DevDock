// Rocket
use rocket::fs::FileServer;
use rocket::{get, routes};

// Rocket extensions
use rocket_dyn_templates::{context, Template};
use rocket_oauth2::OAuth2;

// Docker connection
use bollard::Docker;

mod login;
use login::*;

#[get("/other")]
async fn other(user: User) -> String {
    user.email
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

    dbg!(docker.client_version());

    rocket::build()
        .mount("/", routes![microsoft_login, microsoft_callback, logout])
        .mount("/", routes![index, index_anonymous])
        .mount("/", routes![other])
        .mount("/errors", routes![error_old_token])
        .mount("/static", FileServer::from("./static/"))
        .attach(OAuth2::<MicrosoftUserInfo>::fairing("microsoft"))
        .attach(Template::fairing())
}
