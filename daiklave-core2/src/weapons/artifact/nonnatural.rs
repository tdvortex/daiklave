use super::{worn::WornArtifactWeapon, one_handed::OneHandedArtifactWeapon, two_handed::TwoHandedArtifactWeapon};

pub(in crate::weapons) struct NonnaturalArtifactWeapon<'source>(NonnaturalArtifactWeaponNoAttunement<'source>, Option<u8>);

pub(in crate::weapons) enum NonnaturalArtifactWeaponNoAttunement<'source> {
    Worn(WornArtifactWeapon<'source>),
    OneHanded(OneHandedArtifactWeapon<'source>),
    TwoHanded(TwoHandedArtifactWeapon<'source>),
}