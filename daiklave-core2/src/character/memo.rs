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
    hearthstones::{hearthstone::{GeomancyLevel, HearthstoneName}, UnslottedHearthstoneMemo},
    intimacies::intimacy::{IntimacyLevel, IntimacyTypeMemo},
    languages::LanguagesMemo,
    merits::merit::{NonStackableMerit, StackableMerit, DemenseName, NonStackableMeritName},
    willpower::Willpower,
    flaws::flaw::FlawName,
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
    pub(crate) hearthstone_inventory: HashMap<HearthstoneName, UnslottedHearthstoneMemo>,
    pub(crate) demenses_no_manse: HashMap<DemenseName, GeomancyLevel>,
    pub(crate) nonstackable_merits: HashMap<NonStackableMeritName, NonStackableMerit>,
    pub(crate) stackable_merits: HashMap<(StackableMeritTemplateName, String), StackableMerit>,
    pub(crate) flaws: HashMap<FlawName, (Option<BookReference>, String)>,
    pub(crate) languages: LanguagesMemo,
    pub(crate) intimacies: HashMap<IntimacyTypeMemo, IntimacyLevel>,
    pub(crate) experience: ExperiencePool,
}