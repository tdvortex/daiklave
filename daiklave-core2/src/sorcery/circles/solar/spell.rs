use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::sorcery::spell::Spell;

/// A Spell of the third (Solar) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarSpell(Spell);

impl SolarSpell {
    /// Wraps a Spell as a CelestialSpell
    pub fn from_spell(spell: Spell) -> Self {
        Self(spell)
    }
}

impl Deref for SolarSpell {
    type Target = Spell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
