use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::TwoHandedMundaneWeapon, artifact::TwoHandedArtifactWeapon};

pub(in crate::weapons) enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>),
}

pub(in crate::weapons) enum EquippedTwoHandedWeapon<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>, Option<u8>),
}
