use crate::weapons::weapon::{base::BaseWeapon, handedness::WeaponHandedness};

pub(crate) struct BaseArtifactWeapon {
    pub(crate) handedness: WeaponHandedness,
    pub(crate) base_weapon: BaseWeapon,
}

