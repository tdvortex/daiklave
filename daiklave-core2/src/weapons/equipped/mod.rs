mod one_handed;
mod two_handed;

pub(in crate::weapons) use one_handed::{EquippedOneHandedWeapon, EquippedOneHandedWeaponNoAttunement};
pub(in crate::weapons) use two_handed::{EquippedTwoHandedWeapon, EquippedTwoHandedWeaponNoAttunement};