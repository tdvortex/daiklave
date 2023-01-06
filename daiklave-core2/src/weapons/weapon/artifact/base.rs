use crate::weapons::weapon::base::BaseWeapon;

/// A base artifact weapon to be inserted into a character. This
/// wraps the BaseWeaponMemo struct with its wielding characteristics.
pub enum BaseArtifactWeapon {
    /// A Natural base artifact weapon (uncommon).
    Natural(BaseWeapon),
    /// A Worn base artifact weapon like Smashfists.
    Worn(BaseWeapon),
    /// A One-Handed base artifact weapon.
    OneHanded(BaseWeapon),
    /// A Two-Handed base artifact weapon.
    TwoHanded(BaseWeapon),
}
