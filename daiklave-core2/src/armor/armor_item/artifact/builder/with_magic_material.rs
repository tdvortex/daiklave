use crate::{book_reference::BookReference, artifact::MagicMaterial, armor::armor_item::{artifact::BaseArtifactArmor, BaseArmorId}};

use super::with_merit_dots::ArtifactArmorItemBuilderWithMeritDots;

pub struct ArtifactArmorItemBuilderWithMagicMaterial {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) base_armor_id: BaseArmorId,
    pub(crate) base_armor: BaseArtifactArmor,
    pub(crate) magic_material: MagicMaterial,
}

impl ArtifactArmorItemBuilderWithMagicMaterial {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn lore(mut self, lore: &str) -> Self {
        self.lore = Some(lore.to_owned());
        self
    }

    pub fn powers(mut self, powers: &str) -> Self {
        self.powers = Some(powers.to_owned());
        self
    }

    pub fn merit_dots(self, merit_dots: u8) -> ArtifactArmorItemBuilderWithMeritDots {
        ArtifactArmorItemBuilderWithMeritDots {
            name: self.name,
            book_reference: self.book_reference,
            lore: self.lore,
            powers: self.powers,
            base_armor_id: self.base_armor_id,
            base_armor: self.base_armor,
            magic_material: self.magic_material,
            merit_dots,
        }
    }
}