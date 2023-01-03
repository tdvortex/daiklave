mod one_handed;
mod two_handed;

pub(in crate::weapons) use one_handed::{EquippedOneHandedWeapon, EquippedOneHandedWeaponNoAttunement, EquippedOneHandedWeaponMemo, EquippedOneHandedWeaponNoAttunementMemo};
pub(in crate::weapons) use two_handed::{EquippedTwoHandedWeapon, EquippedTwoHandedWeaponNoAttunement, EquippedTwoHandedWeaponMemo, EquippedTwoHandedWeaponNoAttunementMemo};