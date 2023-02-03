mod with_description;
mod with_duration;
mod with_mote_cost;
mod with_willpower;
pub use with_description::TerrestrialControlSpellBuilderWithDescription;
pub use with_duration::TerrestrialControlSpellBuilderWithDuration;
pub use with_mote_cost::TerrestrialControlSpellBuilderWithMoteCost;
pub use with_willpower::TerrestrialControlSpellBuilderWithWillpower;

use std::num::NonZeroU8;

use crate::{
    book_reference::BookReference,
    sorcery::{
        spell::{builder::SpellBuilder, SpellKeyword},
        ShapingRitualDetails, SorceryArchetypeDetails, SorceryArchetypeName,
    },
};

/// A builder to construct a new spell to be the Control Spell for a
/// Terrestrial Circle initiation.
pub struct TerrestrialControlSpellBuilder {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
    pub(crate) spell_builder: SpellBuilder,
}

impl TerrestrialControlSpellBuilder {
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

    /// Sets a Sorcerous Motes cost to cast this spell.
    pub fn sorcerous_motes(
        self,
        sorcerous_motes: NonZeroU8,
    ) -> TerrestrialControlSpellBuilderWithMoteCost {
        TerrestrialControlSpellBuilderWithMoteCost {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_summary: self.shaping_ritual_summary,
            shaping_ritual: self.shaping_ritual,
            spell_builder: self.spell_builder.sorcerous_motes(sorcerous_motes),
        }
    }

    /// Defines this spell as a Ritual; this costs no explicit Sorcerous Motes
    /// to use, but can only be cast outside of combat.
    pub fn ritual(self) -> TerrestrialControlSpellBuilderWithMoteCost {
        TerrestrialControlSpellBuilderWithMoteCost {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_summary: self.shaping_ritual_summary,
            shaping_ritual: self.shaping_ritual,
            spell_builder: self.spell_builder.ritual(),
        }
    }
}
