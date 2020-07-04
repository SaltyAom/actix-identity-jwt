mod route;
mod jwt;
mod error;

use std::io;

use actix_web::{ HttpServer, App, web };
use actix_identity::{ IdentityService, CookieIdentityPolicy };
use dotenv::dotenv;

use route::common::{ index, login ,get_user, get_user_without_template, logout };

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&[69; 32])
                        .name("auth")
                        .secure(false)
                    )
            )
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