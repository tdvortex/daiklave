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

impl From<&ArtifactArmorNoAttunement<'_>> for ArtifactArmorNoAttunementMemo {
    fn from(view: &ArtifactArmorNoAttunement<'_>) -> Self {
        Self {
            book_reference: view.book_reference,
            lore: view.lore.map(|s| s.to_owned()),
            powers: view.powers.map(|s| s.to_owned()),
            base_armor_name: view.base_armor_name.to_owned(),
            base_armor: view.base_armor.to_owned(),
            magic_material: view.magic_material,
            merit_dots: view.merit_dots,
            hearthstone_slots: view
                .hearthstone_slots
                .iter()
                .map(|maybe_hearthstone| {
                    maybe_hearthstone
                        .as_ref()
                        .map(|hearthstone| hearthstone.into())
                })
                .collect(),
        }
    }
}
