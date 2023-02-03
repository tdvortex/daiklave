mod memo;
pub(crate) use memo::TwoHandedArtifactWeaponMemo;

use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::inner::ArtifactWeaponInner,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TwoHandedArtifactWeaponView<'source>(pub(crate) ArtifactWeaponInner<'source>);

impl<'source> From<&'source TwoHandedArtifactWeaponMemo> for TwoHandedArtifactWeaponView<'source> {
    fn from(memo: &'source TwoHandedArtifactWeaponMemo) -> Self {
        Self((&memo.0).into())
    }
}

impl<'source> Deref for TwoHandedArtifactWeaponView<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> TwoHandedArtifactWeaponView<'source> {
    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
