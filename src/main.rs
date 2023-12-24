#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::*;

use bollard::Docker;

use std::env;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
async fn rocket() -> _ {
    let port = env::var("ROCKET_PORT").unwrap_or("8000".to_string());
    log::info!("Listening on http://localhost:{port}");

    let docker = Docker::connect_with_socket_defaults().unwrap();
    dbg!(docker.version().await.unwrap());

    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static/"))
        .mount("/", routes![index])
}
