mod memo;
pub(crate) use memo::WonderNoAttunementMemo;

use crate::{
    artifact::{ArtifactName, MagicMaterial},
    book_reference::BookReference,
    hearthstones::{hearthstone::Hearthstone, HearthstonePosition, SlottedHearthstone},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WonderNoAttunement<'source> {
    book_reference: Option<BookReference>,
    lore: Option<&'source str>,
    powers: &'source str,
    pub hearthstone_slots: Vec<Option<SlottedHearthstone<'source>>>,
    pub merit_dots: u8,
    magic_material: Option<MagicMaterial>,
    pub attunement_cost: Option<u8>,
}

impl<'source> WonderNoAttunement<'source> {
    pub fn as_memo(&self) -> WonderNoAttunementMemo {
        WonderNoAttunementMemo {
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
            attunement_cost: self.attunement_cost,
        }
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

    pub fn slotted_hearthstones(
        &self,
        name: &'source str,
    ) -> impl Iterator<Item = Hearthstone<'source>> + '_ {
        self.hearthstone_slots
            .iter()
            .filter_map(move |maybe_slotted| {
                (*maybe_slotted).map(|slotted| {
                    Hearthstone(HearthstonePosition::Slotted(
                        ArtifactName::Wonder(name),
                        slotted,
                    ))
                })
            })
    }

    pub fn open_slots(&self) -> u8 {
        self.hearthstone_slots
            .iter()
            .filter(|maybe_slotted| maybe_slotted.is_none())
            .count()
            .min(u8::MAX as usize) as u8
    }
}
