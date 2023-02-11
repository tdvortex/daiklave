use axum::response::Response;

/// Handles GET requests received after the user returns from Discord's OAuth2
/// endpoint with an authorization code.
pub async fn handle_login_callback() -> Response {
    todo!()
}