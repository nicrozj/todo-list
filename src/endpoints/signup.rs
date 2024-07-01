use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use axum::response::{Html, IntoResponse};
use serde::Deserialize;
use axum::extract::{Form, State};

use crate::model::user::User;
use crate::views::signup::signup_view;
use crate::Database;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", get(signup_page))
        .route("/", post(signup))
}

async fn signup_page() -> impl IntoResponse {
    Html(signup_view())
}

#[derive(Deserialize)]
struct SignupForm {
    login: String,
    password: String,
    repeat_password: String,
}

async fn signup(State(database): State<Database>, Form(form): Form<SignupForm>) -> Result<impl IntoResponse, impl IntoResponse> {
    const HEADERS: [(&str, &str); 1] = [("hx-location", "/auth")];

    let is_login_free = match User::is_login_free(&form.login, &database).await {
        Ok(is_login_free) => is_login_free,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    if !(form.password == form.repeat_password && is_login_free) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match User::signup(&form.login, &form.password, &database).await {
        Ok(_) => Ok((HEADERS, StatusCode::OK)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
