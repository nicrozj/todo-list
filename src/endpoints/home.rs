use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use axum::response::{Html, IntoResponse, Redirect};
use axum::extract::{Path, Request, State};
use chrono::NaiveDate;
use axum::middleware::map_request_with_state;

use crate::model::user::User;
use crate::views::home::home_view;
use crate::model::session::*;
use crate::Database;

pub fn get_nest(database: Database) -> Router<Database> {
    Router::new()
        .route("/:date", get(home_page))
        .route("/:date/tasks", get(get_tasks))
        .layer(map_request_with_state(database, auth_middleware))
}

async fn home_page(Path(date): Path<NaiveDate>) -> impl IntoResponse {
    Html(home_view(date))
}

async fn get_tasks(Path(date): Path<NaiveDate>, Extension(user): Extension<User>) -> impl IntoResponse {
    format!("{} {}", date, user.get_login())
}

async fn auth_middleware(State(database): State<Database>, mut req: Request) -> Result<Request, impl IntoResponse> {
    let cookie = match req.headers().get("cookie") {
        Some(cookie) => cookie.clone(),
        None => return Err(Redirect::to("/auth"))
    };

    let user_agent = match req.headers().get("user-agent") {
        Some(user_agent) => user_agent.clone(),
        None => return Err(Redirect::to("/auth"))
    };

    let user_agent = user_agent.to_str().unwrap_or_default();

    let token = match cookie.to_str().unwrap_or_default().split_once('=') {
        Some(token) => token.1,
        None => return Err(Redirect::to("/auth"))
    };

    let session = Session::from_token(token, &database)
        .await
        .map_err(|_| Redirect::to("/auth"))?
        .ok_or(Redirect::to("/auth"))?;

    if !session.is_valid_token() {
        return Err(Redirect::to("/auth"));
    }

    if !session.is_valid_user_agent(user_agent) {
        return Err(Redirect::to("/auth"));
    }

    let user = User::from_id(session.get_user_id(), &database)
        .await
        .map_err(|_| Redirect::to("/auth"))?
        .ok_or(Redirect::to("/auth"))?;

    req.extensions_mut().insert(user);

    Ok(req)
}
