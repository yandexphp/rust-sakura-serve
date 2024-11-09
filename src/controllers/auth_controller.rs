use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::models::user::User;
use crate::state::app_state::AppState;

#[derive(Deserialize)]
pub struct SignUpData {
    pub username: String,
    pub login: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
    pub date_of_birth: String,
    pub avatar_url: Option<String>,
    pub country: String,
    pub region: String,
    pub city: String,
    pub address: String,
    pub zip_code: String,
}

#[derive(Deserialize)]
pub struct SignInData {
    pub login: String,
    pub password: String,
}

#[post("/signUp")]
pub async fn sign_up(
    data: web::Json<SignUpData>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let login = data.login.trim();
    let username = data.username.trim();
    let password = data.password.trim();
    
    log::info!("Starting registration for user: {}", username);
    
    if username.is_empty() || login.is_empty() || password.is_empty() {
        log::error!("Missing required fields");
        return HttpResponse::BadRequest().json(json!({
            "message": "All fields except avatar_url are required"
        }));
    }

    if app_state.users_store.find_user_by_username(&username).await.is_some()
        || app_state.users_store.find_user_by_login_or_email(&login).await.is_some()
    {
        log::error!("User already exists: username = {}, login = {}", username, login);
        return HttpResponse::BadRequest().json(json!({
            "message": "User already exists"
        }));
    }

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

    log::info!("Creating new user: {}", username);

    let new_user = User {
        id: Uuid::new_v4(),
        username: username.to_string(),
        login: login.to_string(),
        email: data.email.clone(),
        password_hash,
        phone_number: Some(data.phone_number.clone()),
        date_of_birth: Some(data.date_of_birth.clone()),
        avatar_url: data.avatar_url.clone(),
        registration_date: Utc::now(),
        last_login_date: None,
        country: Some(data.country.clone()),
        region: Some(data.region.clone()),
        city: Some(data.city.clone()),
        address: Some(data.address.clone()),
        zip_code: Some(data.zip_code.clone()),
        credit_cards: Some(Vec::new()),
    };

    log::info!("Adding user to store: {}", username);

    match app_state.users_store.add_user(new_user).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "User registered successfully",
            "errorCode": "SUCCESS"
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "message": "Error saving user",
            "errorCode": "USER_SAVE_ERROR"
        })),
    }
}

#[post("/signIn")]
pub async fn sign_in(
    data: web::Json<SignInData>,
    session: Session,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Ok(Some(_)) = session.get::<Uuid>("user_id") {
        return HttpResponse::BadRequest().json(json!({
            "message": "You are already logged in",
            "errorCode": "ALREADY_LOGGED_IN"
        }));
    }

    let login_or_email = data.login.trim();
    let password = data.password.trim();

    if login_or_email.is_empty() || password.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "login/email and password cannot be empty",
            "errorCode": "EMPTY_FIELDS"
        }));
    }

    let user = match app_state.users_store.find_user_by_login_or_email(login_or_email).await {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized().json(json!({
                "message": "Invalid credentials",
                "errorCode": "INVALID_CREDENTIALS"
            }));
        }
    };

    match verify(password, &user.password_hash) {
        Ok(true) => {
            {
                let mut users = app_state.users_store.users.lock().await;
                if let Some(existing_user) = users.iter_mut().find(|u| u.id == user.id) {
                    existing_user.last_login_date = Some(Utc::now());
                }
            }

            app_state.users_store.save().await.unwrap();

            session.insert("user_id", user.id).unwrap();

            HttpResponse::Ok().json(json!({
                "message": "Logged in successfully"
            }))
        }
        _ => HttpResponse::Unauthorized().json(json!({
            "message": "Invalid credentials",
            "errorCode": "INVALID_CREDENTIALS"
        })),
    }
}

#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    if let Ok(Some(_user_id)) = session.get::<Uuid>("user_id") {
        session.remove("user_id");
        HttpResponse::Ok().json(json!({
            "message": "Logged out successfully"
        }))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "Unauthorized",
            "errorCode": "UNAUTHORIZED_ACCESS"
        }))
    }
}