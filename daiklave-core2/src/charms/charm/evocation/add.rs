use crate::{CharacterMutation, charms::charm::AddCharm};

use super::{Evocation, EvocationName};

/// An Evocation to add to a character.
pub struct AddEvocation {
    name: EvocationName,
    evocation: Evocation,
}

impl From<AddEvocation> for CharacterMutation {
    fn from(add_evocation: AddEvocation) -> Self {
        AddCharm::from(add_evocation).into()
    }
}