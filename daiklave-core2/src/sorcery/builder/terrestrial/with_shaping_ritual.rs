use crate::sorcery::{
    add::AddSorceryCircle, archetype::SorceryArchetypeName,
    circles::terrestrial::AddTerrestrialSpell, AddSorcery, AddTerrestrialSorcery,
    ShapingRitualDetails, SorceryArchetypeDetails,
};

/// A Terrestrial sorcery builder after the shaping ritual has been specified.
pub struct TerrestrialSorceryBuilderWithShapingRitual {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
}

impl TerrestrialSorceryBuilderWithShapingRitual {
    /// Sets the control spell for the circle and ends the builder.
    pub fn control_spell(self, control_spell: AddTerrestrialSpell) -> AddSorcery {
        AddSorcery(Box::new(AddSorceryCircle::Terrestrial(
            AddTerrestrialSorcery {
                archetype_name: self.archetype_name,
                archetype: self.archetype,
                shaping_ritual_summary: self.shaping_ritual_summary,
                shaping_ritual: self.shaping_ritual,
                control_spell_name: control_spell.name,
                control_spell: control_spell.spell,
            },
        )))
    }
}
