use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    timestamp: u64,
    version: &'static str,
}

async fn health_handler() -> (StatusCode, Json<HealthResponse>) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            timestamp,
            version: env!("CARGO_PKG_VERSION"),
        }),
    )
}

pub fn router() -> Router {
    Router::new().route("/health", get(health_handler))
}

#[tokio::main]
async fn main() {
    let app = router();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::Request,
    };
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_health_endpoint_returns_200() {
        let app = router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_health_endpoint_returns_json() {
        use axum::body::to_bytes;

        let app = router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        assert!(json["timestamp"].is_u64());
        assert!(json["version"].is_string());
    }
}
