use crate::sorcery::{
    archetype::{ShapingRitualSummary, SorceryArchetypeName},
    circles::celestial::AddCelestialSpell,
    AddCelestialSorcery, AddSorcery, ShapingRitual, SorceryArchetype,
};

/// A Celestial sorcery builder after the shaping ritual has been specified.
pub struct CelestialSorceryBuilderWithShapingRitual {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: Option<SorceryArchetype>,
    pub(crate) shaping_ritual_name: ShapingRitualSummary,
    pub(crate) shaping_ritual: ShapingRitual,
}

impl CelestialSorceryBuilderWithShapingRitual {
    /// Sets the control spell for this circle, and completes the builder.
    pub fn control_spell(self, control_spell: AddCelestialSpell) -> AddSorcery {
        AddSorcery::Celestial(AddCelestialSorcery {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_name: self.shaping_ritual_name,
            shaping_ritual: self.shaping_ritual,
            control_spell_name: control_spell.0,
            control_spell: control_spell.1,
        })
    }
}
