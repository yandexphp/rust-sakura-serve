use actix_session::Session;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::state::app_state::AppState;

#[get("/")]
pub async fn get_cart(
    session: Session,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        let cart = app_state.carts_store.get_cart(user_id).await;
        HttpResponse::Ok().json(json!(cart))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "Unauthorized",
            "errorCode": "UNAUTHORIZED_ACCESS"
        }))
    }
}

#[post("/add/{product_id}")]
pub async fn add_product_to_cart(
    session: Session,
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let product_id = path.into_inner();

    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        match app_state.carts_store.add_product_to_cart(user_id, product_id).await {
            Ok(_) => HttpResponse::Ok().json(json!({ "message": "Product added to cart successfully" })),
            Err(e) => HttpResponse::BadRequest().json(json!({
                "message": e.to_string(),
                "errorCode": "BAD_REQUEST_ERROR"
            })),
        }
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "Unauthorized",
            "errorCode": "UNAUTHORIZED_ACCESS"
        }))
    }
}

#[delete("/{product_id}")]
pub async fn remove_product_from_cart(
    session: Session,
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let product_id = path.into_inner();

    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        match app_state.carts_store.remove_product_from_cart(user_id, product_id).await {
            Ok(_) => HttpResponse::Ok().json(json!({ "message": "Product removed from cart successfully" })),
            Err(e) => HttpResponse::BadRequest().json(json!({
                "message": e.to_string(),
                "errorCode": "BAD_REQUEST_ERROR"
            })),
        }
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "Unauthorized",
            "errorCode": "UNAUTHORIZED_ACCESS"
        }))
    }
}
