use netflix_service::models::{Movie, TmdbResponse, Video, VideoResponse, PageQuery, SearchQuery};
use serde_json;

#[test]
fn test_movie_serialization() {
    let movie = Movie {
        id: 123,
        title: Some("Test Movie".to_string()),
        name: None,
        overview: Some("A test".to_string()),
        poster_path: Some("/poster.jpg".to_string()),
        backdrop_path: Some("/backdrop.jpg".to_string()),
        vote_average: Some(8.5),
        release_date: Some("2024-01-01".to_string()),
        media_type: Some("movie".to_string()),
    };

    let json = serde_json::to_string(&movie).unwrap();
    assert!(json.contains("\"id\":123"));
    assert!(json.contains("\"title\":\"Test Movie\""));
}

#[test]
fn test_movie_deserialization() {
    let json = r#"{
        "id": 456,
        "title": "Deserialized Movie",
        "overview": "Test overview",
        "vote_average": 7.5
    }"#;

    let movie: Movie = serde_json::from_str(json).unwrap();
    assert_eq!(movie.id, 456);
    assert_eq!(movie.title, Some("Deserialized Movie".to_string()));
    assert_eq!(movie.vote_average, Some(7.5));
}

#[test]
fn test_tmdb_response_structure() {
    let response = TmdbResponse {
        page: 1,
        total_pages: 5,
        results: vec![
            Movie {
                id: 1,
                title: Some("Movie 1".to_string()),
                name: None,
                overview: None,
                poster_path: None,
                backdrop_path: None,
                vote_average: None,
                release_date: None,
                media_type: None,
            },
            Movie {
                id: 2,
                title: Some("Movie 2".to_string()),
                name: None,
                overview: None,
                poster_path: None,
                backdrop_path: None,
                vote_average: None,
                release_date: None,
                media_type: None,
            },
        ],
    };

    assert_eq!(response.page, 1);
    assert_eq!(response.total_pages, 5);
    assert_eq!(response.results.len(), 2);
}

#[test]
fn test_video_response_structure() {
    let response = VideoResponse {
        id: 789,
        results: vec![
            Video {
                id: "vid1".to_string(),
                key: "abc123".to_string(),
                site: "YouTube".to_string(),
                r#type: "Trailer".to_string(),
                name: "Official Trailer".to_string(),
            },
        ],
    };

    assert_eq!(response.id, 789);
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].site, "YouTube");
}

#[test]
fn test_page_query_default() {
    let query = PageQuery { page: None };
    assert!(query.page.is_none());

    let query = PageQuery { page: Some(5) };
    assert_eq!(query.page, Some(5));
}

#[test]
fn test_search_query_structure() {
    let query = SearchQuery {
        query: "avengers".to_string(),
        page: Some(2),
    };

    assert_eq!(query.query, "avengers");
    assert_eq!(query.page, Some(2));
}

#[test]
fn test_movie_with_tv_show_fields() {
    // Test that TV show can use 'name' field instead of 'title'
    let tv_show = Movie {
        id: 999,
        title: None,
        name: Some("TV Show Name".to_string()),
        overview: Some("A TV show".to_string()),
        poster_path: None,
        backdrop_path: None,
        vote_average: Some(8.0),
        release_date: None,
        media_type: Some("tv".to_string()),
    };

    assert_eq!(tv_show.name, Some("TV Show Name".to_string()));
    assert!(tv_show.title.is_none());
}

#[test]
fn test_movie_optional_fields() {
    // Test that all optional fields can be None
    let minimal_movie = Movie {
        id: 100,
        title: None,
        name: None,
        overview: None,
        poster_path: None,
        backdrop_path: None,
        vote_average: None,
        release_date: None,
        media_type: None,
    };

    assert_eq!(minimal_movie.id, 100);
    assert!(minimal_movie.title.is_none());
    assert!(minimal_movie.overview.is_none());
}
