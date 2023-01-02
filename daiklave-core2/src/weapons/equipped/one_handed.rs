use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::OneHandedMundaneWeapon, artifact::OneHandedArtifactWeapon};

pub(in crate::weapons) enum EquippedOneHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>),
}

pub(in crate::weapons) enum EquippedOneHandedWeapon<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>, Option<u8>),
}