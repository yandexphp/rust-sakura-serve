use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use chrono::Utc;

use crate::state::app_state::AppState;

#[get("/validate/{promo_code}")]
pub async fn validate_promo_code(
    path: web::Path<String>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let promo_code = path.into_inner();

    match app_state.promocodes_store.load_promo_codes().await {
        Ok(promo_codes) => {
            if let Some(promo) = promo_codes.iter().find(|p| p.code == promo_code) {
                let current_time = Utc::now();
                let available_at = promo.available_at.parse::<chrono::DateTime<Utc>>().unwrap();
                let expired_at = promo.expired_at.parse::<chrono::DateTime<Utc>>().unwrap();

                if current_time < available_at {
                    return HttpResponse::BadRequest().json(json!({
                        "message": "Promo code is not yet available",
                        "errorCode": "PROMO_CODE_NOT_YET_AVAILABLE"
                    }));
                } else if current_time > expired_at {
                    return HttpResponse::BadRequest().json(json!({
                        "message": "Promo code has expired",
                        "errorCode": "PROMO_CODE_EXPIRED"
                    }));
                } else {
                    HttpResponse::Ok().json(json!({
                        "message": "Promo code is valid",
                        "errorCode": "SUCCESS"
                    }))
                }
            } else {
                HttpResponse::NotFound().json(json!({
                    "message": "Promo code not found",
                    "errorCode": "PROMO_CODE_NOT_FOUND"
                }))
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "message": "Failed to load promo codes",
            "errorCode": "INTERNAL_SERVER_ERROR"
        }))
    }
}
