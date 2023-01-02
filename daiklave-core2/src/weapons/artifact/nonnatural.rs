use std::ops::Deref;

use super::{worn::WornArtifactWeapon, one_handed::OneHandedArtifactWeapon, two_handed::TwoHandedArtifactWeapon, named::NamedArtifactWeapon};

pub(in crate::weapons) struct NonnaturalArtifactWeapon<'source>(NonnaturalArtifactWeaponNoAttunement<'source>, Option<u8>);

pub(in crate::weapons) enum NonnaturalArtifactWeaponNoAttunement<'source> {
    Worn(WornArtifactWeapon<'source>),
    OneHanded(OneHandedArtifactWeapon<'source>),
    TwoHanded(TwoHandedArtifactWeapon<'source>),
}

impl<'source> Deref for NonnaturalArtifactWeaponNoAttunement<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::OneHanded(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(deref) => deref,
        }
    }
}