use axum::{self, response::Html, routing::get};
use cercis::prelude::*;
mod components;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    let app = axum::Router::new().route("/", get(todo_list));

    axum::serve(listener, app).await.unwrap();
}

async fn todo_list() -> Html<String> {
    Html(rsx!(Page { title: "Test" }).render())
}
