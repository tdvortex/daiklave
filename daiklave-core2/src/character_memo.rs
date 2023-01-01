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
    willpower::Willpower, CharacterView,
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

impl<'source> CharacterMemo {
    pub fn as_ref(&'source self) -> CharacterView<'source> {
        CharacterView { 
            name: self.name.as_str(),
            concept: self.concept.as_deref(),
            exalt_state: self.exalt_state.as_ref(),
            willpower: self.willpower,
            health: self.health,
            attributes: self.attributes,
            abilities: self.abilities.as_ref(),
            craft: self.craft.as_ref(),
        }
    }
}