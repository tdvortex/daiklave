use serde::Serialize;

/// A generic error response with a message explaining why the request was
/// unsuccessful.
#[derive(Serialize)]
pub struct WhyError {
    /// The reason the request could not be completed.
    pub why: String,
}