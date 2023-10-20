use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use log::*;
use log4rs;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use logging;

async fn hello(_req: HttpRequest) -> Result<impl Responder, Error> {
    info!("{}", _req.uri().to_string());
    Ok(HttpResponse::Ok().body("Hello world!"))
}

async fn echo(_req: HttpRequest) -> Result<impl Responder, Error> {
    info!("{}", _req.uri().to_string());
    Ok(HttpResponse::Ok().body("echo"))
}

async fn manual_hello(_req: HttpRequest) -> Result<impl Responder, Error> {
    info!("{}", _req.uri().to_string());
    Ok(HttpResponse::Ok().body("Hey there!"))
}

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::logsetup();
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
            .route("/hey", web::get().to(manual_hello))
            .route("/echo", web::post().to(echo))
            .route("/hello", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::http::Method;
    use actix_web::{http::header::ContentType, test};

    use super::*;

    #[actix_web::test]
    async fn test_hello_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/hello")
            .method(Method::GET)
            .to_http_request();
        let resp = hello(req).await;
        assert_eq!(resp.is_ok(), true);
    }

    #[actix_web::test]
    async fn test_echo_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .method(Method::POST)
            .uri("/echo")
            .to_http_request();
        let resp = echo(req).await;
        assert_eq!(resp.is_ok(), true);
    }
}
