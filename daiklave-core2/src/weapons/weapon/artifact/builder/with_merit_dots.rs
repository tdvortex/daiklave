use crate::{
    artifact::MagicMaterial,
    book_reference::BookReference,
    weapons::weapon::{artifact::base::BaseArtifactWeapon, BaseWeaponId},
};

use super::with_heartstone_slots::ArtifactWeaponBuilderWithHearthstoneSlots;

/// An artifact builder after the number of merit dots is specified.
/// The next step is .hearthstone_slots().
pub struct ArtifactWeaponBuilderWithMeritDots {
    pub(crate) name: String,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) base_weapon_id: BaseWeaponId,
    pub(crate) base_weapon: BaseArtifactWeapon,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
}

impl ArtifactWeaponBuilderWithMeritDots {
    /// Add flavor text to describe the weapon's forging, history, and prior
    /// wielders.
    pub fn lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    /// Add passive or unique magical effects that are not Evocations, such as
    /// Beloved Adorei's emotional bond to her wielder.
    pub fn powers(mut self, powers: String) -> Self {
        self.powers = Some(powers);
        self
    }

    /// Add a book reference for the weapon. Note that this is a reference for
    /// the named instance of the artifact and not the base weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Puts a number of (empty) hearthstone slots into the weapon.
    pub fn hearthstone_slots(self, slots: usize) -> ArtifactWeaponBuilderWithHearthstoneSlots {
        ArtifactWeaponBuilderWithHearthstoneSlots {
            name: self.name,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon,
            magic_material: self.magic_material,
            merit_dots: self.merit_dots,
            hearthstone_slots: slots,
            lore: None,
            powers: None,
            book_reference: None,
        }
    }
}
