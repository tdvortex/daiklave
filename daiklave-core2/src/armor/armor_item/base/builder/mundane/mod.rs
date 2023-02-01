mod with_weight_class;
pub use with_weight_class::MundaneArmorBuilderWithWeightClass;

use std::collections::HashSet;

use crate::{book_reference::BookReference, armor::armor_item::{ArmorTag, mundane::MundaneArmorName, ArmorWeightClass}};

/// A builder to construct a mundane, non-artifact piece of armor.
pub struct MundaneArmorBuilder {
    pub(crate) name: MundaneArmorName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
}

impl MundaneArmorBuilder {
    /// Creates a new builder with the given name.
    pub fn name(name: impl Into<MundaneArmorName>) -> Self {
        Self {
            name: name.into(),
            book_reference: Default::default(),
            tags: Default::default(),
        }
    }

    /// The book reference for the armor item.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds a tag to the armor item.
    pub fn tag(mut self, tag: ArmorTag) -> Self {
        self.tags.insert(tag);
        self
    }

    /// Sets the weight class of the armor item.
    pub fn weight_class(
        self,
        weight_class: ArmorWeightClass,
    ) -> MundaneArmorBuilderWithWeightClass {
        MundaneArmorBuilderWithWeightClass {
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class,
        }
    }
}