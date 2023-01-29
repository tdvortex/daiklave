use crate::{CharacterMutation, charms::charm::AddCharm};

use super::{MartialArtsCharmDetails, MartialArtsCharmName};

/// A Martial Arts charm to be added to a character.
pub struct AddMartialArtsCharm {
    name: MartialArtsCharmName,
    charm: MartialArtsCharmDetails,
}

impl From<AddMartialArtsCharm> for CharacterMutation {
    fn from(add_ma_charm: AddMartialArtsCharm) -> Self {
        AddCharm::from(add_ma_charm).into()
    }
}