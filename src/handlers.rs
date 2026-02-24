use axum::{ extract::{ Path, Query, State }, Json, http::StatusCode, response::IntoResponse };
use crate::error::TmdbError;
use crate::models::{ PageQuery, SearchQuery };
use crate::state::AppState;

pub async fn root() -> &'static str {
    "Netflix Backend is Online"
}

pub async fn get_trending_movies(
    State(state): State<AppState>,
    Query(params): Query<PageQuery>
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);

    match state.tmdb_client.get_trending(page).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => map_error_to_response(e).into_response(),
    }
}

pub async fn search_content(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);

    match state.tmdb_client.search_content(&params.query, page).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => map_error_to_response(e).into_response(),
    }
}

pub async fn get_movie_videos(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    match state.tmdb_client.get_movie_videos(id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => map_error_to_response(e).into_response(),
    }
}

/// Maps TmdbError to appropriate HTTP response
fn map_error_to_response(error: TmdbError) -> (StatusCode, &'static str) {
    match error {
        TmdbError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
        TmdbError::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid or missing API key"),
        TmdbError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"),
        TmdbError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
        TmdbError::ServerError(_) => (StatusCode::BAD_GATEWAY, "Upstream server error"),
        TmdbError::NetworkError(_) => (StatusCode::SERVICE_UNAVAILABLE, "Network error occurred"),
        TmdbError::ParseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse response"),
        TmdbError::Unknown(_, _) => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error occurred"),
    }
}
