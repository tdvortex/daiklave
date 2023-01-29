use crate::sorcery::spell::SpellName;

use super::TerrestrialSpell;

pub struct AddTerrestrialSpell {
    pub(crate) name: SpellName,
    pub(crate) spell: TerrestrialSpell,
}