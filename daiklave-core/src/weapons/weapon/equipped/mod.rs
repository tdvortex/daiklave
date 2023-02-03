mod one_handed;
mod two_handed;

pub use one_handed::EquipHand;
pub(crate) use one_handed::{
    EquippedOneHandedWeapon, EquippedOneHandedWeaponMemo, EquippedOneHandedWeaponNoAttunement,
    EquippedOneHandedWeaponNoAttunementMemo,
};
pub(crate) use two_handed::{
    EquippedTwoHandedWeapon, EquippedTwoHandedWeaponMemo, EquippedTwoHandedWeaponNoAttunement,
    EquippedTwoHandedWeaponNoAttunementMemo,
};

/// The position of an equipped weapon.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Equipped {
    /// Natural weapons are always equipped.
    Natural,
    /// Worn weapons may be equipped without using a hand.
    Worn,
    /// One-handed weapons may be wielded in the main hand.
    MainHand,
    /// One-handed weapons may be wielded in the off hand.
    OffHand,
    /// Two-handed weapons require two hands to wield.
    TwoHanded,
}
