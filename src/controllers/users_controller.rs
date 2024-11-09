use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::models::user::{CreditCard, FullProfileUpdate, PartialProfileUpdate};
use crate::state::app_state::AppState;
use crate::utils::func::mask_card_number;

#[derive(Serialize, Deserialize)]
pub struct MyData {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize)]
pub struct CreditCardInput {
    pub cardholder_name: String,
    pub card_number: String,
    pub expiration_date: String,
    pub is_primary: bool,
}

#[get("/profile")]
pub async fn profile(session: Session, app_state: web::Data<AppState>) -> impl Responder {
    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        let users = app_state.users_store.users.lock().await;

        if let Some(user) = users.iter().find(|u| u.id == user_id) {
            let masked_credit_cards: Vec<CreditCard> = user.credit_cards
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|cc| CreditCard {
                    id: cc.id.clone(),
                    cardholder_name: cc.cardholder_name.clone(),
                    card_number: mask_card_number(&cc.card_number),
                    expiration_date: cc.expiration_date.clone(),
                    is_primary: cc.is_primary,
                })
                .collect();

            return HttpResponse::Ok().json(json!({
                "id": user.id,
                "username": user.username,
                "login": user.login,
                "password_hash": "******************",
                "email": user.email,
                "phone_number": user.phone_number,
                "date_of_birth": user.date_of_birth,
                "avatar_url": user.avatar_url,
                "registration_date": user.registration_date,
                "last_login_date": user.last_login_date,
                "country": user.country,
                "region": user.region,
                "city": user.city,
                "address": user.address,
                "zip_code": user.zip_code,
                "credit_cards": masked_credit_cards,
            }));
        }
    }

    HttpResponse::Unauthorized().json(json!({
            "message": "Unauthorized",
            "errorCode": "UNAUTHORIZED_ACCESS"
        }))
}

#[post("/profile")]
pub async fn update_profile_full(
    session: Session,
    app_state: web::Data<AppState>,
    payload: web::Json<FullProfileUpdate>,
) -> impl Responder {
    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        if let Some(mut user) = app_state.users_store.find_user_by_id(user_id).await {
            let password_hash = match hash(payload.password.clone(), DEFAULT_COST) {
                Ok(h) => h,
                Err(e) => {
                    log::error!("Error hashing password: {}", e);
                    return HttpResponse::InternalServerError().json(json!({
                        "message": "Error hashing password",
                        "errorCode": "HASHING_ERROR"
                    }));
                }
            };

            user.username = payload.username.clone();
            user.password_hash = password_hash.clone();
            user.email = payload.email.clone();
            user.phone_number = Some(payload.phone_number.clone());
            user.date_of_birth = Some(payload.date_of_birth.clone());
            user.avatar_url = Some(payload.avatar_url.clone().unwrap_or_else(|| "".to_string()));
            user.country = Some(payload.country.clone());
            user.region = Some(payload.region.clone());
            user.city = Some(payload.city.clone());
            user.address = Some(payload.address.clone());
            user.zip_code = Some(payload.zip_code.clone());

            if let Some(credit_cards) = &payload.credit_cards {
                user.credit_cards = Some(credit_cards.clone());
            }

            if app_state.users_store.update_user(user).await {
                return HttpResponse::Ok().json(json!({
                    "message": "Profile updated successfully",
                    "errorCode": "SUCCESS"
                }));
            } else {
                log::error!("Failed to update user with ID: {}", user_id);
                return HttpResponse::InternalServerError().json(json!({
                    "message": "Failed to update profile",
                    "errorCode": "UPDATE_PROFILE_FAILED"
                }));
            }
        } else {
            log::error!("User session not found with ID: {}", user_id);
            return HttpResponse::Unauthorized().json(json!({
                "message": "Unauthorized",
                "errorCode": "UNAUTHORIZED_ACCESS"
            }));
        }
    }

    HttpResponse::Unauthorized().json(json!({
        "message": "Unauthorized",
        "errorCode": "UNAUTHORIZED_ACCESS"
    }))
}

#[put("/profile")]
pub async fn update_profile_partial(
    session: Session,
    app_state: web::Data<AppState>,
    payload: web::Json<PartialProfileUpdate>,
) -> impl Responder {
    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        if let Some(mut user) = app_state.users_store.find_user_by_id(user_id).await {
            if let Some(phone_number) = &payload.phone_number {
                user.phone_number = Some(phone_number.clone());
            }
            if let Some(username) = &payload.username {
                user.username = username.clone();
            }
            if let Some(password) = &payload.password {
                let password_hash = match hash(password, DEFAULT_COST) {
                    Ok(h) => h,
                    Err(e) => {
                        log::error!("Error hashing password: {}", e);
                        return HttpResponse::InternalServerError().json(json!({
                            "message": "Error hashing password",
                            "errorCode": "HASHING_ERROR"
                        }));
                    }
                };
                user.password_hash = password_hash;
            }
            if let Some(email) = &payload.email {
                user.email = email.clone();
            }
            if let Some(phone_number) = &payload.phone_number {
                user.phone_number = Some(phone_number.clone());
            }
            if let Some(date_of_birth) = &payload.date_of_birth {
                user.date_of_birth = Some(date_of_birth.clone());
            }
            if let Some(avatar_url) = &payload.avatar_url {
                user.avatar_url = Some(avatar_url.clone());
            }
            if let Some(country) = &payload.country {
                user.country = Some(country.clone());
            }
            if let Some(region) = &payload.region {
                user.region = Some(region.clone());
            }
            if let Some(city) = &payload.city {
                user.city = Some(city.clone());
            }
            if let Some(address) = &payload.address {
                user.address = Some(address.clone());
            }
            if let Some(zip_code) = &payload.zip_code {
                user.zip_code = Some(zip_code.clone());
            }
            if let Some(credit_cards) = &payload.credit_cards {
                user.credit_cards = Some(credit_cards.clone());
            }

            if app_state.users_store.update_user(user).await {
                return HttpResponse::Ok().json(json!({
                    "message": "Profile updated successfully",
                    "errorCode": "SUCCESS"
                }));
            } else {
                log::error!("Failed to update user with ID: {}", user_id);
                return HttpResponse::InternalServerError().json(json!({
                    "message": "Failed to update profile",
                    "errorCode": "PROFILE_UPDATE_FAILED"
                }));
            }
        } else {
            log::error!("User not found with ID: {}", user_id);
            return HttpResponse::Unauthorized().json(json!({
                "message": "Unauthorized",
                "errorCode": "UNAUTHORIZED_ACCESS"
            }));
        }
    }

    HttpResponse::Unauthorized().json(json!({
        "message": "Unauthorized",
        "errorCode": "UNAUTHORIZED_ACCESS"
    }))
}

#[get("/{id}")]
pub async fn get_user(path: web::Path<Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let users = app_state.users_store.users.lock().await;

    if let Some(user) = users.iter().find(|u| u.id == user_id) {
        return HttpResponse::Ok().json(json!({
            "id": user.id,
            "avatar_url": user.avatar_url,
            "username": user.username,
        }));
    }

    HttpResponse::NotFound().json(json!({
        "message": "User not found",
        "errorCode": "USER_NOT_FOUND"
    }))
}

#[post("/creditcard/add")]
pub async fn add_credit_card(
    session: Session,
    data: web::Json<CreditCardInput>,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        let mut users = app_state.users_store.users.lock().await;

        if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
            if let Some(ref credit_cards) = user.credit_cards {
                if credit_cards.iter().any(|c| c.card_number == data.card_number) {
                    return HttpResponse::BadRequest().json(json!({
                        "message": "This credit card has already been added",
                        "errorCode": "CARD_ALREADY_ADDED"
                    }));
                }
            }

            if data.is_primary {
                if let Some(ref mut credit_cards) = user.credit_cards {
                    for card in credit_cards.iter_mut() {
                        card.is_primary = false;
                    }
                }
            }

            let new_card = CreditCard {
                id: Uuid::new_v4(),
                cardholder_name: data.cardholder_name.clone(),
                card_number: data.card_number.clone(),
                expiration_date: data.expiration_date.clone(),
                is_primary: data.is_primary,
            };

            if let Some(ref mut credit_cards) = user.credit_cards {
                credit_cards.push(new_card);
            } else {
                user.credit_cards = Some(vec![new_card]);
            }

            drop(users);

            if let Err(e) = app_state.users_store.save().await {
                log::error!("Failed to save data - error: {}", e.to_string());
                return HttpResponse::InternalServerError().json(json!({
                    "message": "Failed to save data",
                    "errorCode": "SAVE_ERROR"
                }));
            }

            return HttpResponse::Ok().json(json!({
                "message": "Credit card added successfully",
                "errorCode": "SUCCESS"
            }));
        }
    }

    HttpResponse::Unauthorized().json(json!({
        "message": "Unauthorized",
        "errorCode": "UNAUTHORIZED_ACCESS"
    }))
}

#[delete("/creditcard/{id}")]
pub async fn delete_credit_card(
    session: Session,
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let id = path.into_inner();

    if let Some(user_id) = session.get::<Uuid>("user_id").unwrap_or(None) {
        let mut users = app_state.users_store.users.lock().await;

        if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
            if let Some(ref mut credit_cards) = user.credit_cards {
                let initial_len = credit_cards.len();

                credit_cards.retain(|card| card.id != id);

                if credit_cards.len() < initial_len {
                    drop(users);
                    app_state.users_store.save().await.unwrap();

                    return HttpResponse::Ok().json(json!({
                        "message": "Credit card deleted successfully",
                        "errorCode": "CARD_DELETED_SUCCESS"
                    }));
                } else {
                    return HttpResponse::NotFound().json(json!({
                        "message": "Credit card not found",
                        "errorCode": "CARD_NOT_FOUND"
                    }));
                }
            } else {
                return HttpResponse::NotFound().json(json!({
                    "message": "No credit cards found for this user",
                    "errorCode": "NO_CARDS_FOUND"
                }));
            }
        }
    }

    HttpResponse::Unauthorized().json(json!({
        "message": "Unauthorized",
        "errorCode": "UNAUTHORIZED_ACCESS"
    }))
}