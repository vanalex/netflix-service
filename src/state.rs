// src/state.rs
use crate::tmdb_client::TmdbClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub tmdb_client: Arc<dyn TmdbClient>,
}