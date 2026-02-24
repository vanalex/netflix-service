use netflix_service::error::TmdbError;

#[test]
fn test_error_display() {
    let error = TmdbError::NotFound;
    assert_eq!(error.to_string(), "Resource not found");

    let error = TmdbError::Unauthorized;
    assert_eq!(error.to_string(), "Unauthorized: Invalid or missing API key");

    let error = TmdbError::RateLimitExceeded;
    assert_eq!(error.to_string(), "API rate limit exceeded");

    let error = TmdbError::ServerError(503);
    assert_eq!(error.to_string(), "Server error: 503");
}

#[tokio::test]
async fn test_error_from_reqwest() {
    // Test conversion from reqwest::Error to TmdbError
    let reqwest_error = reqwest::get("http://invalid-url-that-does-not-exist-12345.com")
        .await
        .unwrap_err();

    let tmdb_error: TmdbError = reqwest_error.into();

    match tmdb_error {
        TmdbError::NetworkError(_) => {}, // Expected
        _ => panic!("Expected NetworkError variant"),
    }
}

#[test]
fn test_error_from_status_codes() {
    let status = reqwest::StatusCode::NOT_FOUND;
    let error = TmdbError::from_status(status, "Not found".to_string());
    assert!(matches!(error, TmdbError::NotFound));

    let status = reqwest::StatusCode::UNAUTHORIZED;
    let error = TmdbError::from_status(status, "Unauthorized".to_string());
    assert!(matches!(error, TmdbError::Unauthorized));

    let status = reqwest::StatusCode::TOO_MANY_REQUESTS;
    let error = TmdbError::from_status(status, "Rate limit".to_string());
    assert!(matches!(error, TmdbError::RateLimitExceeded));

    let status = reqwest::StatusCode::BAD_REQUEST;
    let error = TmdbError::from_status(status, "Bad request".to_string());
    assert!(matches!(error, TmdbError::BadRequest(_)));

    let status = reqwest::StatusCode::INTERNAL_SERVER_ERROR;
    let error = TmdbError::from_status(status, "Server error".to_string());
    assert!(matches!(error, TmdbError::ServerError(500)));

    let status = reqwest::StatusCode::IM_A_TEAPOT;
    let error = TmdbError::from_status(status, "Teapot".to_string());
    assert!(matches!(error, TmdbError::Unknown(418, _)));
}

#[test]
fn test_error_is_retryable() {
    assert!(TmdbError::NetworkError("timeout".to_string()).is_retryable());
    assert!(TmdbError::RateLimitExceeded.is_retryable());
    assert!(TmdbError::ServerError(503).is_retryable());

    assert!(!TmdbError::NotFound.is_retryable());
    assert!(!TmdbError::Unauthorized.is_retryable());
    assert!(!TmdbError::BadRequest("invalid".to_string()).is_retryable());
}

#[test]
fn test_error_clone() {
    let error = TmdbError::NotFound;
    let cloned = error.clone();

    assert!(matches!(cloned, TmdbError::NotFound));
}

#[test]
fn test_error_debug() {
    let error = TmdbError::NotFound;
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("NotFound"));
}
