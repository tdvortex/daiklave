use serde::{Deserialize, Serialize};

use super::{details::HearthstoneDetailsMemo, stability::HearthstoneStability};

/// A template for a Hearthstone to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HearthstoneTemplate {
    pub(crate) details: HearthstoneDetailsMemo,
    pub(crate) stability: HearthstoneStability,
}
