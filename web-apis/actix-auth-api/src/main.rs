use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use log::*;
use log4rs;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use logging;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::logsetup();
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new().wrap(TracingLogger::default()).service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
