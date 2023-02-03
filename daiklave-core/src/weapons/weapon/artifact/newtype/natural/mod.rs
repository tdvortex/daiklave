use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::inner::ArtifactWeaponInner,
};

mod memo;
pub(crate) use memo::NaturalArtifactWeaponMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NaturalArtifactWeaponView<'source>(pub(crate) ArtifactWeaponInner<'source>);

impl<'source> Deref for NaturalArtifactWeaponView<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> From<&'source NaturalArtifactWeaponMemo> for NaturalArtifactWeaponView<'source> {
    fn from(memo: &'source NaturalArtifactWeaponMemo) -> Self {
        Self((&memo.0).into())
    }
}

impl<'source> NaturalArtifactWeaponView<'source> {
    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
