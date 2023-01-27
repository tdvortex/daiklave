use serde::{Deserialize, Serialize};

use crate::{
    armor::armor_item::base::BaseArmor, artifact::MagicMaterial, book_reference::BookReference,
    hearthstones::SlottedHearthstoneMemo,
};

use super::ArtifactArmorNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactArmorNoAttunementMemo {
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) base_armor_name: String,
    pub(crate) base_armor: BaseArmor,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
    pub(crate) hearthstone_slots: Vec<Option<SlottedHearthstoneMemo>>,
}

impl<'source> ArtifactArmorNoAttunementMemo {
    pub fn as_ref(&'source self) -> ArtifactArmorNoAttunement<'source> {
        ArtifactArmorNoAttunement {
            book_reference: self.book_reference,
            lore: self.lore.as_deref(),
            powers: self.powers.as_deref(),
            base_armor_name: self.base_armor_name.as_str(),
            base_armor: &self.base_armor,
            magic_material: self.magic_material,
            merit_dots: self.merit_dots,
            hearthstone_slots: self
                .hearthstone_slots
                .iter()
                .map(|option| option.as_ref().map(|hearthstone| hearthstone.as_ref()))
                .collect(),
        }
    }
}
