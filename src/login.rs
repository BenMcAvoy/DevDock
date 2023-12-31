use std::str::FromStr;

// Error handling
use anyhow::{Context, Error};

// Web requests
use reqwest::header::AUTHORIZATION;

// Rocket
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::{Debug, Redirect};
use rocket::{request, warn};

// Rocket extensions
use rocket_dyn_templates::{context, Template};
use rocket_oauth2::{OAuth2, TokenResponse};

// Time management
use chrono::{DateTime, Utc};

pub struct User {
    pub created: DateTime<Utc>,
    pub email: String,
    pub id: String,
}

fn remove_cookies(cookies: &CookieJar<'_>) {
    cookies.remove(Cookie::from("created"));
    cookies.remove(Cookie::from("email"));
    cookies.remove(Cookie::from("id"));
}

// Runs during a request to an endpoint
#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<User, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");

        let cookie_values = (
            cookies.get_private("email"),
            cookies.get_private("id"),
            cookies.get_private("created"),
        );

        if let (Some(email), Some(id), Some(created)) = cookie_values {
            let created: DateTime<Utc> = DateTime::from_str(created.value()).unwrap();
            let since = Utc::now().signed_duration_since(created);

            if since.num_days() >= 1 {
                // TODO: Redirect to `/errors/old_token`
                remove_cookies(cookies);
            }

            return request::Outcome::Success(User {
                email: email.value().to_string(),
                id: id.value().to_string(),
                created,
            });
        }

        // TODO: Show this error to the user and ask them
        // if they have cookies enabled.
        warn!("Cookies don't exist");
        request::Outcome::Forward(Status::Unauthorized)
    }
}

#[derive(serde::Deserialize)]
pub struct MicrosoftUserInfo {
    #[serde(default, rename = "mail")]
    email: String,

    #[serde(default)]
    id: String,
}

#[get("/login/microsoft")]
pub fn microsoft_login(oauth2: OAuth2<MicrosoftUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user.read"]).unwrap()
}

#[get("/auth/microsoft")]
pub async fn microsoft_callback(
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

    cookies.add_private(
        Cookie::build(("email", user_info.email))
            .same_site(SameSite::Lax)
            .build(),
    );

    cookies.add_private(
        Cookie::build(("id", user_info.id))
            .same_site(SameSite::Lax)
            .build(),
    );

    let created_at = Utc::now();

    cookies.add_private(
        Cookie::build(("created", created_at.to_string()))
            .same_site(SameSite::Lax)
            .build(),
    );

    Ok(Redirect::to("/"))
}

#[get("/old_token", rank = 2)]
pub fn error_old_token(user: User, cookies: &CookieJar<'_>) -> Template {
    let days_since = Utc::now().signed_duration_since(user.created).num_days();

    remove_cookies(cookies);

    Template::render(
        "old_token",
        context! {
            days_since
        },
    )
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    remove_cookies(cookies);

    Redirect::to("/")
}
