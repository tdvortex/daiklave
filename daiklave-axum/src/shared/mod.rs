/// Common functionality related to authorization.
pub mod authorization;

/// Errors that may be returned from attempting to perform an operation.
/// These should be handled by either returning an appropriate HTTP status code
/// (such as 404 Not Found or 500 Internal Server Error) for an API response,
/// or 200 Ok with a Discord message payload describing the error.
pub mod error;