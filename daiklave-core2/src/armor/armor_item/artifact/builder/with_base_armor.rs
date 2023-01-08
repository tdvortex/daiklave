use crate::{
    armor::armor_item::{artifact::BaseArtifactArmor, BaseArmorId},
    artifact::MagicMaterial,
    book_reference::BookReference,
};

use super::with_magic_material::ArtifactArmorItemBuilderWithMagicMaterial;

/// An artifact armor builder after the base artifact armor has been
/// specified.
pub struct ArtifactArmorItemBuilderWithBaseArmor {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) base_armor_id: BaseArmorId,
    pub(crate) base_armor: BaseArtifactArmor,
}

impl ArtifactArmorItemBuilderWithBaseArmor {
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

    /// Specifies the magic material the artifact is made of.
    pub fn material(
        self,
        magic_material: MagicMaterial,
    ) -> ArtifactArmorItemBuilderWithMagicMaterial {
        ArtifactArmorItemBuilderWithMagicMaterial {
            name: self.name,
            book_reference: self.book_reference,
            lore: self.lore,
            powers: self.powers,
            base_armor_id: self.base_armor_id,
            base_armor: self.base_armor,
            magic_material,
        }
    }
}
