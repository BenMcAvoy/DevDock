#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::*;

#[get("/")]
async fn login() -> Template {
    let auth_url = "Unknown";

    Template::render(
        "login",
        context! {
            url: auth_url,
        },
    )
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static/"))
        .mount("/", routes![login])
}
