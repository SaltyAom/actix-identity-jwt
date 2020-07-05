pub mod common {
    use std::option::Option;

    use actix_identity::Identity;
    use actix_web::{get, web, HttpResponse};

    use crate::jwt::{decode, encode};

    use askama::Template;

    #[derive(Template)]
    #[template(path = "user.html")]
    struct UserPage<'a> {
        name: &'a str,
    }

    #[get("/")]
    pub async fn index() -> String {
        "Hello World".to_owned()
    }

    #[get("/login/{username}")]
    pub async fn login(user: Identity, username: web::Path<String>) -> String {
        let name = encode(username.to_string());

        if name.is_none() {
            return "Login failed".to_owned()
        };

        user.remember(name.unwrap());

        format!("Logged in as {}", username)
    }

    #[get("/user")]
    pub async fn get_user(user: Identity) -> HttpResponse {
        if user.identity().is_none() {
            let page = UserPage { name: "Anonymous" }.render().unwrap();

            return HttpResponse::Ok().content_type("text/html").body(page);
        };

        let username = decode(user.identity().unwrap());

        let page = if let Some(name) = username {
            UserPage {
                name: &name,
            }
            .render()
            .unwrap()
        } else {
            UserPage {
                name: "Anonymous",
            }
            .render()
            .unwrap()
        };

        HttpResponse::Ok().content_type("text/html").body(page)
    }

    #[get("/user/without-template")]
    pub async fn get_user_without_template(user: Identity) -> String {
        if user.identity().is_none() {
            return format!("Logged in as anonymous");
        }

        let name = decode(user.identity().unwrap()).unwrap();

        format!("Logged in as {}", name)
    }

    #[get("/logout")]
    pub async fn logout(user: Identity) -> String {
        user.forget();

        "Logged out".to_owned()
    }
}
