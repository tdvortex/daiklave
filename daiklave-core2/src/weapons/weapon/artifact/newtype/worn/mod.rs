mod memo;
pub(crate) use memo::WornArtifactWeaponMemo;

use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::inner::ArtifactWeaponInner,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WornArtifactWeaponView<'source>(pub(crate) ArtifactWeaponInner<'source>);

impl<'source> Deref for WornArtifactWeaponView<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> From<&'source WornArtifactWeaponMemo> for WornArtifactWeaponView<'source> {
    fn from(memo: &'source WornArtifactWeaponMemo) -> Self {
        Self((&memo.0).into())
    }
}

impl<'source> WornArtifactWeaponView<'source> {
    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
