use super::{natural::NaturalArtifactWeapon, worn::WornArtifactWeapon};

pub(in crate::weapons) struct HandlessArtifactWeapon<'source>(HandlessArtifactWeaponNoAttunement<'source>, Option<u8>);

pub(in crate::weapons) enum HandlessArtifactWeaponNoAttunement<'source> {
    Natural(NaturalArtifactWeapon<'source>),
    Worn(WornArtifactWeapon<'source>),
}