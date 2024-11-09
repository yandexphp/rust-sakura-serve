use actix_files::{Files, NamedFile};
use actix_web::{get, web, HttpResponse, Result};
use actix_web::Either;
use std::path::PathBuf;
use serde_json::json;

#[get("/")]
pub async fn index() -> Result<Either<NamedFile, HttpResponse>> {
    let app_path: PathBuf = ["app", "www", "index.html"].iter().collect();

    if app_path.exists() {
        match NamedFile::open_async(app_path).await {
            Ok(named_file) => Ok(Either::Left(named_file)),
            Err(e) => {
                log::error!("Error opening file: {}", e);
                Ok(Either::Right(HttpResponse::InternalServerError().json(json!({
                    "message": "Error opening file",
                    "errorCode": "FILE_OPEN_ERROR"
                }))))
            }
        }
    } else {
        let embedded_html = include_str!("../app/www/default.html");
        Ok(Either::Right(HttpResponse::Ok().content_type("text/html").body(embedded_html)))
    }
}

pub fn init_app_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index)
        .service(Files::new("/", "./app/www").index_file("index.html"))
        .service(Files::new("/static", "./app/www/static").show_files_listing())
        .default_service(web::route().to(|| async { HttpResponse::NotFound().body("Resource not found.") }));
}
