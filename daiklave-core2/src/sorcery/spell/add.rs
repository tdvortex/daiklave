use crate::{charms::charm::AddCharm, CharacterMutation};

use super::{SpellMutation, SpellName};

/// A mutation to add a spell to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSpell {
    pub(crate) name: SpellName,
    pub(crate) spell: SpellMutation,
}

impl From<AddSpell> for CharacterMutation {
    fn from(add_spell: AddSpell) -> Self {
        AddCharm::from(add_spell).into()
    }
}
