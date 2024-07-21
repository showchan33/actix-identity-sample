use actix_identity::{Identity, IdentityMiddleware};
use actix_session::config::CookieContentSecurity::Private;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::middleware::Logger;
use actix_web::{
  cookie::Key, get, post, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

mod middleware;
use middleware::AuthMiddleware;

#[derive(Deserialize)]
struct LoginForm {
  username: String,
  password: String,
}

#[get("/login")]
async fn login_form() -> HttpResponse {
  HttpResponse::Ok().body(
    r#"
    <!doctype html>
    <html>
    <head>
    <title>login</title>
    </head>
    <meta charset="utf-8" />
    <body>
      <form action="/login" method="post">
        <input type="text" name="username" placeholder="Username" />
        <input type="password" name="password" placeholder="Password" />
        <button type="submit">Login</button>
      </form>
    </body>
    </html>
  "#,
  )
}

#[post("/login")]
async fn login(request: HttpRequest, form: web::Form<LoginForm>) -> HttpResponse {
  if form.username == "user" && form.password == "password" {
    Identity::login(&request.extensions(), form.username.clone().into()).unwrap();
    HttpResponse::Found()
      .append_header(("location", "/welcome"))
      .finish()
  } else {
    HttpResponse::Unauthorized().body("Unauthorized")
  }
}

#[get("/logout")]
async fn logout(id: Identity) -> impl Responder {
  dbg!("logout");
  id.logout();
  HttpResponse::Found()
    .append_header(("location", "/"))
    .finish()
}

#[get("/")]
async fn index() -> HttpResponse {
  HttpResponse::Ok().body("Welcome to the public page!")
}

#[get("/welcome")]
async fn welcome(id: Option<Identity>) -> HttpResponse {
  if let Some(username) = id {
    HttpResponse::Ok().body(format!("Welcome, {}!", username.id().unwrap()))
  } else {
    HttpResponse::Ok().body("dummy")
  }
}

#[get("/secret")]
async fn secret() -> HttpResponse {
  HttpResponse::Ok().body(format!(
    "This page is only accessible to authenticated users."
  ))
}

fn session_middleware(secret_key: &str) -> SessionMiddleware<CookieSessionStore> {
  SessionMiddleware::builder(
    CookieSessionStore::default(),
    Key::from(secret_key.as_bytes()),
  )
  .cookie_name(String::from("mycookie"))
  .cookie_content_security(Private)
  .cookie_secure(false)
  .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "debug");
  env_logger::init();

  let secret_key = "secret_key".repeat(7);

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .wrap(AuthMiddleware)
      .wrap(IdentityMiddleware::default())
      .wrap(session_middleware(&secret_key))
      .service(index)
      .service(login_form)
      .service(login)
      .service(logout)
      .service(welcome)
      .service(secret)
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
