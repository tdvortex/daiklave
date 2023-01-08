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