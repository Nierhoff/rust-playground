use model::*;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::openapi;


#[openapi]
#[post("/signup", data = "<user>")]
pub fn sign_up(user: Json<UserSignUp>) -> Json<User> {
    log::info!("signup called with {}", user.username);
    Json(User {
        username: user.username.to_string(),
        email: user.email.to_string(),
    })
}

#[openapi]
#[post("/login", data = "<user>")]
pub fn login(user: Json<Login>) -> Json<User> {
    log::info!("loging called with {}", user.username);
    Json(User {
        username: user.username.to_string(),
        email: "john@doe.com".to_string(),
    })
}

#[openapi]
#[get("/logout")]
pub fn logout() -> String {
    "".to_string()
}
