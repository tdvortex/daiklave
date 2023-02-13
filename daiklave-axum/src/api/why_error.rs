use axum::Json;
use hyper::StatusCode;
use serde::Serialize;

/// A generic error response with a message explaining why the request was
/// unsuccessful.
#[derive(Serialize)]
pub struct WhyError {
    /// The reason the request could not be completed.
    pub why: String,
}

/// A response of 500 Internal Server Error with an opaque message.
pub fn internal_server_error() -> (StatusCode, Json<WhyError>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(WhyError {
        why: "internal server error".to_owned()
    }))
}

/// A response of 401 Unauthorized telling the user they are not logged in.
pub fn not_logged_in() -> (StatusCode, Json<WhyError>) {
    (StatusCode::UNAUTHORIZED, Json(WhyError {
        why: "not logged in".to_owned()
    }))
}

/// A response of 404 Not Found. This is also returned for items that do exist
/// but where the user doesn't have the authorization to know that they exist.
pub fn not_found() -> (StatusCode, Json<WhyError>) {
    (StatusCode::NOT_FOUND, Json(WhyError {
        why: "not found".to_owned()
    }))
}

/// A response of 403 Forbidden with an additional clarification that this
/// action requires storyteller privileges.
pub fn not_storyteller() -> (StatusCode, Json<WhyError>) {
    (StatusCode::FORBIDDEN, Json(WhyError {
        why: "not the storyteller".to_owned()
    }))
}
