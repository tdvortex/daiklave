use crate::{Character, languages::language::{LanguageMutation, MajorLanguage}};

impl<'source> Default for Character<'source> {
    fn default() -> Self {
        Self {
            name: "New Character",
            concept: Default::default(),
            exaltation: Default::default(),
            willpower: Default::default(),
            health: Default::default(),
            attributes: Default::default(),
            abilities: Default::default(),
            craft: Default::default(),
            hearthstone_inventory: Default::default(),
            demenses_no_manse: Default::default(),
            stackable_merits: Default::default(),
            nonstackable_merits: Default::default(),
            flaws: Default::default(),
            native_language: &LanguageMutation::MajorLanguage(MajorLanguage::LowRealm),
            other_languages: Default::default(),
            intimacies: Default::default(),
            experience: Default::default(),
        }
    }
}
