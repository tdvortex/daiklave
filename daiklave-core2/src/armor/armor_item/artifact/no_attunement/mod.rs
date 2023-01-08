use crate::{book_reference::BookReference, armor::armor_item::{BaseArmorId, base::BaseArmor}, artifact::MagicMaterial, hearthstone::OwnedHearthstone};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactArmorNoAttunement<'source> {
    pub(crate) name: &'source str,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<&'source str>,
    pub(crate) powers: Option<&'source str>,
    pub(crate) base_armor_id: BaseArmorId,
    pub(crate) base_armor: &'source BaseArmor,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
    pub(crate) hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
}

impl<'source> ArtifactArmorNoAttunement<'source> {
    pub fn name(&self) -> &'source str {
        self.name
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
}