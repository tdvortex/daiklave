use crate::weapons::weapon::equipped::EquipHand;

use super::newtype::{
    NaturalMundaneWeapon, OneHandedMundaneWeapon, TwoHandedMundaneWeapon, WornMundaneWeapon,
};

/// An nonmagical, nonunique weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MundaneWeapon {
    /// A Natural weapon.
    Natural(NaturalMundaneWeapon),
    /// A Worn weapon, and whether it is equipped.
    Worn(WornMundaneWeapon, bool),
    /// A OneHanded weapon, and the hand it's equipped in (if any).
    OneHanded(OneHandedMundaneWeapon, Option<EquipHand>),
    /// A TwoHanded weapon, and whether it is equipped.
    TwoHanded(TwoHandedMundaneWeapon, bool),
}
