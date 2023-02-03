use serde::{Deserialize, Serialize};

use crate::{
    artifact::MagicMaterial, book_reference::BookReference, hearthstones::SlottedHearthstoneMemo,
};

use super::WonderNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WonderNoAttunementMemo {
    pub book_reference: Option<BookReference>,
    pub lore: Option<String>,
    pub powers: String,
    pub hearthstone_slots: Vec<Option<SlottedHearthstoneMemo>>,
    pub merit_dots: u8,
    pub magic_material: Option<MagicMaterial>,
    pub attunement_cost: Option<u8>,
}

impl From<&WonderNoAttunement<'_>> for WonderNoAttunementMemo {
    fn from(view: &WonderNoAttunement<'_>) -> Self {
        Self {
            book_reference: view.book_reference,
            lore: view.lore.map(|s| s.to_owned()),
            powers: view.powers.to_owned(),
            hearthstone_slots: view
                .hearthstone_slots
                .iter()
                .map(|option| option.as_ref().map(|hearthstone| hearthstone.into()))
                .collect(),
            merit_dots: view.merit_dots,
            magic_material: view.magic_material,
            attunement_cost: view.attunement_cost,
        }
    }
}
