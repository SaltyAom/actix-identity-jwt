mod error;
mod jwt;
mod route;
mod time;

use std::env;
use std::io;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use route::common::{get_user, get_user_without_template, index, login, logout};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(env::var("cookie_secret").unwrap().as_bytes())
                    .name("auth")
                    .secure(false),
            ))
            .service(index)
            .service(login)
            .service(get_user)
            .service(get_user_without_template)
            .service(logout)
            .default_service(web::resource("").to(error::http_error::not_found))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
