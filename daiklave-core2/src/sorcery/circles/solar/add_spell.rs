use crate::sorcery::spell::SpellName;

use super::SolarSpell;

pub struct AddSolarSpell {
    pub(crate) name: SpellName,
    pub(crate) spell: SolarSpell,
}