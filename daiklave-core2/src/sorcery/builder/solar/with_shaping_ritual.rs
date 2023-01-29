use crate::sorcery::{
    archetype::{SorceryArchetypeName},
    circles::solar::AddSolarSpell,
    AddSolarSorcery, AddSorcery, ShapingRitual, SorceryArchetype, add::AddSorceryCircle,
};

/// A Solar sorcery builder after the shaping ritual has been specified.
pub struct SolarSorceryBuilderWithShapingRitual {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: Option<SorceryArchetype>,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitual,
}

impl SolarSorceryBuilderWithShapingRitual {
    /// Sets the control spell for the circle and ends the builder.
    pub fn control_spell(self, control_spell: AddSolarSpell) -> AddSorcery {
        AddSorcery(Box::new(AddSorceryCircle::Solar(AddSolarSorcery {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_summary: self.shaping_ritual_summary,
            shaping_ritual: self.shaping_ritual,
            control_spell_name: control_spell.name,
            control_spell: control_spell.spell,
        })))
    }
}
