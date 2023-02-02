use crate::sorcery::spell::SpellName;

use super::CelestialSpell;

pub struct AddCelestialSpell {
    pub(crate) name: SpellName,
    pub(crate) spell: CelestialSpell,
}
