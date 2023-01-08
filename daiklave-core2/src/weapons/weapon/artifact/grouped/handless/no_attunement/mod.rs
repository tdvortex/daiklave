use std::ops::Deref;

use crate::weapons::weapon::artifact::{
    named::NamedArtifactWeapon,
    newtype::{NaturalArtifactWeaponView, WornArtifactWeaponView},
};

mod memo;
pub(crate) use memo::HandlessArtifactWeaponNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HandlessArtifactWeaponNoAttunement<'source> {
    Natural(NaturalArtifactWeaponView<'source>),
    Worn(WornArtifactWeaponView<'source>),
}

impl<'source> HandlessArtifactWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> HandlessArtifactWeaponNoAttunementMemo {
        match self {
            HandlessArtifactWeaponNoAttunement::Natural(view) => {
                HandlessArtifactWeaponNoAttunementMemo::Natural(view.as_memo())
            }
            HandlessArtifactWeaponNoAttunement::Worn(view) => {
                HandlessArtifactWeaponNoAttunementMemo::Worn(view.as_memo())
            }
        }
    }
}

impl<'source> Deref for HandlessArtifactWeaponNoAttunement<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            HandlessArtifactWeaponNoAttunement::Natural(deref) => deref,
            HandlessArtifactWeaponNoAttunement::Worn(deref) => deref,
        }
    }
}
