use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::inner::ArtifactWeaponInner,
};

mod memo;
pub use memo::OneHandedArtifactWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OneHandedArtifactWeaponView<'source>(pub(crate) ArtifactWeaponInner<'source>);

impl<'source> Deref for OneHandedArtifactWeaponView<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> OneHandedArtifactWeaponView<'source> {
    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
