use serde::{Serialize, Deserialize};

use super::{stability::HearthstoneStability, details::HearthstoneDetailsMemo};

/// A template for a Hearthstone to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HearthstoneTemplate {
    details: HearthstoneDetailsMemo,
    stability: HearthstoneStability,
}
