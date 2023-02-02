use crate::{
    charms::charm::AddCharm, martial_arts::style::MartialArtsStyleName, CharacterMutation,
};

use super::{
    builder::MartialArtsCharmBuilderWithDescription, MartialArtsCharmDetails, MartialArtsCharmName,
};

/// A Martial Arts charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMartialArtsCharm {
    pub(crate) name: MartialArtsCharmName,
    pub(crate) style: MartialArtsStyleName,
    pub(crate) charm: MartialArtsCharmDetails,
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
