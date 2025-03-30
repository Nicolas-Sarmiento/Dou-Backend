// use axum::{Extension, Json, extract::Path, http::StatusCode};
// use sqlx::PgPool;
// use bcrypt::{hash, DEFAULT_COST};
// use serde::Deserialize;
// use crate::models::User;

// #[derive(Deserialize)]
// pub struct UpdateUser {
//     pub username: String,
//     pub user_password: String,
//     pub user_email: String,
//     pub user_role: i32,
// }

// pub async fn update_user(
//     Extension(pool): Extension<PgPool>,
//     Path(user_id): Path<i32>,  // Extraer el user_id desde la URL
//     Json(payload): Json<UpdateUser>,
// ) -> Result<Json<User>, StatusCode> {
//     // Hashear la nueva contraseña
//     let hashed_password = hash(payload.user_password, DEFAULT_COST)
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let user = sqlx::query_as!(
//         User,
//         r#"
//         UPDATE users 
//         SET username = $1, user_password = $2, user_email = $3, user_role = $4
//         WHERE user_id = $5
//         RETURNING user_id, username, user_email, user_role
//         "#,
//         payload.username,
//         hashed_password,
//         payload.user_email,
//         payload.user_role,
//         user_id  // Ahora tomamos el user_id de la URL
//     )
//     .fetch_one(&pool)
//     .await
//     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(Json(user))
// }


