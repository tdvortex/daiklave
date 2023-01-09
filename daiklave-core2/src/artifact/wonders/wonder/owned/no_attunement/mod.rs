mod memo;
pub(crate) use memo::WonderNoAttunementMemo;

use crate::{
    artifact::MagicMaterial, book_reference::BookReference, hearthstone::OwnedHearthstone,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WonderNoAttunement<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    lore: Option<&'source str>,
    powers: &'source str,
    hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
    merit_dots: u8,
    magic_material: Option<MagicMaterial>,
}

impl<'source> WonderNoAttunement<'source> {
    pub fn as_memo(&self) -> WonderNoAttunementMemo {
        WonderNoAttunementMemo {
            name: self.name.to_owned(),
            book_reference: self.book_reference,
            lore: self.lore.as_ref().map(|s| s.to_string()),
            powers: self.powers.to_string(),
            hearthstone_slots: self
                .hearthstone_slots
                .iter()
                .map(|option| option.map(|heartstone| heartstone.as_memo()))
                .collect(),
            merit_dots: self.merit_dots,
            magic_material: self.magic_material,
        }
    }

    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn lore(&self) -> Option<&'source str> {
        self.lore
    }

    pub fn powers(&self) -> &'source str {
        self.powers
    }

    pub fn hearthstone_slots(&self) -> u8 {
        self.hearthstone_slots.len().min(u8::MAX as usize) as u8
    }

    pub fn material(&self) -> Option<MagicMaterial> {
        self.magic_material
    }
}
