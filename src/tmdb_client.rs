use crate::error::TmdbError;
use crate::models::{TmdbResponse, VideoResponse};
use async_trait::async_trait;

/// Trait defining the contract for TMDB API operations.
///
/// All methods return `Result<T, TmdbError>` where:
/// - `Ok(T)` contains the successfully parsed response
/// - `Err(TmdbError)` provides detailed error information
///
/// Implementations should handle HTTP status codes appropriately
/// and convert them to the corresponding `TmdbError` variants.
#[async_trait]
pub trait TmdbClient: Send + Sync {
    /// Fetches trending movies/TV shows for the week
    ///
    /// # Arguments
    /// * `page` - Page number (1-indexed)
    ///
    /// # Errors
    /// Returns `TmdbError` if the request fails or response cannot be parsed
    async fn get_trending(&self, page: i32) -> Result<TmdbResponse, TmdbError>;

    /// Searches for content (movies/TV shows) by query string
    ///
    /// # Arguments
    /// * `query` - Search query string
    /// * `page` - Page number (1-indexed)
    ///
    /// # Errors
    /// Returns `TmdbError` if the request fails or response cannot be parsed
    async fn search_content(&self, query: &str, page: i32) -> Result<TmdbResponse, TmdbError>;

    /// Fetches videos (trailers, teasers, etc.) for a specific movie
    ///
    /// # Arguments
    /// * `movie_id` - TMDB movie ID
    ///
    /// # Errors
    /// Returns `TmdbError::NotFound` if movie doesn't exist
    /// Returns other `TmdbError` variants for request/parse failures
    async fn get_movie_videos(&self, movie_id: i32) -> Result<VideoResponse, TmdbError>;
}

pub struct RealTmdbClient {
    api_key: String,
    client: reqwest::Client,
}

impl RealTmdbClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl TmdbClient for RealTmdbClient {
    async fn get_trending(&self, page: i32) -> Result<TmdbResponse, TmdbError> {
        let url = format!(
            "https://api.themoviedb.org/3/trending/all/week?api_key={}&page={}",
            self.api_key, page
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(TmdbError::from_status(status, body));
        }

        let data = response.json::<TmdbResponse>().await?;
        Ok(data)
    }

    async fn search_content(&self, query: &str, page: i32) -> Result<TmdbResponse, TmdbError> {
        let url = format!(
            "https://api.themoviedb.org/3/search/multi?api_key={}&query={}&page={}&include_adult=false",
            self.api_key, query, page
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(TmdbError::from_status(status, body));
        }

        let data = response.json::<TmdbResponse>().await?;
        Ok(data)
    }

    async fn get_movie_videos(&self, movie_id: i32) -> Result<VideoResponse, TmdbError> {
        let url = format!(
            "https://api.themoviedb.org/3/movie/{}/videos?api_key={}",
            movie_id, self.api_key
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(TmdbError::from_status(status, body));
        }

        let data = response.json::<VideoResponse>().await?;
        Ok(data)
    }
}
