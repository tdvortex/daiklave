use serde::{Deserialize, Serialize};

use crate::weapons::weapon::equipped::{EquippedOneHandedWeaponMemo, EquippedTwoHandedWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponMemo),
    OffHand(EquippedOneHandedWeaponMemo),
    Both(Box<[EquippedOneHandedWeaponMemo; 2]>),
    TwoHanded(EquippedTwoHandedWeaponMemo),
}
