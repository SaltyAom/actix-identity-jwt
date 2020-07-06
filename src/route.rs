pub mod common {
    use std::option::Option;

    use actix_identity::Identity;
    use actix_web::{get, web, HttpResponse};

    use askama::Template;

    use crate::jwt::{decode, encode};
    use crate::time::get_expire_time;

    #[derive(Template)]
    #[template(path = "user.html")]
    struct UserPage<'a> {
        name: &'a str,
    }

    const ANONYMOUS_PAGE: UserPage<'static> = UserPage { name: "Anonymous" };

    #[get("/")]
    #[inline(always)]
    pub async fn index() -> String {
        "Hello World".to_owned()
    }

    #[get("/login/{username}")]
    #[inline(always)]
    pub async fn login(user: Identity, username: web::Path<String>) -> String {
        let name = encode(&username);

        if name.is_none() {
            return "Login failed".to_owned();
        };

        user.remember(name.unwrap());

        format!("Logged in as {}", username)
    }

    #[get("/user")]
    #[inline(always)]
    pub async fn get_user(user: Identity) -> HttpResponse {
        if user.identity().is_none() {
            return HttpResponse::Ok()
                .content_type("text/html")
                .body(ANONYMOUS_PAGE.render().unwrap());
        };

        let auth = decode(&user.identity().unwrap());

        let page = if let Some(token) = auth {
            if token.exp < get_expire_time() {
                // Refresh Token
                user.remember(encode(&token.name).unwrap());

                UserPage { name: &token.name }.render().unwrap()
            } else {
                user.forget();

                ANONYMOUS_PAGE.render().unwrap()
            }
        } else {
            ANONYMOUS_PAGE.render().unwrap()
        };

        HttpResponse::Ok().content_type("text/html").body(page)
    }

    #[get("/user/without-template")]
    #[inline(always)]
    pub async fn get_user_without_template(user: Identity) -> String {
        if user.identity().is_none() {
            return format!("Logged in as anonymous");
        }

        let auth = decode(&user.identity().unwrap()).unwrap();

        format!("Logged in as {}", auth.name)
    }

    #[get("/logout")]
    #[inline(always)]
    pub async fn logout(user: Identity) -> String {
        user.forget();

        "Logged out".to_owned()
    }
}
