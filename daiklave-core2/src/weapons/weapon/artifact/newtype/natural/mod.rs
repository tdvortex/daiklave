use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::named::NamedArtifactWeapon,
};

mod memo;

pub use memo::NaturalArtifactWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NaturalArtifactWeaponView<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for NaturalArtifactWeaponView<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> NaturalArtifactWeaponView<'source> {
    pub(crate) fn as_memo(&self) -> NaturalArtifactWeapon {
        NaturalArtifactWeapon(self.0.as_memo())
    }

    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
