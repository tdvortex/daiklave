use serde::{Deserialize, Serialize};

use crate::{
    artifact::MagicMaterial, book_reference::BookReference, hearthstones::SlottedHearthstoneMemo,
    weapons::weapon::base::BaseWeapon,
};

use super::ArtifactWeaponInner;

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

impl From<&ArtifactWeaponInner<'_>> for ArtifactWeaponInnerMemo {
    fn from(value: &ArtifactWeaponInner<'_>) -> Self {
        Self {
            book_reference: value.book_reference,
            merit_dots: value.merit_dots,
            magic_material: value.magic_material,
            base_weapon_name: value.base_weapon_name.into(),
            base_weapon: value.base_weapon.to_owned(),
            lore: value.lore.map(|s| s.into()),
            powers: value.powers.map(|s| s.into()),
            hearthstone_slots: value
                .hearthstone_slots
                .iter()
                .map(|maybe_hearthstone| maybe_hearthstone.as_ref().map(|slotted| slotted.into()))
                .collect(),
        }
    }
}
