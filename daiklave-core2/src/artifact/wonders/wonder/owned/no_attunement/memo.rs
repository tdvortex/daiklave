use serde::{Deserialize, Serialize};

use crate::{
    artifact::MagicMaterial, book_reference::BookReference, hearthstones::OwnedHearthstoneMemo,
};

use super::WonderNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WonderNoAttunementMemo {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub lore: Option<String>,
    pub powers: String,
    pub hearthstone_slots: Vec<Option<OwnedHearthstoneMemo>>,
    pub merit_dots: u8,
    pub magic_material: Option<MagicMaterial>,
    pub attunement_cost: Option<u8>,
}

impl<'source> WonderNoAttunementMemo {
    pub fn as_ref(&'source self) -> WonderNoAttunement<'source> {
        WonderNoAttunement {
            name: self.name.as_str(),
            book_reference: self.book_reference,
            lore: self.lore.as_deref(),
            powers: self.powers.as_ref(),
            hearthstone_slots: self
                .hearthstone_slots
                .iter()
                .map(|option| option.as_ref().map(|hearthstone| hearthstone.as_ref()))
                .collect(),
            merit_dots: self.merit_dots,
            magic_material: self.magic_material,
            attunement_cost: self.attunement_cost,
        }
    }
}
