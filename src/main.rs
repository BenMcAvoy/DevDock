#[macro_use]
extern crate rocket;

use rocket_dyn_templates::*;

use std::env;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    let port = env::var("ROCKET_PORT").unwrap_or("8000".to_string());
    log::info!("Listening on http://localhost:{port}");

    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
