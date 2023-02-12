use axum::response::IntoResponse;

/// Routes related to a specific character. 
pub mod character;

/// TODO
pub async fn post_character() -> impl IntoResponse {
    todo!()
}