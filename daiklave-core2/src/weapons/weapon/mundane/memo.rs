use crate::weapons::weapon::equipped::EquipHand;

use super::newtype::{
    NaturalMundaneWeaponMemo, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeaponMemo,
    WornMundaneWeaponMemo,
};

/// An owned copy of a Mundane Weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MundaneWeaponMemo {
    /// A Natural weapon.
    Natural(NaturalMundaneWeaponMemo),
    /// A Worn weapon, and whether it is equipped.
    Worn(WornMundaneWeaponMemo, bool),
    /// A OneHanded weapon, and the hand it's equipped in (if any).
    OneHanded(OneHandedMundaneWeaponMemo, Option<EquipHand>),
    /// A TwoHanded weapon, and whether it is equipped.
    TwoHanded(TwoHandedMundaneWeaponMemo, bool),
}
