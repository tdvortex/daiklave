use crate::{charms::charm::AddCharm, CharacterMutation};

use super::{builder::EvocationBuilder, Evocation, EvocationName, EvokableName};

/// An Evocation to add to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddEvocation {
    pub(crate) name: EvocationName,
    pub(crate) evocation: Evocation,
}

impl AddEvocation {
    /// Starts constructing a new Evocation for the given item.
    pub fn evocation_of(evokable: EvokableName<'_>) -> EvocationBuilder {
        EvocationBuilder::evocation_of(evokable)
    }
}

impl From<AddEvocation> for CharacterMutation {
    fn from(add_evocation: AddEvocation) -> Self {
        AddCharm::from(add_evocation).into()
    }
}
