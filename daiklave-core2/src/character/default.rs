use crate::Character;

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
            languages: Default::default(),
            intimacies: Default::default(),
        }
    }
}
