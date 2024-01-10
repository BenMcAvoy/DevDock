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
    let ivec = state.sled.get(IVec::from(&*user.id)).unwrap().unwrap();
    let port: u16 = u16::from_be_bytes([ivec[0], ivec[1]]);

    Template::render("status", context! { port })
}
