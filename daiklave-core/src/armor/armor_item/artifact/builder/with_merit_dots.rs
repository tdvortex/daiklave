use crate::{
    armor::armor_item::artifact::{ArtifactArmorName, BaseArtifactArmor},
    artifact::MagicMaterial,
    book_reference::BookReference,
};

use super::with_hearthstone_slots::ArtifactArmorItemBuilderWithHearthstoneSlots;

/// An artifact armor item builder after merit dots have been specified.
pub struct ArtifactArmorItemBuilderWithMeritDots {
    pub(crate) name: ArtifactArmorName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) base_armor_name: String,
    pub(crate) base_armor: BaseArtifactArmor,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
}

impl ArtifactArmorItemBuilderWithMeritDots {
    /// The book and page number where the named artifact armor is listed.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Flavor text about the artifact's history, prior wearers, etc.
    pub fn lore(mut self, lore: &str) -> Self {
        self.lore = Some(lore.to_owned());
        self
    }

    /// Persistent powers that the bearer gets for free.
    pub fn powers(mut self, powers: &str) -> Self {
        self.powers = Some(powers.to_owned());
        self
    }

    /// Sets the number of hearthstone slots in the artifact.
    pub fn hearthstone_slots(
        self,
        hearthstone_slots: u8,
    ) -> ArtifactArmorItemBuilderWithHearthstoneSlots {
        ArtifactArmorItemBuilderWithHearthstoneSlots {
            name: self.name,
            book_reference: self.book_reference,
            lore: self.lore,
            powers: self.powers,
            base_armor_name: self.base_armor_name,
            base_armor: self.base_armor,
            magic_material: self.magic_material,
            merit_dots: self.merit_dots,
            hearthstone_slots,
        }
    }
}
