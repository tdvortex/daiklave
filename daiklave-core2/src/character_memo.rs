use serde::{Deserialize, Serialize};

use crate::{
    abilities::{
        AbilitiesMemo, 
    },
    attributes::{Attributes},
    craft::CraftMemo,
    exaltation::{
        ExaltationMemo,
    },
    health::{Health},
    willpower::Willpower,
};

/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterMemo {
    name: String,
    concept: Option<String>,
    exalt_state: ExaltationMemo,
    willpower: Willpower,
    health: Health,
    attributes: Attributes,
    abilities: AbilitiesMemo,
    craft: CraftMemo,
}