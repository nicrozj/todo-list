use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use axum::extract::{Form, State};
use serde::Deserialize;
use axum::response::{Html, IntoResponse};
use chrono::{Duration, Utc};

use crate::views::auth::auth_view;
use crate::model::user::User;
use crate::Database;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", get(auth_page))
        .route("/", post(login))
}

async fn auth_page() -> impl IntoResponse {
    Html(auth_view())
}

#[derive(Deserialize)]
pub struct LoginForm {
    login: String,
    password: String,
}

async fn login(State(db): State<Database>, headers: HeaderMap, Form(form): Form<LoginForm>) -> Result<impl IntoResponse, impl IntoResponse> {
    let is_auth = match User::login(&form.login, &form.password, &db).await {
        Ok(is_auth) => is_auth,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    let user_id = match User::get_user_id_from_login(&form.login, &db).await {
        Ok(value) => value,
        Err(e) => panic!("Error: {}!", e),
    };

    let user_agent = headers.get("User-Agent")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let expire_date = Utc::now().naive_utc() + Duration::days(14);

    let token = User::create_token(user_id, &form.login, user_agent, &expire_date, &db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = format!("token={}", token);
    let headers: [(&str, String); 2] = [
        ("set-cookie", cookie),
        ("hx-location", "/".to_string())
    ];

    match is_auth {
        true => Ok((headers, StatusCode::OK)),
        false => Err(StatusCode::FORBIDDEN)
    }
}
