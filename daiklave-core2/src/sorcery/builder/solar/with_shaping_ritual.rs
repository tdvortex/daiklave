use crate::sorcery::{
    archetype::{ShapingRitualSummary, SorceryArchetypeName},
    circles::solar::AddSolarSpell,
    AddSolarSorcery, AddSorcery, ShapingRitual, SorceryArchetype,
};

/// A Solar sorcery builder after the shaping ritual has been specified.
pub struct SolarSorceryBuilderWithShapingRitual {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: Option<SorceryArchetype>,
    pub(crate) shaping_ritual_name: ShapingRitualSummary,
    pub(crate) shaping_ritual: ShapingRitual,
}

impl SolarSorceryBuilderWithShapingRitual {
    /// Sets the control spell for the circle and ends the builder.
    pub fn control_spell(self, control_spell: AddSolarSpell) -> AddSorcery {
        AddSorcery::Solar(AddSolarSorcery {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_name: self.shaping_ritual_name,
            shaping_ritual: self.shaping_ritual,
            control_spell_name: control_spell.0,
            control_spell: control_spell.1,
        })
    }
}
