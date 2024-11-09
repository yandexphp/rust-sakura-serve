use actix_session::Session;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::state::app_state::AppState;

#[derive(Deserialize)]
pub struct OrderRequest {
    pub product_ids: Vec<Uuid>,
    pub promo_code: Option<String>,
}

#[get("/")]
pub async fn get_orders(
    session: Session,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        let orders = app_state.orders_store.get_orders(user_id).await;
        HttpResponse::Ok().json(json!(orders))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "Unauthorized",
            "errorCode": "UNAUTHORIZED_ACCESS"
        }))
    }
}

#[post("/create")]
pub async fn create_order(
    session: Session,
    data: web::Json<OrderRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        match app_state.orders_store.add_order(
            user_id,
            data.product_ids.clone(),
            data.promo_code.clone(),
            &app_state
        ).await {
            Ok(_) => HttpResponse::Ok().json(json!({ "message": "Order created successfully" })),
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

#[delete("/{order_id}")]
pub async fn delete_order(
    session: Session,
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let order_id = path.into_inner();

    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        match app_state.orders_store.remove_order(user_id, order_id).await {
            Ok(_) => HttpResponse::Ok().json(json!({ "message": "Order deleted successfully" })),
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
