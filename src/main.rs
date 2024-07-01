use axum::Router;
use simplelog::*;

pub use self::model::database::Database;

mod endpoints;
mod views;
mod model;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    SimpleLogger::init(LevelFilter::Info, Config::default()).ok();

    let database = Database::from_env();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    let app = Router::new().nest("/", endpoints::get_nest(database.clone())).with_state(database);

    axum::serve(listener, app).await.unwrap();
}
