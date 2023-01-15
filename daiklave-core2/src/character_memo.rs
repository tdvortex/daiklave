use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilitiesMemo,
    attributes::Attributes,
    craft::CraftMemo,
    exaltation::ExaltationMemo,
    health::Health,
    hearthstones::{HearthstoneId, UnslottedHearthstoneMemo, hearthstone::GeomancyLevel},
    willpower::Willpower,
    Character, unique_id::UniqueId, merits::merit::{StackableMeritId, StackableMerit, NonStackableMerit, NonStackableMeritId}, languages::LanguagesMemo,
};

/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterMemo {
    pub(crate) name: String,
    pub(crate) concept: Option<String>,
    pub(crate) exalt_state: ExaltationMemo,
    pub(crate) willpower: Willpower,
    pub(crate) health: Health,
    pub(crate) attributes: Attributes,
    pub(crate) abilities: AbilitiesMemo,
    pub(crate) craft: CraftMemo,
    pub(crate) hearthstone_inventory: HashMap<HearthstoneId, UnslottedHearthstoneMemo>,
    pub(crate) demenses_no_manse: HashMap<UniqueId, (String, GeomancyLevel)>,
    pub(crate) nonstackable_merits: HashMap<NonStackableMeritId, NonStackableMerit>,
    pub(crate) stackable_merits: HashMap<StackableMeritId, StackableMerit>,
    pub(crate) languages: LanguagesMemo,
}

impl<'source> CharacterMemo {
    /// Creates a borrowed reference to this owned object.
    pub fn as_ref(&'source self) -> Character<'source> {
        Character {
            name: self.name.as_str(),
            concept: self.concept.as_deref(),
            exaltation: self.exalt_state.as_ref(),
            willpower: self.willpower,
            health: self.health,
            attributes: self.attributes,
            abilities: self.abilities.as_ref(),
            craft: self.craft.as_ref(),
            hearthstone_inventory: self
                .hearthstone_inventory
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            demenses_no_manse: self.demenses_no_manse.iter().map(|(k, (s, g))| (*k, (s.as_str(), *g))).collect(),
            nonstackable_merits: self.nonstackable_merits.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            stackable_merits: self.stackable_merits.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            languages: self.languages.as_ref(),
        }
    }
}
