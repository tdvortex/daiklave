mod memo;
pub use memo::TwoHandedArtifactWeapon;

use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::traits::ArtifactWeaponTraits,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TwoHandedArtifactWeaponView<'source>(pub(crate) ArtifactWeaponTraits<'source>);

impl<'source> Deref for TwoHandedArtifactWeaponView<'source> {
    type Target = ArtifactWeaponTraits<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> TwoHandedArtifactWeaponView<'source> {
    pub(crate) fn as_memo(&self) -> TwoHandedArtifactWeapon {
        TwoHandedArtifactWeapon(self.0.as_memo())
    }

    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
