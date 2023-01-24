use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilitiesMemo,
    attributes::Attributes,
    book_reference::BookReference,
    craft::CraftMemo,
    exaltation::ExaltationMemo,
    experience::ExperiencePool,
    health::Health,
    hearthstones::{hearthstone::GeomancyLevel, HearthstoneId, UnslottedHearthstoneMemo},
    intimacies::intimacy::IntimacyId,
    intimacies::intimacy::IntimacyInnerMemo,
    languages::LanguagesMemo,
    merits::merit::{NonStackableMerit, NonStackableMeritId, StackableMerit, StackableMeritId},
    unique_id::UniqueId,
    willpower::Willpower,
    Character,
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
    pub(crate) flaws: HashMap<String, (Option<BookReference>, String)>,
    pub(crate) languages: LanguagesMemo,
    pub(crate) intimacies: HashMap<IntimacyId, IntimacyInnerMemo>,
    pub(crate) experience: ExperiencePool,
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
            demenses_no_manse: self
                .demenses_no_manse
                .iter()
                .map(|(k, (s, g))| (*k, (s.as_str(), *g)))
                .collect(),
            nonstackable_merits: self
                .nonstackable_merits
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            stackable_merits: self
                .stackable_merits
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            flaws: self
                .flaws
                .iter()
                .map(|(name, (book_reference, description))| {
                    (name.as_str(), (*book_reference, description.as_str()))
                })
                .collect(),
            languages: self.languages.as_ref(),
            intimacies: self
                .intimacies
                .iter()
                .map(|(id, memo)| (*id, memo.as_ref()))
                .collect(),
            experience: self.experience,
        }
    }
}
