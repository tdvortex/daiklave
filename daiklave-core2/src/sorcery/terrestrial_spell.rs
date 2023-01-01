use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::spell::Spell;

/// A Spell of the first (Terrestrial) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrestrialSpell(Spell);

impl TerrestrialSpell {
    /// Wraps a Spell as a TerrestrialSpell
    pub fn from_spell(spell: Spell) -> Self {
        Self(spell)
    }
}

impl Deref for TerrestrialSpell {
    type Target = Spell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}