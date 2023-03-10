use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone,
    weapons::weapon::artifact::{
        inner::ArtifactWeaponInner,
        newtype::{NaturalArtifactWeaponView, WornArtifactWeaponView},
    },
};

mod memo;
pub(crate) use memo::HandlessArtifactWeaponNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HandlessArtifactWeaponNoAttunement<'source> {
    Natural(NaturalArtifactWeaponView<'source>),
    Worn(WornArtifactWeaponView<'source>),
}

impl<'source> HandlessArtifactWeaponNoAttunement<'source> {
    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        match self {
            HandlessArtifactWeaponNoAttunement::Natural(view) => view.hearthstone_slots_mut(),
            HandlessArtifactWeaponNoAttunement::Worn(view) => view.hearthstone_slots_mut(),
        }
    }
}

impl<'source> Deref for HandlessArtifactWeaponNoAttunement<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            HandlessArtifactWeaponNoAttunement::Natural(deref) => deref,
            HandlessArtifactWeaponNoAttunement::Worn(deref) => deref,
        }
    }
}

impl<'source> From<&'source HandlessArtifactWeaponNoAttunementMemo> for HandlessArtifactWeaponNoAttunement<'source> {
    fn from(value: &'source HandlessArtifactWeaponNoAttunementMemo) -> Self {
        match value {
            HandlessArtifactWeaponNoAttunementMemo::Natural(weapon) => Self::Natural(weapon.into()),
            HandlessArtifactWeaponNoAttunementMemo::Worn(weapon) => Self::Worn(weapon.into()),
        }
    }
}