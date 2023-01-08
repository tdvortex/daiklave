mod with_base_armor;
mod with_hearthstone_slots;
mod with_magic_material;
mod with_merit_dots;

use crate::{book_reference::BookReference, armor::armor_item::BaseArmorId};

use self::with_base_armor::ArtifactArmorItemBuilderWithBaseArmor;

use super::BaseArtifactArmor;

/// A builder to construct a new artifact armor. Enforces that required fields
/// are specified in order: name, base artifact, magic material, merit dots,
/// and finally hearthstone slots. Optional fields (lore, powers, and book
/// reference) may be specified at any time prior to the final build().
pub struct ArtifactArmorItemBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
}

impl ArtifactArmorItemBuilder {
    /// The book and page number where the named artifact armor is listed.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Flavor text about the artifact's history, prior wearers, etc.
    pub fn lore(mut self, lore: &str) -> Self {
        self.lore = Some(lore.to_owned());
        self
    }

    /// Persistent powers that the bearer gets for free.
    pub fn powers(mut self, powers: &str) -> Self {
        self.powers = Some(powers.to_owned());
        self
    }

    /// Specifies the base armor item (like "Silken Armor") that the armor item
    /// is an example of.
    pub fn base_artifact(self, base_artifact_id: BaseArmorId, base_artifact: BaseArtifactArmor) -> ArtifactArmorItemBuilderWithBaseArmor {
        ArtifactArmorItemBuilderWithBaseArmor {
            name: self.name,
            book_reference: self.book_reference,
            lore: self.lore,
            powers: self.powers,
            base_armor: base_artifact,
            base_armor_id: base_artifact_id,
        }
    }
}