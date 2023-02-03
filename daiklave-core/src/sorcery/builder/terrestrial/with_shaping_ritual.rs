use crate::sorcery::{
    archetype::SorceryArchetypeName,
    circles::terrestrial::AddTerrestrialSpell,
    spell::{builder::SpellBuilder, SpellName},
    AddTerrestrialSorcery, ShapingRitualDetails, SorceryArchetypeDetails,
};

use super::control_spell_builder::TerrestrialControlSpellBuilder;

/// A Terrestrial sorcery builder after the shaping ritual has been specified.
pub struct TerrestrialSorceryBuilderWithShapingRitual {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
}

impl TerrestrialSorceryBuilderWithShapingRitual {
    /// Starts building a new control spell for the circle initiation with the 
    /// given name.
    pub fn control_spell_name(self, name: impl Into<SpellName>) -> TerrestrialControlSpellBuilder {
        TerrestrialControlSpellBuilder {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_summary: self.shaping_ritual_summary,
            shaping_ritual: self.shaping_ritual,
            spell_builder: SpellBuilder::name(name.into()),
        }
    }

    /// Sets the control spell for the circle and ends the builder.
    pub fn control_spell(self, control_spell: AddTerrestrialSpell) -> AddTerrestrialSorcery {
        AddTerrestrialSorcery {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_summary: self.shaping_ritual_summary,
            shaping_ritual: self.shaping_ritual,
            control_spell_name: control_spell.name,
            control_spell: control_spell.spell,
        }
    }
}
