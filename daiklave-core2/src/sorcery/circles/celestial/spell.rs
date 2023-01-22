use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::sorcery::spell::SpellInner;

/// A Spell of the second (Celestial) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CelestialSpell(SpellInner);

impl Deref for CelestialSpell {
    type Target = SpellInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SpellInner> for CelestialSpell {
    fn from(inner: SpellInner) -> Self {
        Self(inner)
    }
}
