use axum::response::{IntoResponse, Redirect};
use axum::Router;
use axum::routing::get;
use chrono::Utc;

use crate::Database;

pub fn get_nest() -> Router<Database> {
    Router::new().route("/", get(redirect_actual_date))
}

async fn redirect_actual_date() -> impl IntoResponse {
    let now = Utc::now().naive_utc();
    let uri = format!("/home/{}", now.format("%Y-%m-%d"));

    Redirect::to(&uri)
}
