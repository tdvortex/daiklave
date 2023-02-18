use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilitiesVanillaMemo,
    attributes::Attributes,
    book_reference::BookReference,
    craft::CraftMemo,
    exaltation::ExaltationMemo,
    experience::ExperiencePool,
    flaws::flaw::FlawName,
    health::Health,
    hearthstones::{
        hearthstone::{GeomancyLevel, HearthstoneName},
        UnslottedHearthstoneMemo,
    },
    intimacies::intimacy::{IntimacyLevel, IntimacyTypeMemo},
    languages::language::LanguageMutation,
    merits::merit::{
        template::{NonStackableMeritName, StackableMeritTemplateName},
        DemenseName, NonStackableMeritInstance, StackableMeritInstance,
    },
    willpower::Willpower,
    Character, CharacterMutation, CharacterMutationError,
};

/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterMemo {
    /// The name of the character.
    pub name: String,
    pub(crate) concept: Option<String>,
    pub(crate) exaltation: ExaltationMemo,
    pub(crate) willpower: Willpower,
    pub(crate) health: Health,
    pub(crate) attributes: Attributes,
    pub(crate) abilities: AbilitiesVanillaMemo,
    pub(crate) craft: CraftMemo,
    pub(crate) hearthstone_inventory: HashMap<HearthstoneName, UnslottedHearthstoneMemo>,
    pub(crate) demenses_no_manse: HashMap<DemenseName, GeomancyLevel>,
    pub(crate) nonstackable_merits: HashMap<NonStackableMeritName, NonStackableMeritInstance>,
    pub(crate) stackable_merits:
        HashMap<(StackableMeritTemplateName, String), StackableMeritInstance>,
    pub(crate) flaws: HashMap<FlawName, (Option<BookReference>, String)>,
    pub(crate) native_language: LanguageMutation,
    pub(crate) other_languages: HashSet<LanguageMutation>,
    pub(crate) intimacies: HashMap<IntimacyTypeMemo, IntimacyLevel>,
    pub(crate) experience: ExperiencePool,
}

impl From<Character<'_>> for CharacterMemo {
    fn from(character: Character<'_>) -> Self {
        Self {
            name: character.name.to_owned(),
            concept: character.concept.map(|s| s.to_owned()),
            exaltation: character.exaltation.into(),
            willpower: character.willpower,
            health: character.health,
            attributes: character.attributes,
            abilities: (&character.abilities).into(),
            craft: character.craft.into(),
            hearthstone_inventory: character
                .hearthstone_inventory
                .into_iter()
                .map(|(name, unslotted)| (name.into(), unslotted.into()))
                .collect(),
            demenses_no_manse: character
                .demenses_no_manse
                .into_iter()
                .map(|(name, level)| (name.into(), level))
                .collect(),
            nonstackable_merits: character
                .nonstackable_merits
                .iter()
                .map(|(name, &instance)| ((*name).into(), instance.to_owned()))
                .collect(),
            stackable_merits: character
                .stackable_merits
                .iter()
                .map(|((template_name, detail), &instance)| {
                    (((*template_name).into(), (*detail).into()), instance.to_owned())
                })
                .collect(),
            flaws: character
                .flaws
                .into_iter()
                .map(|(name, (maybe_book_reference, description))| {
                    (name.into(), (maybe_book_reference, description.into()))
                })
                .collect(),
            native_language: character.native_language.to_owned(),
            other_languages: character
                .other_languages
                .into_iter()
                .map(|language| language.to_owned())
                .collect(),
            intimacies: character
                .intimacies
                .iter()
                .map(|(&intimacy_type, &level)| (intimacy_type.to_owned(), level))
                .collect(),
            experience: character.experience,
        }
    }
}

impl CharacterMemo {
    /// Applies a specific CharacterMutation or returns an error. Note that
    /// this operation is applied immutably, returning a cloned and updated
    /// version of the character.
    pub fn apply_mutation(&self, mutation: &CharacterMutation) -> Result<CharacterMemo, CharacterMutationError> {
        let mut character_view: Character = self.into();
        character_view.apply_mutation(mutation)?;
        Ok(character_view.into())
    }
}