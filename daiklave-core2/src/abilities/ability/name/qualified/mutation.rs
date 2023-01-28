use crate::{craft::CraftName, martial_arts::style::MartialArtsStyleName, abilities::ability::name::AbilityNameVanilla};

use super::AbilityNameQualified;

pub enum AbilityNameQualifiedMutation {
    Vanilla(AbilityNameVanilla),
    Craft(CraftName),
    MartialArts(MartialArtsStyleName),
}

impl From<AbilityNameQualified<'_>> for AbilityNameQualifiedMutation {
    fn from(name: AbilityNameQualified<'_>) -> Self {
        match name {
            AbilityNameQualified::Vanilla(vanilla) => vanilla.into(),
            AbilityNameQualified::Craft(craft_name) => AbilityNameQualifiedMutation::Craft(craft_name.into()),
            AbilityNameQualified::MartialArts(style_name) => AbilityNameQualifiedMutation::MartialArts(style_name.into()),
        }
    }
}

impl From<AbilityNameVanilla> for AbilityNameQualifiedMutation {
    fn from(vanilla: AbilityNameVanilla) -> Self {
        Self::Vanilla(vanilla)
    }
}