use thiserror::Error;

/// An error related to Experience points.
#[derive(Debug, Error)]
pub enum ExperienceError {
    /// Cannot spend more Experience than you have
    #[error("Not enough experience")]
    InsufficientExperience,
}
