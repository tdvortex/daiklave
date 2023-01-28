mod circle;
use crate::CharacterMutation;

pub use self::circle::AddSorceryCircle;

use super::{builder::SorceryBuilder};

/// A mutation to add Sorcery to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSorcery(pub Box<AddSorceryCircle>);

impl AddSorcery {
    pub fn builder() -> SorceryBuilder {
        SorceryBuilder
    }
}

impl From<AddSorcery> for CharacterMutation {
    fn from(add_sorcery: AddSorcery) -> Self {
        Self::AddSorcery(add_sorcery)
    }
}
