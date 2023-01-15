use serde::{Serialize, Deserialize};

/// The purchase constraints on a specific merit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeritType {
    /// Innate merits may only be purchased at character creation except with
    /// Storyteller permission or dramatic story effects like Wyld mutation.
    Innate,
    /// Purchased merits can be purchased whenever desired using experience
    /// points.
    Purchased,
    /// Story merits can be added whenever the story demands it, including at
    /// character creation, but cannot be purchased with experience points
    /// directly.
    Story,
    /// Supernatural merits are generally not purchasable without explicit
    /// Storyteller permission.
    Supernatural,
}