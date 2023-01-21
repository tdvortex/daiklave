use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::sorcery::spell::SpellInner;

/// A Spell of the first (Terrestrial) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrestrialSpell(SpellInner);

impl Deref for TerrestrialSpell {
    type Target = SpellInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl From<SpellInner> for TerrestrialSpell {
    fn from(inner: SpellInner) -> Self {
        Self(inner)
    }
}