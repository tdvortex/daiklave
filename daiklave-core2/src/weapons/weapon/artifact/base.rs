use crate::weapons::weapon::base::BaseWeaponMemo;

/// A base artifact weapon to be inserted into a character. This
/// wraps the BaseWeaponMemo struct with its wielding characteristics.
pub enum BaseArtifactWeapon {
    /// A Natural base artifact weapon (uncommon).
    Natural(BaseWeaponMemo),
    /// A Worn base artifact weapon like Smashfists.
    Worn(BaseWeaponMemo),
    /// A One-Handed base artifact weapon.
    OneHanded(BaseWeaponMemo),
    /// A Two-Handed base artifact weapon.
    TwoHanded(BaseWeaponMemo),
}
