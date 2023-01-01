use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::sorcery::Spell;

/// A Spell of the second (Celestial) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CelestialSpell(Spell);

impl CelestialSpell {
    /// Wraps a Spell as a CelestialSpell
    pub fn from_spell(spell: Spell) -> Self {
        Self(spell)
    }
}

impl Deref for CelestialSpell {
    type Target = Spell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
