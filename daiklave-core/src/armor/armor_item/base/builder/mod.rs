mod artifact;
mod mundane;
mod with_weight_class;
pub use artifact::{BaseArtifactArmorBuilder, BaseArtifactArmorBuilderWithWeightClass};
pub use mundane::{MundaneArmorBuilder, MundaneArmorBuilderWithWeightClass};
pub use with_weight_class::BaseArmorItemBuilderWithWeightClass;

use std::collections::HashSet;

use crate::{
    armor::armor_item::{ArmorTag, ArmorWeightClass},
    book_reference::BookReference,
};

/// Constructs a base armor item, like "Chain Shirt" or "Articulated Plate".
/// Weight class is required; book reference and tags are optional.
pub struct BaseArmorBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
}

impl BaseArmorBuilder {
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

    /// Sets the base armor item to be an artifact.
    pub fn artifact(self) -> BaseArtifactArmorBuilder {
        BaseArtifactArmorBuilder {
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
        }
    }

    /// Sets the base armor item to be mundane.
    pub fn mundane(self) -> MundaneArmorBuilder {
        MundaneArmorBuilder {
            name: self.name.into(),
            book_reference: self.book_reference,
            tags: self.tags,
        }
    }

    /// Sets the weight class of the armor item.
    pub fn weight_class(
        self,
        weight_class: ArmorWeightClass,
    ) -> BaseArmorItemBuilderWithWeightClass {
        BaseArmorItemBuilderWithWeightClass {
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class,
        }
    }
}
