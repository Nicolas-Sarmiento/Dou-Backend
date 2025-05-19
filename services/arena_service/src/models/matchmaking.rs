use std::collections::VecDeque;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub user_email: String,
    pub user_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchmakingResponse {
    pub status: String,
    pub arena_id: String,
}

type PlayerId = String;
type ArenaId = String;
type ArenaInfo = ();

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub matchmaking_queue: Arc<Mutex<VecDeque<PlayerId>>>,
    pub arenas: Arc<Mutex<std::collections::HashMap<ArenaId, ArenaInfo>>>,
}
