use netflix_service::error::TmdbError;
use netflix_service::models::{Movie, TmdbResponse, Video, VideoResponse};
use netflix_service::tmdb_client::TmdbClient;
use async_trait::async_trait;
use std::collections::HashMap;

/// Mock implementation of TmdbClient for testing purposes.
///
/// Supports configurable responses for different scenarios including:
/// - Success responses with custom data
/// - Error responses for testing error handling
/// - Default responses when no specific configuration is provided
///
/// # Example
/// ```ignore
/// use netflix_service::error::TmdbError;
/// use mock_tmdb_client::MockTmdbClient;
///
/// let mock = MockTmdbClient::builder()
///     .with_search_error("error_query", 1, TmdbError::NotFound)
///     .build();
/// ```
pub struct MockTmdbClient {
    trending_responses: HashMap<i32, Result<TmdbResponse, TmdbError>>,
    search_responses: HashMap<(String, i32), Result<TmdbResponse, TmdbError>>,
    video_responses: HashMap<i32, Result<VideoResponse, TmdbError>>,
    default_trending: Option<Result<TmdbResponse, TmdbError>>,
    default_search: Option<Result<TmdbResponse, TmdbError>>,
    default_video: Option<Result<VideoResponse, TmdbError>>,
}

impl MockTmdbClient {
    /// Creates a new MockTmdbClient with default success responses
    pub fn new() -> Self {
        Self {
            trending_responses: HashMap::new(),
            search_responses: HashMap::new(),
            video_responses: HashMap::new(),
            default_trending: None,
            default_search: None,
            default_video: None,
        }
    }

    /// Creates a builder for configuring mock responses
    pub fn builder() -> MockTmdbClientBuilder {
        MockTmdbClientBuilder::new()
    }

    fn default_trending_response(&self, page: i32) -> Result<TmdbResponse, TmdbError> {
        Ok(TmdbResponse {
            page,
            total_pages: 10,
            results: vec![
                Movie {
                    id: 123,
                    title: Some("Test Movie 1".to_string()),
                    name: None,
                    overview: Some("A great test movie".to_string()),
                    poster_path: Some("/test1.jpg".to_string()),
                    backdrop_path: Some("/backdrop1.jpg".to_string()),
                    vote_average: Some(8.5),
                    release_date: Some("2024-01-01".to_string()),
                    media_type: Some("movie".to_string()),
                },
                Movie {
                    id: 456,
                    title: None,
                    name: Some("Test Show 1".to_string()),
                    overview: Some("A great test show".to_string()),
                    poster_path: Some("/test2.jpg".to_string()),
                    backdrop_path: Some("/backdrop2.jpg".to_string()),
                    vote_average: Some(7.8),
                    release_date: Some("2024-02-01".to_string()),
                    media_type: Some("tv".to_string()),
                },
            ],
        })
    }

    fn default_search_response(&self, query: &str, page: i32) -> Result<TmdbResponse, TmdbError> {
        Ok(TmdbResponse {
            page,
            total_pages: 5,
            results: vec![
                Movie {
                    id: 789,
                    title: Some(format!("Search Result for '{}'", query)),
                    name: None,
                    overview: Some("Matching content".to_string()),
                    poster_path: Some("/search.jpg".to_string()),
                    backdrop_path: Some("/search_backdrop.jpg".to_string()),
                    vote_average: Some(9.0),
                    release_date: Some("2023-12-01".to_string()),
                    media_type: Some("movie".to_string()),
                },
            ],
        })
    }

    fn default_video_response(&self, movie_id: i32) -> Result<VideoResponse, TmdbError> {
        Ok(VideoResponse {
            id: movie_id,
            results: vec![
                Video {
                    id: "video123".to_string(),
                    key: "abc123xyz".to_string(),
                    site: "YouTube".to_string(),
                    r#type: "Trailer".to_string(),
                    name: "Official Trailer".to_string(),
                },
                Video {
                    id: "video456".to_string(),
                    key: "def456uvw".to_string(),
                    site: "YouTube".to_string(),
                    r#type: "Teaser".to_string(),
                    name: "Teaser".to_string(),
                },
            ],
        })
    }
}

#[async_trait]
impl TmdbClient for MockTmdbClient {
    async fn get_trending(&self, page: i32) -> Result<TmdbResponse, TmdbError> {
        // Check for specific page response
        if let Some(response) = self.trending_responses.get(&page) {
            return response.clone();
        }

        // Fall back to default configured response
        if let Some(response) = &self.default_trending {
            return response.clone();
        }

        // Use built-in default
        self.default_trending_response(page)
    }

    async fn search_content(&self, query: &str, page: i32) -> Result<TmdbResponse, TmdbError> {
        let key = (query.to_string(), page);

        // Check for specific query/page response
        if let Some(response) = self.search_responses.get(&key) {
            return response.clone();
        }

        // Fall back to default configured response
        if let Some(response) = &self.default_search {
            return response.clone();
        }

        // Use built-in default
        self.default_search_response(query, page)
    }

    async fn get_movie_videos(&self, movie_id: i32) -> Result<VideoResponse, TmdbError> {
        // Check for specific movie ID response
        if let Some(response) = self.video_responses.get(&movie_id) {
            return response.clone();
        }

        // Fall back to default configured response
        if let Some(response) = &self.default_video {
            return response.clone();
        }

        // Use built-in default
        self.default_video_response(movie_id)
    }
}

/// Builder for creating MockTmdbClient with custom responses
pub struct MockTmdbClientBuilder {
    trending_responses: HashMap<i32, Result<TmdbResponse, TmdbError>>,
    search_responses: HashMap<(String, i32), Result<TmdbResponse, TmdbError>>,
    video_responses: HashMap<i32, Result<VideoResponse, TmdbError>>,
    default_trending: Option<Result<TmdbResponse, TmdbError>>,
    default_search: Option<Result<TmdbResponse, TmdbError>>,
    default_video: Option<Result<VideoResponse, TmdbError>>,
}

impl MockTmdbClientBuilder {
    pub fn new() -> Self {
        Self {
            trending_responses: HashMap::new(),
            search_responses: HashMap::new(),
            video_responses: HashMap::new(),
            default_trending: None,
            default_search: None,
            default_video: None,
        }
    }

    /// Set a specific response for a trending request with given page
    pub fn with_trending_response(mut self, page: i32, response: Result<TmdbResponse, TmdbError>) -> Self {
        self.trending_responses.insert(page, response);
        self
    }

    /// Set a default response for all trending requests
    pub fn with_default_trending(mut self, response: Result<TmdbResponse, TmdbError>) -> Self {
        self.default_trending = Some(response);
        self
    }

    /// Set a specific response for a search request with given query and page
    pub fn with_search_response(mut self, query: &str, page: i32, response: Result<TmdbResponse, TmdbError>) -> Self {
        self.search_responses.insert((query.to_string(), page), response);
        self
    }

    /// Set a default response for all search requests
    pub fn with_default_search(mut self, response: Result<TmdbResponse, TmdbError>) -> Self {
        self.default_search = Some(response);
        self
    }

    /// Set a specific response for a movie videos request with given movie ID
    pub fn with_video_response(mut self, movie_id: i32, response: Result<VideoResponse, TmdbError>) -> Self {
        self.video_responses.insert(movie_id, response);
        self
    }

    /// Set a default response for all movie video requests
    pub fn with_default_video(mut self, response: Result<VideoResponse, TmdbError>) -> Self {
        self.default_video = Some(response);
        self
    }

    /// Convenience method to set a trending error
    pub fn with_trending_error(self, page: i32, error: TmdbError) -> Self {
        self.with_trending_response(page, Err(error))
    }

    /// Convenience method to set a search error
    pub fn with_search_error(self, query: &str, page: i32, error: TmdbError) -> Self {
        self.with_search_response(query, page, Err(error))
    }

    /// Convenience method to set a video error
    pub fn with_video_error(self, movie_id: i32, error: TmdbError) -> Self {
        self.with_video_response(movie_id, Err(error))
    }

    /// Build the MockTmdbClient
    pub fn build(self) -> MockTmdbClient {
        MockTmdbClient {
            trending_responses: self.trending_responses,
            search_responses: self.search_responses,
            video_responses: self.video_responses,
            default_trending: self.default_trending,
            default_search: self.default_search,
            default_video: self.default_video,
        }
    }
}
