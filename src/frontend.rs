// Rocket
use rocket::get;
use rocket::State;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

// Db
use sled::IVec;

// Internal
use crate::login::User;
use crate::AppState;

#[get("/status")]
pub async fn status(user: User, state: &State<AppState>) -> Template {
    let port = state
        .sled
        .get(IVec::from(&*user.id))
        .unwrap()
        .map(|p| String::from_utf8(p.to_vec()))
        .unwrap_or(Ok("No container".to_string()))
        .unwrap();

    Template::render("status", context! { port })
}
