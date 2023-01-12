mod memo;
pub use memo::WornArtifactWeapon;

use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::named::NamedArtifactWeapon,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WornArtifactWeaponView<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for WornArtifactWeaponView<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> WornArtifactWeaponView<'source> {
    pub(crate) fn as_memo(&self) -> WornArtifactWeapon {
        WornArtifactWeapon(self.0.as_memo())
    }

    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
