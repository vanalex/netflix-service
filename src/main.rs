// src/main.rs
use axum::{routing::get, Router};
use dotenv::dotenv;
use std::{env, sync::Arc};
use tower_http::{cors::CorsLayer, services::ServeDir};
use netflix_service::{handlers, state::AppState, tmdb_client::RealTmdbClient};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set in .env");

    let tmdb_client = Arc::new(RealTmdbClient::new(api_key));

    let state = AppState {
        tmdb_client,
    };

    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/api/trending", get(handlers::get_trending_movies))
        .route("/api/search", get(handlers::search_content))
        .route("/api/movie/{id}/videos", get(handlers::get_movie_videos))
        .nest_service("/stream", ServeDir::new("assets"))
        .layer(cors)
        .with_state(state);

    //let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}