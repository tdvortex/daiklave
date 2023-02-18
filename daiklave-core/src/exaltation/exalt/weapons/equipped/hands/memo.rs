use serde::{Deserialize, Serialize};

use crate::weapons::weapon::equipped::{EquippedOneHandedWeaponMemo, EquippedTwoHandedWeaponMemo};

use super::ExaltHands;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponMemo),
    OffHand(EquippedOneHandedWeaponMemo),
    Both([EquippedOneHandedWeaponMemo; 2]),
    TwoHanded(EquippedTwoHandedWeaponMemo),
}

impl From<&ExaltHands<'_>> for ExaltHandsMemo {
    fn from(value: &ExaltHands<'_>) -> Self {
        match value {
            ExaltHands::Empty => Self::Empty,
            ExaltHands::MainHand(weapon) => Self::MainHand(weapon.into()),
            ExaltHands::OffHand(weapon) => Self::OffHand(weapon.into()),
            ExaltHands::Both(weapons) => Self::Both([(&weapons[0]).into(), (&weapons[1]).into()]),
            ExaltHands::TwoHanded(weapon) => Self::TwoHanded(weapon.into()),
        }
    }
}