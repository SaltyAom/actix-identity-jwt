pub mod common {
    use actix_web::{ get, web, HttpResponse };
    use actix_identity::{ Identity };

    use crate::jwt::{ encode, decode };

    use askama::Template;

    #[derive(Template)]
    #[template(path = "user.html")]
    struct UserPage<'a> {
        name: &'a str
    }

    #[get("/")]
    pub async fn index() -> String {
        "Hello World".to_owned()
    }

    #[get("/login/{username}")]
    pub async fn login(user: Identity, username: web::Path<String> ) -> String {
        user.remember(encode(username.to_string()));

        format!("Logged in as {}", username)
    }

    #[get("/user")]
    pub async fn get_user(user: Identity) -> HttpResponse {
        if user.identity().is_none() {
            let page = UserPage {
                    name: "Anonymous"
                }.render().unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(
                    page
                )
        }

        let page = UserPage {
            name: &decode(user.identity().unwrap())
        }.render().unwrap();

        HttpResponse::Ok()
            .content_type("text/html")
            .body(
                page
            )
    }

    #[get("/user/without-template")]
    pub async fn get_user_without_template(user: Identity) -> String {
        if user.identity().is_none() {
            return format!("Logged in as anonymous")
        }

        format!("Logged in as {}", decode(user.identity().unwrap()))
    }

    #[get("/logout")]
    pub async fn logout(user: Identity) -> String {
        user.forget();

        "Logged out".to_owned()
    }
}