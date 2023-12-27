// Error handling
use anyhow::{Context, Error};

// Web requests
use reqwest::header::AUTHORIZATION;

// Rocket
use rocket::fs::FileServer;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::request;
use rocket::response::{Debug, Redirect};
use rocket::{get, routes};

// Rocket extensions
use rocket_dyn_templates::{context, Template};
use rocket_oauth2::{OAuth2, TokenResponse};

struct User {
    pub email: String,
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<User, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        if let Some(cookie) = cookies.get_private("email") {
            return request::Outcome::Success(User {
                email: cookie.value().to_string(),
            });
        }

        request::Outcome::Forward(Status::Unauthorized)
    }
}

#[derive(serde::Deserialize)]
struct MicrosoftUserInfo {
    #[serde(default, rename = "mail")]
    email: String,
}

#[get("/login/microsoft")]
fn microsoft_login(oauth2: OAuth2<MicrosoftUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user.read"]).unwrap()
}

#[get("/auth/microsoft")]
async fn microsoft_callback(
    token: TokenResponse<MicrosoftUserInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    let user_info: MicrosoftUserInfo = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://graph.microsoft.com/v1.0/me")
        .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
        .send()
        .await
        .context("failed to complete request")?
        .json()
        .await
        .context("failed to deserialize response")?;

    // Set a private cookie with the user's name.
    cookies.add_private(
        Cookie::build(("email", user_info.email))
            .same_site(SameSite::Lax)
            .build(),
    );

    Ok(Redirect::to("/"))
}

#[get("/other")]
async fn other(user: User) -> String {
    user.email
}

#[get("/")]
fn index(user: User) -> Template {
    Template::render(
        "index",
        context! {
            email: user.email
        },
    )
}

#[get("/", rank = 2)]
fn index_anonymous() -> Template {
    Template::render("anonymous_index", context! {})
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("email"));
    Redirect::to("/")
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                index_anonymous,
                logout,
                microsoft_callback,
                microsoft_login,
                other,
            ],
        )
        .mount("/static", FileServer::from("./static/"))
        .attach(Template::fairing())
        .attach(OAuth2::<MicrosoftUserInfo>::fairing("microsoft"))
}
