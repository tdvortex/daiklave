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

impl<'source> From<&'source WonderNoAttunementMemo> for WonderNoAttunement<'source> {
    fn from(memo: &'source WonderNoAttunementMemo) -> Self {
        Self {
            book_reference: memo.book_reference,
            lore: memo.lore.as_deref(),
            powers: memo.powers.as_ref(),
            hearthstone_slots: memo
                .hearthstone_slots
                .iter()
                .map(|option| option.as_ref().map(|hearthstone| hearthstone.into()))
                .collect(),
            merit_dots: memo.merit_dots,
            magic_material: memo.magic_material,
            attunement_cost: memo.attunement_cost,
        }
    }
}

impl<'source> WonderNoAttunement<'source> {
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
