use axum::Router;

use crate::Database;

mod root;
mod home;
mod auth;
mod signup;

pub fn get_nest(database: Database) -> Router<Database> {
    Router::new()
        .nest("/", root::get_nest())
        .nest("/auth", auth::get_nest())
        .nest("/signup", signup::get_nest())
        .nest("/home", home::get_nest(database))
}
