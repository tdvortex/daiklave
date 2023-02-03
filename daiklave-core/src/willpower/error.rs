use thiserror::Error;

#[derive(Debug, Error)]
pub enum WillpowerError {
    #[error("Cannot spend more Willpower than you have")]
    InsufficientWillpower,
    #[error("Willpower must be rated between 1 and 10")]
    InvalidRating,
}
