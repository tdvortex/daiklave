use serde::{Deserialize, Serialize};

use crate::{
    artifact::MagicMaterial, book_reference::BookReference, hearthstones::SlottedHearthstoneMemo,
    weapons::weapon::base::BaseWeapon,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactWeaponInnerMemo {
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_dots: u8,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) base_weapon_name: String,
    pub(crate) base_weapon: BaseWeapon,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) hearthstone_slots: Vec<Option<SlottedHearthstoneMemo>>,
}
