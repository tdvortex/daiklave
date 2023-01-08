use crate::{armor::armor_item::{BaseArmorId, base::BaseArmor}, book_reference::BookReference, hearthstone::OwnedHearthstoneMemo, artifact::MagicMaterial};

/// An owned copy of a named piece of artifact armor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactArmorMemo {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) base_armor_id: BaseArmorId,
    pub(crate) base_armor: BaseArmor,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
    pub(crate) hearthstone_slots: Vec<Option<OwnedHearthstoneMemo>>,
}