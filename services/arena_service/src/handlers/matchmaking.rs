use axum::{Extension, Json, http::StatusCode};
use std::sync::Arc;
use crate::{models::matchmaking::MatchmakingResponse, AppState};

pub async fn matchmaking(
    Extension(state): Extension<AppState>,
) -> Result<Json<MatchmakingResponse>, StatusCode> {
        // Acceder a la cola FIFO
    let mut queue = state.matchmaking_queue.lock().await;

    // Ejemplo: meter jugador a la cola
    let player_id = "jugador123".to_string();
    queue.push_back(player_id);

    // Aquí puedes hacer matchmaking (emparejar si hay 2 o más jugadores)
    // y crear arena, etc...

    // Ejemplo respuesta
    Ok(Json(MatchmakingResponse {
        status: "waiting".to_string(),
        arena_id: "".to_string(),
    }))
}