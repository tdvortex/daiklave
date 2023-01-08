use crate::weapons::weapon::{base::BaseWeapon, handedness::WeaponHandedness};

/// A base artifact weapon to be inserted into a full named artifact. 
pub struct BaseArtifactWeapon {
    pub(crate) handedness: WeaponHandedness,
    pub(crate) base_weapon: BaseWeapon,
}