mod memo;
use std::ops::Deref;

pub(crate) use memo::NonnaturalArtifactWeaponNoAttunementMemo;

use crate::weapons::weapon::artifact::{
    named::NamedArtifactWeapon,
    newtype::{OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunement<'source> {
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

impl<'source> NonnaturalArtifactWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> NonnaturalArtifactWeaponNoAttunementMemo {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(view) => {
                NonnaturalArtifactWeaponNoAttunementMemo::Worn(view.as_memo())
            }
            NonnaturalArtifactWeaponNoAttunement::OneHanded(view) => {
                NonnaturalArtifactWeaponNoAttunementMemo::OneHanded(view.as_memo())
            }
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(view) => {
                NonnaturalArtifactWeaponNoAttunementMemo::TwoHanded(view.as_memo())
            }
        }
    }
}
