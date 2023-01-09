use crate::weapons::weapon::equipped::EquipHand;

use super::newtype::{
    NaturalMundaneWeapon, OneHandedMundaneWeapon, TwoHandedMundaneWeapon, WornMundaneWeapon,
};

/// An nonmagical, nonunique weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MundaneWeapon(pub(crate) MundaneWeaponHandedness);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MundaneWeaponHandedness {
    Natural(NaturalMundaneWeapon),
    Worn(WornMundaneWeapon, bool),
    OneHanded(OneHandedMundaneWeapon, Option<EquipHand>),
    TwoHanded(TwoHandedMundaneWeapon, bool),
}
