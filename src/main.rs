#[macro_use]
extern crate rocket;

use graph_rs_sdk::Graph;
use rocket::fs::FileServer;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::response::content::RawHtml;
use rocket::State;
use rocket_dyn_templates::*;

use bollard::Docker;
use cookie::Cookie;
use dotenvy::dotenv;

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};

use oauth2::reqwest::async_http_client;

use std::env;
use std::sync::Arc;

// use static_init::{dynamic, Lazy};

// #[dynamic(lazy)]
// static BSFENGINE: Engine = Engine::new();

#[get("/")]
fn index() {
    Redirect::to("/app/login");
}

#[get("/login")]
async fn login(client: &State<Arc<BasicClient>>, jar: &CookieJar<'_>) -> Template {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let cookie = Cookie::new("verifier", pkce_verifier.secret().clone());

    jar.add(cookie);

    // let pkce_verifier_str = BSFENGINE::encode(pkce_verifier);

    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(String::from("User.Read")))
        .set_pkce_challenge(pkce_challenge)
        .url();

    Template::render(
        "login",
        context! {
            url: auth_url,
        },
    )
}

#[get("/callback?<code>")]
async fn callback(
    code: String,
    client: &State<Arc<BasicClient>>,
    jar: &CookieJar<'_>,
) -> RawHtml<&'static str> {
    let pkce_verifier: PkceCodeVerifier =
        PkceCodeVerifier::new(jar.get("verifier").unwrap().value().to_owned());

    let code = AuthorizationCode::new(code);
    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await.unwrap();

    dbg!(token);

    RawHtml("<h1>Authed</h1>")
}

#[launch]
async fn rocket() -> _ {
    if let Err(e) = dotenv() {
        log::error!("Failed to load environment variables: {e}");
    }

    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let client_id = env::var("CLIENT_ID").unwrap();
    let tenant_id = env::var("TENANT_ID").unwrap();
    let port = env::var("ROCKET_PORT").unwrap_or("8000".to_string());

    println!("URL           : http://localhost:{port}");
    println!("Client secret : {tenant_id}");
    println!("Client ID     : {client_id}");
    println!("Tenant ID     : {tenant_id}");

    // let docker = Docker::connect_with_socket_defaults().unwrap();
    // dbg!(docker.version().await.unwrap());

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
            tenant_id
        ))
        .unwrap(),
        Some(
            TokenUrl::new(format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                tenant_id
            ))
            .unwrap(),
        ),
    )
    .set_redirect_uri(RedirectUrl::new(format!("http://localhost:{port}/app/callback")).unwrap());

    let client = Arc::new(client);

    rocket::build()
        .manage(client)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static/"))
        .mount("/app", routes![login, callback])
        .mount("/", routes![index])
}
