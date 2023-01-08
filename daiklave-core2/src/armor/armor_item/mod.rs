/// Properties of artifact armor
pub mod artifact;

mod armor_type;
mod base;
mod equipped;
mod id;
mod memo;
mod tag;
/// Properties of mundane armor
pub mod mundane;
mod weight_class;

use std::collections::HashSet;

pub use id::ArmorId;
pub use base::{BaseArmorId};
pub(crate) use armor_type::ArmorType;
pub(crate) use equipped::{EquippedArmor, EquippedArmorMemo, EquippedArmorNoAttunement, EquippedArmorNoAttunementMemo};
pub use tag::ArmorTag;
pub use weight_class::ArmorWeightClass;

use crate::book_reference::BookReference;

use self::{base::builder::BaseArmorItemBuilder, artifact::builder::ArtifactArmorItemBuilder};

/// A single piece of armor owned by a character
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmorItem<'source>(pub(crate) ArmorType<'source>, pub(crate) bool);

impl<'source> ArmorItem<'source> {
    /// Starts constructing a base armor item.
    pub fn base(name: &str) -> BaseArmorItemBuilder {
        BaseArmorItemBuilder {
            name: name.to_owned(),
            book_reference: None,
            tags: HashSet::new(),
        }
    }

    /// Starts construct an artifact armor item.
    pub fn artifact(name: &str) -> ArtifactArmorItemBuilder {
        ArtifactArmorItemBuilder { 
            name: name.to_owned(), 
            book_reference: None, 
            lore: None, 
            powers: None,
        }
    }

    /// The Id of the armor item
    pub fn id(&self) -> ArmorId {
        self.0.id()
    }

    /// The name of the armor item. For artifacts, this will return the name of
    /// the unique armor item (like "Brilliant Sentinel") not the name of the
    /// base armor item (like "Articulated Plate").
    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    /// The book reference for the armor item, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// The weight class of the armor item.
    pub fn weight_class(&self) -> ArmorWeightClass {
        self.0.weight_class()
    }

    /// The bonus to soak granted from wearing the armor item.
    pub fn soak_bonus(&self) -> u8 {
        self.0.soak_bonus()
    }

    /// The mobility penalty incurred from wearing the armor item.
    pub fn mobility_penalty(&self) -> i8 {
        self.0.mobility_penalty()
    }

    /// The hardness from wearing the armor. (Zero for mundane armor.)
    pub fn hardness(&self) -> u8 {
        self.0.hardness()
    }

    /// The number of motes it takes to attune to this armor item.
    /// None for mundane armor.
    pub fn attunement_cost(&self) -> Option<u8> {
        self.0.attunement_cost()
    }

    /// An iterator over the armor item's tags. 
    pub fn tags(&self) -> impl Iterator<Item = ArmorTag> + '_ {
        self.0.tags()
    }

    /// The total number of hearthstone slots in the armor item. Zero for
    /// mundane armor.
    pub fn hearthstone_slots(&self) -> u8 {
        self.0.hearthstone_slots()
    }

    /// Returns true if the armor item is currently equipped.
    pub fn is_equipped(&self) -> bool {
        self.1
    }
}