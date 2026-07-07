use model::*;
use rocket::{get, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

mod routes;

#[openapi] // Generates OpenAPI documentation for this route
#[get("/hello")]
fn hello() -> String {
    log::info!("hello");
    "Hello, OpenAPI!".to_string()
}

#[openapi]
#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    log::info!("get_users");
    Json(vec![User {
        username: "johndoe".to_string(),
        email: "john@doe.com".to_string(),
    }])
}

#[openapi]
#[get("/user/<user_id>")]
fn get_user(user_id: &str) -> Json<User> {
    log::info!("get_user called with {}", user_id);
    Json(User {
        username: "johndoe".to_string(),
        email: "john@doe.com".to_string(),
    })
}

fn rocker_build() -> rocket::Rocket<rocket::Build> {
    rocket::build()
    .mount(
        "/",
        openapi_get_routes![hello, 
            routes::auth::sign_up, 
            routes::auth::login, 
            routes::auth::logout, 
            get_users, 
            get_user],
    ) // Mount API routes
    .mount(
        "/swagger",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/openapi.json".to_string(), // OpenAPI JSON file
            ..Default::default()
        }),
    )
}

#[rocket::main]
async fn main() {
    let launch_result = rocker_build()
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

#[cfg(test)]
mod tests;
