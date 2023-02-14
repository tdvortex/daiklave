use serde::{Serialize, Deserialize};

use crate::{
    abilities::ability::name::AbilityNameVanilla, craft::CraftName,
    martial_arts::style::MartialArtsStyleName,
};

use super::AbilityNameQualified;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AbilityNameQualifiedMutation {
    Vanilla(AbilityNameVanilla),
    Craft(CraftName),
    MartialArts(MartialArtsStyleName),
}

impl From<AbilityNameQualified<'_>> for AbilityNameQualifiedMutation {
    fn from(name: AbilityNameQualified<'_>) -> Self {
        match name {
            AbilityNameQualified::Vanilla(vanilla) => vanilla.into(),
            AbilityNameQualified::Craft(craft_name) => {
                AbilityNameQualifiedMutation::Craft(craft_name.into())
            }
            AbilityNameQualified::MartialArts(style_name) => {
                AbilityNameQualifiedMutation::MartialArts(style_name.into())
            }
        }
    }
}

impl From<AbilityNameVanilla> for AbilityNameQualifiedMutation {
    fn from(vanilla: AbilityNameVanilla) -> Self {
        Self::Vanilla(vanilla)
    }
}

impl From<CraftName> for AbilityNameQualifiedMutation {
    fn from(craft_name: CraftName) -> Self {
        Self::Craft(craft_name)
    }
}

impl From<MartialArtsStyleName> for AbilityNameQualifiedMutation {
    fn from(style_name: MartialArtsStyleName) -> Self {
        Self::MartialArts(style_name)
    }
}
