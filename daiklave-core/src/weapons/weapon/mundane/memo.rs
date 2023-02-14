use serde::{Serialize, Deserialize};

use crate::weapons::weapon::equipped::EquipHand;

use super::newtype::{
    NaturalMundaneWeapon, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeapon, WornMundaneWeapon,
};

/// An nonmagical, nonunique weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneWeapon(pub(crate) MundaneWeaponHandedness);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MundaneWeaponHandedness {
    Natural(NaturalMundaneWeapon),
    Worn(WornMundaneWeapon, bool),
    OneHanded(OneHandedMundaneWeaponMemo, Option<EquipHand>),
    TwoHanded(TwoHandedMundaneWeapon, bool),
}
