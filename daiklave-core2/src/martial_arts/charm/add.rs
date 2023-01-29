use crate::{CharacterMutation, charms::charm::AddCharm};

use super::{MartialArtsCharm, MartialArtsCharmName};

/// A Martial Arts charm to be added to a character.
pub struct AddMartialArtsCharm {
    name: MartialArtsCharmName,
    charm: MartialArtsCharm,
}

impl From<AddMartialArtsCharm> for CharacterMutation {
    fn from(add_ma_charm: AddMartialArtsCharm) -> Self {
        AddCharm::from(add_ma_charm).into()
    }
}