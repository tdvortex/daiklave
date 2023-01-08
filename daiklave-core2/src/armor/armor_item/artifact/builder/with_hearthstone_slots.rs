use crate::{
    armor::armor_item::{
        artifact::{
            no_attunement::ArtifactArmorNoAttunementMemo, ArtifactArmor, BaseArtifactArmor,
        },
        BaseArmorId,
    },
    artifact::MagicMaterial,
    book_reference::BookReference,
};

/// An artifact armor builder after specifying the number of hearthstone slots.
pub struct ArtifactArmorItemBuilderWithHearthstoneSlots {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) base_armor_id: BaseArmorId,
    pub(crate) base_armor: BaseArtifactArmor,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
    pub(crate) hearthstone_slots: u8,
}

impl ArtifactArmorItemBuilderWithHearthstoneSlots {
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

    /// Finalizes the construction of a piece of artifact armor.
    pub fn build(self) -> ArtifactArmor {
        ArtifactArmor(
            ArtifactArmorNoAttunementMemo {
                name: self.name,
                book_reference: self.book_reference,
                lore: self.lore,
                powers: self.powers,
                base_armor_id: self.base_armor_id,
                base_armor: self.base_armor.0,
                magic_material: self.magic_material,
                merit_dots: self.merit_dots,
                hearthstone_slots: (0..self.hearthstone_slots).fold(Vec::new(), |mut acc, _| {
                    acc.push(None);
                    acc
                }),
            },
            None,
        )
    }
}
