use serde::{Deserialize, Serialize};

use crate::weapons::weapon::equipped::{
    EquippedOneHandedWeaponNoAttunementMemo, EquippedTwoHandedWeaponNoAttunementMemo,
};

use super::MortalHands;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MortalHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponNoAttunementMemo),
    OffHand(EquippedOneHandedWeaponNoAttunementMemo),
    Both([EquippedOneHandedWeaponNoAttunementMemo; 2]),
    TwoHanded(EquippedTwoHandedWeaponNoAttunementMemo),
}

impl From<&MortalHands<'_>> for MortalHandsMemo {
    fn from(hands: &MortalHands<'_>) -> Self {
        match hands {
            MortalHands::Empty => Self::Empty,
            MortalHands::MainHand(weapon) => Self::MainHand(weapon.into()),
            MortalHands::OffHand(weapon) => Self::OffHand(weapon.into()),
            MortalHands::Both(weapons) => Self::Both([(&weapons[0]).into(), (&weapons[1]).into()]),
            MortalHands::TwoHanded(weapon) => Self::TwoHanded(weapon.into()),
        }
    }
}