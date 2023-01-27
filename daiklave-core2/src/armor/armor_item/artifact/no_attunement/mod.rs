mod memo;
pub use memo::ArtifactArmorNoAttunementMemo;

use crate::{
    armor::armor_item::base::BaseArmor, artifact::MagicMaterial, book_reference::BookReference,
    hearthstones::SlottedHearthstone,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactArmorNoAttunement<'source> {
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<&'source str>,
    pub(crate) powers: Option<&'source str>,
    pub(crate) base_armor_name: &'source str,
    pub(crate) base_armor: &'source BaseArmor,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
    pub(crate) hearthstone_slots: Vec<Option<SlottedHearthstone<'source>>>,
}

impl<'source> ArtifactArmorNoAttunement<'source> {
    pub fn as_memo(&self) -> ArtifactArmorNoAttunementMemo {
        ArtifactArmorNoAttunementMemo {
            book_reference: self.book_reference,
            lore: self.lore.map(|s| s.to_owned()),
            powers: self.powers.map(|s| s.to_owned()),
            base_armor_name: self.base_armor_name.to_owned(),
            base_armor: self.base_armor.to_owned(),
            magic_material: self.magic_material,
            merit_dots: self.merit_dots,
            hearthstone_slots: self
                .hearthstone_slots
                .iter()
                .map(|option| option.map(|hearthstone| hearthstone.as_memo()))
                .collect(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub(crate) fn base_armor(&self) -> &'source BaseArmor {
        self.base_armor
    }

    pub fn hearthstone_slots(&self) -> u8 {
        self.hearthstone_slots.len().min(u8::MAX as usize) as u8
    }

    pub(crate) fn slotted_hearthstones(
        &self,
    ) -> impl Iterator<Item = SlottedHearthstone<'source>> + '_ {
        self.hearthstone_slots
            .iter()
            .filter_map(|maybe_hearthstone| *maybe_hearthstone)
    }

    pub(crate) fn open_slots(&self) -> u8 {
        self.hearthstone_slots
            .iter()
            .filter(|maybe_slotted| maybe_slotted.is_none())
            .count()
            .min(u8::MAX as usize) as u8
    }
}
