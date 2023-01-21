use std::collections::HashSet;

use crate::{charms::charm::CharmMutation, book_reference::BookReference, sorcery::{spell::{cost::SpellCost, Spell, SpellKeyword}, SorceryCircle, TerrestrialSpell, CelestialSpell, SolarSpell}};

pub struct SpellBuilderWithDescription {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) circle: SorceryCircle,
    pub(crate) cost: SpellCost,
    pub(crate) duration: String,
    pub(crate) description: String,
}

impl SpellBuilderWithDescription {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn build(self) -> CharmMutation {
        let spell = Spell {
            name: self.name,
            summary: self.summary,
            cost: self.cost,
            duration: self.duration,
            description: self.description,
            book_reference: self.book_reference,
            keywords: self.keywords,
        };

        match self.circle {
            SorceryCircle::Terrestrial => CharmMutation::TerrestrialSpell(TerrestrialSpell::from_spell(spell)),
            SorceryCircle::Celestial => CharmMutation::CelestialSpell(CelestialSpell::from_spell(spell)),
            SorceryCircle::Solar => CharmMutation::SolarSpell(SolarSpell::from_spell(spell)),
        }
    }
}