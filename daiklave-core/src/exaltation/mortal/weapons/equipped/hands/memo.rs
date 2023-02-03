use serde::{Deserialize, Serialize};

use crate::weapons::weapon::equipped::{
    EquippedOneHandedWeaponNoAttunementMemo, EquippedTwoHandedWeaponNoAttunementMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MortalHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponNoAttunementMemo),
    OffHand(EquippedOneHandedWeaponNoAttunementMemo),
    Both(Box<[EquippedOneHandedWeaponNoAttunementMemo; 2]>),
    TwoHanded(EquippedTwoHandedWeaponNoAttunementMemo),
}
