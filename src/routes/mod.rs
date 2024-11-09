use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth_routes;
pub mod users_routes;
pub mod app_routes;
pub mod store_routes;

#[derive(OpenApi)]
struct ApiDoc;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/api/docs/swagger-ui/{_:.*}")
            .url("/api/docs/openapi.json", ApiDoc::openapi()),
    );

    cfg.service(
        web::scope("/api")
            .configure(auth_routes::init_auth_routes)
            .configure(users_routes::init_users_routes)
            .configure(store_routes::init_store_routes),
    );

    app_routes::init_app_routes(cfg);
}
