use crate::{CharacterMutation, charms::charm::AddCharm, martial_arts::style::MartialArtsStyleName};

use super::{MartialArtsCharmDetails, MartialArtsCharmName, builder::MartialArtsCharmBuilderWithDescription};

/// A Martial Arts charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMartialArtsCharm {
    name: MartialArtsCharmName,
    style: MartialArtsStyleName,
    charm: MartialArtsCharmDetails,
}

impl From<MartialArtsCharmBuilderWithDescription> for AddMartialArtsCharm {
    fn from(builder: MartialArtsCharmBuilderWithDescription) -> Self {
        builder.build()
    }
}

impl From<AddMartialArtsCharm> for CharacterMutation {
    fn from(add_ma_charm: AddMartialArtsCharm) -> Self {
        AddCharm::from(add_ma_charm).into()
    }
}