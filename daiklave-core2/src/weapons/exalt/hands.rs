use serde::{Serialize, Deserialize};

use crate::weapons::equipped::{EquippedOneHandedWeapon, EquippedTwoHandedWeapon, EquippedOneHandedWeaponMemo, EquippedTwoHandedWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons::exalt) enum ExaltHands<'source> {
    Empty,
    MainHand(EquippedOneHandedWeapon<'source>),
    OffHand(EquippedOneHandedWeapon<'source>),
    Both([EquippedOneHandedWeapon<'source>; 2]),
    TwoHanded(EquippedTwoHandedWeapon<'source>),
}

impl<'source> Default for ExaltHands<'source> {
    fn default() -> Self {
        ExaltHands::Empty
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::exalt) enum ExaltHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponMemo),
    OffHand(EquippedOneHandedWeaponMemo),
    Both([EquippedOneHandedWeaponMemo; 2]),
    TwoHanded(EquippedTwoHandedWeaponMemo),
}

impl<'source> ExaltHandsMemo {
    pub fn as_ref(&'source self) -> ExaltHands<'source> {
        match self {
            ExaltHandsMemo::Empty => ExaltHands::Empty,
            ExaltHandsMemo::MainHand(memo) => ExaltHands::MainHand(memo.as_ref()),
            ExaltHandsMemo::OffHand(memo) => ExaltHands::OffHand(memo.as_ref()),
            ExaltHandsMemo::Both(arr) => ExaltHands::Both(arr.map(|el| el.as_ref())),
            ExaltHandsMemo::TwoHanded(memo) => ExaltHands::TwoHanded(memo.as_ref()),
        }
    }
}