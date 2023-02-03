use std::num::NonZeroU8;

use crate::{
    book_reference::BookReference,
    sorcery::{
        circles::terrestrial::AddTerrestrialSpell,
        spell::{builder::SpellBuilderWithDescription, SpellKeyword},
        AddTerrestrialSorcery, ShapingRitualDetails, SorceryArchetypeDetails, SorceryArchetypeName,
    },
    CharacterMutation,
};

/// A builder to construct a new spell to be the Control Spell for a
/// Terrestrial Circle initiation, after the spell description has been
/// specified.
pub struct TerrestrialControlSpellBuilderWithDescription {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
    pub(crate) spell_builder: SpellBuilderWithDescription,
}

impl TerrestrialControlSpellBuilderWithDescription {
    /// Sets the book reference for this Spell.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.spell_builder = self.spell_builder.book_reference(book_reference);
        self
    }

    /// Provides a short summary of the Spell.
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.spell_builder = self.spell_builder.summary(summary);
        self
    }

    /// Adds a keyword to the Spell.
    pub fn keyword(mut self, keyword: SpellKeyword) -> Self {
        self.spell_builder = self.spell_builder.keyword(keyword);
        self
    }

    /// Describes the control spell bonus of the Spell, if any.
    pub fn control_spell_description(mut self, description: impl Into<String>) -> Self {
        self.spell_builder = self.spell_builder.control_spell_description(description);
        self
    }

    /// Describes the methods opposing sorcerers may use to distort this spell.
    pub fn distortion(mut self, goal_number: NonZeroU8, description: impl Into<String>) -> Self {
        self.spell_builder = self.spell_builder.distortion(goal_number, description);
        self
    }

    /// Completes the builder for both the spell, and the sorcery circle.
    pub fn build(self) -> AddTerrestrialSorcery {
        let AddTerrestrialSpell { name, spell } = self.spell_builder.terrestrial();

        AddTerrestrialSorcery {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_summary: self.shaping_ritual_summary,
            shaping_ritual: self.shaping_ritual,
            control_spell_name: name,
            control_spell: spell,
        }
    }
}

impl From<TerrestrialControlSpellBuilderWithDescription> for CharacterMutation {
    fn from(builder: TerrestrialControlSpellBuilderWithDescription) -> Self {
        builder.build().into()
    }
}
