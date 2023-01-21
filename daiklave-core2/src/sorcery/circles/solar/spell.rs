use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::sorcery::spell::SpellInner;

/// A Spell of the third (Solar) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarSpell(SpellInner);

impl Deref for SolarSpell {
    type Target = SpellInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SpellInner> for SolarSpell {
    fn from(inner: SpellInner) -> Self {
        Self(inner)
    }
}
