use axum::{routing::get, Router};
use axum_test::TestServer;
use super::mock_tmdb_client::MockTmdbClient;
use netflix_service::{error::TmdbError, handlers, models, state::AppState};
use std::sync::Arc;

fn create_test_app() -> Router {
    let tmdb_client = Arc::new(MockTmdbClient::new());

    let state = AppState {
        tmdb_client,
    };

    Router::new()
        .route("/", get(handlers::root))
        .route("/api/trending", get(handlers::get_trending_movies))
        .route("/api/search", get(handlers::search_content))
        .route("/api/movie/{id}/videos", get(handlers::get_movie_videos))
        .with_state(state)
}

fn create_test_app_with_client(client: MockTmdbClient) -> Router {
    let state = AppState {
        tmdb_client: Arc::new(client),
    };

    Router::new()
        .route("/", get(handlers::root))
        .route("/api/trending", get(handlers::get_trending_movies))
        .route("/api/search", get(handlers::search_content))
        .route("/api/movie/{id}/videos", get(handlers::get_movie_videos))
        .with_state(state)
}

#[tokio::test]
async fn test_root_endpoint() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(response.text(), "Netflix Backend is Online");
}

#[tokio::test]
async fn test_trending_movies_endpoint() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending").await;

    assert_eq!(response.status_code(), 200);

    let body: models::TmdbResponse = response.json();
    assert_eq!(body.page, 1);
    assert_eq!(body.total_pages, 10);
    assert_eq!(body.results.len(), 2);

    // Verify first result
    assert_eq!(body.results[0].id, 123);
    assert_eq!(body.results[0].title, Some("Test Movie 1".to_string()));
    assert_eq!(body.results[0].media_type, Some("movie".to_string()));

    // Verify second result
    assert_eq!(body.results[1].id, 456);
    assert_eq!(body.results[1].name, Some("Test Show 1".to_string()));
    assert_eq!(body.results[1].media_type, Some("tv".to_string()));
}

#[tokio::test]
async fn test_trending_movies_with_page_param() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending?page=2").await;

    assert_eq!(response.status_code(), 200);

    let body: models::TmdbResponse = response.json();
    assert_eq!(body.page, 2);
}

#[tokio::test]
async fn test_search_content_endpoint() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/search?query=avengers").await;

    assert_eq!(response.status_code(), 200);

    let body: models::TmdbResponse = response.json();
    assert_eq!(body.page, 1);
    assert_eq!(body.total_pages, 5);
    assert_eq!(body.results.len(), 1);

    // Verify search result contains the query
    assert_eq!(body.results[0].id, 789);
    assert!(body.results[0].title.as_ref().unwrap().contains("avengers"));
}

#[tokio::test]
async fn test_search_content_requires_query() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/search").await;

    // Should fail without query parameter
    assert_ne!(response.status_code(), 200);
}

#[tokio::test]
async fn test_get_movie_videos_endpoint() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/movie/550/videos").await;

    assert_eq!(response.status_code(), 200);

    let body: models::VideoResponse = response.json();
    assert_eq!(body.id, 550);
    assert_eq!(body.results.len(), 2);

    // Verify first video
    assert_eq!(body.results[0].id, "video123");
    assert_eq!(body.results[0].key, "abc123xyz");
    assert_eq!(body.results[0].site, "YouTube");
    assert_eq!(body.results[0].r#type, "Trailer");

    // Verify second video
    assert_eq!(body.results[1].id, "video456");
    assert_eq!(body.results[1].key, "def456uvw");
    assert_eq!(body.results[1].r#type, "Teaser");
}

#[tokio::test]
async fn test_movie_videos_path_parameter() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    // Test with different movie ID
    let response = server.get("/api/movie/299536/videos").await;

    assert_eq!(response.status_code(), 200);

    let body: models::VideoResponse = response.json();
    assert_eq!(body.id, 299536);
}

#[tokio::test]
async fn test_search_with_page_parameter() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/search?query=matrix&page=3").await;

    assert_eq!(response.status_code(), 200);

    let body: models::TmdbResponse = response.json();
    assert_eq!(body.page, 3);
    assert!(body.results[0].title.as_ref().unwrap().contains("matrix"));
}

#[tokio::test]
async fn test_trending_default_page() {
    let app = create_test_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending").await;

    assert_eq!(response.status_code(), 200);

    let body: models::TmdbResponse = response.json();
    // Default page should be 1
    assert_eq!(body.page, 1);
}

// ========== Error Scenario Tests ==========

#[tokio::test]
async fn test_trending_not_found_error() {
    let mock_client = MockTmdbClient::builder()
        .with_trending_error(1, TmdbError::NotFound)
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending").await;

    assert_eq!(response.status_code(), 404);
    assert_eq!(response.text(), "Resource not found");
}

#[tokio::test]
async fn test_trending_unauthorized_error() {
    let mock_client = MockTmdbClient::builder()
        .with_trending_error(1, TmdbError::Unauthorized)
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending").await;

    assert_eq!(response.status_code(), 401);
    assert_eq!(response.text(), "Invalid or missing API key");
}

#[tokio::test]
async fn test_trending_rate_limit_error() {
    let mock_client = MockTmdbClient::builder()
        .with_trending_error(1, TmdbError::RateLimitExceeded)
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending").await;

    assert_eq!(response.status_code(), 429);
    assert_eq!(response.text(), "Rate limit exceeded");
}

#[tokio::test]
async fn test_trending_server_error() {
    let mock_client = MockTmdbClient::builder()
        .with_trending_error(1, TmdbError::ServerError(503))
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending").await;

    assert_eq!(response.status_code(), 502);
    assert_eq!(response.text(), "Upstream server error");
}

#[tokio::test]
async fn test_search_not_found_error() {
    let mock_client = MockTmdbClient::builder()
        .with_search_error("nonexistent", 1, TmdbError::NotFound)
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/search?query=nonexistent").await;

    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
async fn test_movie_videos_not_found() {
    let mock_client = MockTmdbClient::builder()
        .with_video_error(99999, TmdbError::NotFound)
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/movie/99999/videos").await;

    assert_eq!(response.status_code(), 404);
    assert_eq!(response.text(), "Resource not found");
}

// ========== Custom Response Tests ==========

#[tokio::test]
async fn test_custom_trending_response() {
    let custom_response = models::TmdbResponse {
        page: 1,
        total_pages: 1,
        results: vec![models::Movie {
            id: 999,
            title: Some("Custom Movie".to_string()),
            name: None,
            overview: Some("Custom overview".to_string()),
            poster_path: None,
            backdrop_path: None,
            vote_average: Some(10.0),
            release_date: None,
            media_type: Some("movie".to_string()),
        }],
    };

    let mock_client = MockTmdbClient::builder()
        .with_trending_response(1, Ok(custom_response))
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/trending").await;

    assert_eq!(response.status_code(), 200);

    let body: models::TmdbResponse = response.json();
    assert_eq!(body.results.len(), 1);
    assert_eq!(body.results[0].id, 999);
    assert_eq!(body.results[0].title, Some("Custom Movie".to_string()));
    assert_eq!(body.results[0].vote_average, Some(10.0));
}

#[tokio::test]
async fn test_default_error_for_all_trending() {
    let mock_client = MockTmdbClient::builder()
        .with_default_trending(Err(TmdbError::RateLimitExceeded))
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    // Test multiple pages, all should fail
    let response1 = server.get("/api/trending?page=1").await;
    let response2 = server.get("/api/trending?page=2").await;

    assert_eq!(response1.status_code(), 429);
    assert_eq!(response2.status_code(), 429);
}

#[tokio::test]
async fn test_specific_page_override() {
    // Set default to error, but page 3 succeeds
    let custom_response = models::TmdbResponse {
        page: 3,
        total_pages: 5,
        results: vec![],
    };

    let mock_client = MockTmdbClient::builder()
        .with_default_trending(Err(TmdbError::RateLimitExceeded))
        .with_trending_response(3, Ok(custom_response))
        .build();

    let app = create_test_app_with_client(mock_client);
    let server = TestServer::new(app).unwrap();

    let response1 = server.get("/api/trending?page=1").await;
    let response3 = server.get("/api/trending?page=3").await;

    assert_eq!(response1.status_code(), 429);
    assert_eq!(response3.status_code(), 200);
}
