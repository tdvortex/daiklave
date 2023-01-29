use super::{ArtifactWeaponName, handedness::ArtifactWeaponHandedness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddArtifactWeapon {
    pub(crate) name: ArtifactWeaponName,
    pub(crate) handedness: ArtifactWeaponHandedness
}